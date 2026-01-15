//! Format conversion utilities (e.g., length-prefixed to Annex B)

use super::context::VidsyntHevcContext;
use super::types::*;

/// Convert length-prefixed NAL units to Annex B format
///
/// Length-prefixed format (used in MP4 containers):
/// - Each NAL unit is preceded by N bytes (1-4) indicating its length
///
/// Annex B format (used for streaming/decoders):
/// - NAL units are separated by start codes (0x000001 or 0x00000001)
///
/// # Parameters
/// - `ctx`: Context that will own the allocated output buffer
/// - `data`: Pointer to length-prefixed NAL unit data
/// - `len`: Length of input data in bytes
/// - `length_size_minus_one`: Length field size minus 1 (0=1 byte, 1=2 bytes, 2=3 bytes, 3=4 bytes)
/// - `out_data`: Output pointer to receive converted Annex B data
/// - `out_len`: Output pointer to receive length of converted data
///
/// # Returns
/// - `VIDSYNT_SUCCESS` on success
/// - `VIDSYNT_ERROR_INVALID_PARAMETER` if any pointer is null or length_size_minus_one > 3
/// - `VIDSYNT_ERROR_PARSE_FAILED` if data is malformed
///
/// # Safety
/// - `ctx` must be a valid context pointer
/// - `data` must point to `len` valid bytes
/// - `out_data` and `out_len` must be valid pointers to write to
#[no_mangle]
pub unsafe extern "C" fn vidsynt_hevc_convert_length_prefixed_to_annex_b(
    ctx: *mut VidsyntHevcContext,
    data: *const u8,
    len: usize,
    length_size_minus_one: u8,
    out_data: *mut *const u8,
    out_len: *mut usize,
) -> VidsyntResult {
    if ctx.is_null() || data.is_null() || out_data.is_null() || out_len.is_null() {
        return VidsyntResult::InvalidParameter;
    }

    if length_size_minus_one > 3 {
        return VidsyntResult::InvalidParameter;
    }

    let ctx = &mut *ctx;
    let input = std::slice::from_raw_parts(data, len);
    let length_size = (length_size_minus_one + 1) as usize;

    // Allocate output buffer (worst case: 4-byte start codes for all NAL units)
    // Estimate: each NAL unit gets a 4-byte start code
    let mut output = Vec::with_capacity(len + 1024);

    let mut pos = 0;
    while pos < len {
        // Check if we have enough bytes for length field
        if pos + length_size > len {
            return VidsyntResult::ParseFailed;
        }

        // Read NAL unit length
        let mut nal_length: usize = 0;
        for i in 0..length_size {
            nal_length = (nal_length << 8) | (input[pos + i] as usize);
        }
        pos += length_size;

        // Check if we have enough bytes for NAL unit
        if pos + nal_length > len {
            return VidsyntResult::ParseFailed;
        }

        // Write start code (use 4-byte for first NAL, 3-byte for others)
        if output.is_empty() {
            // First NAL unit: use 4-byte start code
            output.extend_from_slice(&[0x00, 0x00, 0x00, 0x01]);
        } else {
            // Subsequent NAL units: use 3-byte start code
            output.extend_from_slice(&[0x00, 0x00, 0x01]);
        }

        // Copy NAL unit data
        output.extend_from_slice(&input[pos..pos + nal_length]);
        pos += nal_length;
    }

    // Store output in context
    let output_len = output.len();
    ctx.conversion_buffers.push(output);
    let output_buffer = ctx.conversion_buffers.last().unwrap();

    *out_data = output_buffer.as_ptr();
    *out_len = output_len;

    VidsyntResult::Success
}

/// Convert Annex B format NAL units to length-prefixed format
///
/// Annex B format (used for streaming/decoders):
/// - NAL units are separated by start codes (0x000001 or 0x00000001)
///
/// Length-prefixed format (used in MP4 containers):
/// - Each NAL unit is preceded by N bytes (1-4) indicating its length
///
/// # Parameters
/// - `ctx`: Context that will own the allocated output buffer
/// - `data`: Pointer to Annex B NAL unit data
/// - `len`: Length of input data in bytes
/// - `length_size_minus_one`: Desired length field size minus 1 (0=1 byte, 1=2 bytes, 2=3 bytes, 3=4 bytes)
/// - `out_data`: Output pointer to receive converted length-prefixed data
/// - `out_len`: Output pointer to receive length of converted data
///
/// # Returns
/// - `VIDSYNT_SUCCESS` on success
/// - `VIDSYNT_ERROR_INVALID_PARAMETER` if any pointer is null or length_size_minus_one > 3
/// - `VIDSYNT_ERROR_PARSE_FAILED` if data is malformed or NAL unit too large for length field
///
/// # Safety
/// - `ctx` must be a valid context pointer
/// - `data` must point to `len` valid bytes
/// - `out_data` and `out_len` must be valid pointers to write to
#[no_mangle]
pub unsafe extern "C" fn vidsynt_hevc_convert_annex_b_to_length_prefixed(
    ctx: *mut VidsyntHevcContext,
    data: *const u8,
    len: usize,
    length_size_minus_one: u8,
    out_data: *mut *const u8,
    out_len: *mut usize,
) -> VidsyntResult {
    if ctx.is_null() || data.is_null() || out_data.is_null() || out_len.is_null() {
        return VidsyntResult::InvalidParameter;
    }

    if length_size_minus_one > 3 {
        return VidsyntResult::InvalidParameter;
    }

    let ctx = &mut *ctx;
    let input = std::slice::from_raw_parts(data, len);
    let length_size = (length_size_minus_one + 1) as usize;
    let max_nal_length = match length_size {
        1 => 0xFF,
        2 => 0xFFFF,
        3 => 0xFFFFFF,
        4 => 0xFFFFFFFF,
        _ => return VidsyntResult::InvalidParameter,
    };

    let mut output = Vec::with_capacity(len);
    let mut pos = 0;

    while pos < len {
        // Find next start code
        let start_code_pos = find_start_code(input, pos);
        if start_code_pos.is_none() {
            // No more start codes, rest of data is part of last NAL unit if we've found at least one
            if output.is_empty() {
                // No start codes found at all
                return VidsyntResult::ParseFailed;
            }
            break;
        }

        let (start_pos, start_code_len) = start_code_pos.unwrap();

        // Skip the first start code (beginning of first NAL unit)
        if output.is_empty() {
            pos = start_pos + start_code_len;
            continue;
        }

        // Calculate NAL unit length (from previous start code to this one)
        let nal_length = start_pos - pos;

        // Check if NAL unit fits in length field
        if nal_length > max_nal_length {
            return VidsyntResult::ParseFailed;
        }

        // Write length prefix
        for i in (0..length_size).rev() {
            output.push(((nal_length >> (i * 8)) & 0xFF) as u8);
        }

        // Write NAL unit data
        output.extend_from_slice(&input[pos..start_pos]);

        pos = start_pos + start_code_len;
    }

    // Handle last NAL unit (from last start code to end of data)
    if pos < len {
        let nal_length = len - pos;

        if nal_length > max_nal_length {
            return VidsyntResult::ParseFailed;
        }

        // Write length prefix
        for i in (0..length_size).rev() {
            output.push(((nal_length >> (i * 8)) & 0xFF) as u8);
        }

        // Write NAL unit data
        output.extend_from_slice(&input[pos..]);
    }

    // Store output in context
    let output_len = output.len();
    ctx.conversion_buffers.push(output);
    let output_buffer = ctx.conversion_buffers.last().unwrap();

    *out_data = output_buffer.as_ptr();
    *out_len = output_len;

    VidsyntResult::Success
}

/// Helper function to find the next start code in Annex B data
///
/// Returns (position, start_code_length) or None if not found
fn find_start_code(data: &[u8], start_pos: usize) -> Option<(usize, usize)> {
    let mut i = start_pos;
    while i + 2 < data.len() {
        // Check for 3-byte start code: 0x000001
        if data[i] == 0x00 && data[i + 1] == 0x00 && data[i + 2] == 0x01 {
            return Some((i, 3));
        }

        // Check for 4-byte start code: 0x00000001
        if i + 3 < data.len()
            && data[i] == 0x00
            && data[i + 1] == 0x00
            && data[i + 2] == 0x00
            && data[i + 3] == 0x01
        {
            return Some((i, 4));
        }

        i += 1;
    }
    None
}
