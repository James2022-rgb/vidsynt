//! NAL unit parsing functions

use super::context::VidsyntHevcContext;
use super::types::*;
use crate::h265::nalu::{Nalu, NaluType, NaluValue, NaluValueContext};
use std::slice;

/// Parse a single NAL unit from raw bytes
///
/// This function parses parameter sets (VPS, SPS, PPS) without requiring context.
/// For slice segments, you need to parse the parameter sets first.
///
/// # Parameters
/// - `ctx`: Context that will own the parsed NAL unit
/// - `data`: Pointer to NAL unit data (including NAL header)
/// - `len`: Length of NAL unit in bytes
/// - `out_nalu`: Output pointer to receive the parsed NAL unit
///
/// # Returns
/// - `VIDSYNT_SUCCESS` on success
/// - `VIDSYNT_ERROR_INVALID_PARAMETER` if any pointer is null or len is 0
/// - `VIDSYNT_ERROR_PARSE_FAILED` if parsing fails
///
/// # Safety
/// - `ctx` must be a valid context pointer
/// - `data` must point to `len` valid bytes
/// - `out_nalu` must be a valid pointer to write to
#[no_mangle]
pub unsafe extern "C" fn vidsynt_hevc_parse_nalu_from_bytes(
    ctx: *mut VidsyntHevcContext,
    data: *const u8,
    len: usize,
    out_nalu: *mut *const VidsyntHevcNalu,
) -> VidsyntResult {
    // Validate parameters
    if ctx.is_null() || data.is_null() || out_nalu.is_null() || len == 0 {
        return VidsyntResult::InvalidParameter;
    }

    let ctx = &mut *ctx;
    let data_slice = slice::from_raw_parts(data, len);

    // Build NAL parsing context from active SPS/PPS (for slice segments)
    let nalu_value_context = NaluValueContext {
        slice_segment_context: ctx.build_slice_context(),
    };

    let nalu = match Nalu::from_bytes(data_slice, nalu_value_context) {
        Ok(nalu) => nalu,
        Err(_) => return VidsyntResult::ParseFailed,
    };

    // Cache parameter sets so they can later be referenced by id
    // (e.g. via `vidsynt_hevc_context_set_active_sps`/`..._set_active_pps`).
    match &nalu.value {
        NaluValue::SpsNut(sps) => {
            ctx.sps_cache
                .insert(sps.sps_seq_parameter_set_id, Box::new(sps.clone()));
        }
        NaluValue::PpsNut(pps) => {
            ctx.pps_cache
                .insert(pps.pps_pic_parameter_set_id, Box::new(pps.clone()));
        }
        _ => {}
    }

    // Store the parsed NAL unit in context
    ctx.nalus.push(Box::new(nalu));

    // Return pointer to the NAL unit (cast to opaque VidsyntHevcNalu)
    let nalu_ref = ctx.nalus.last().unwrap().as_ref();
    *out_nalu = nalu_ref as *const Nalu as *const VidsyntHevcNalu;

    VidsyntResult::Success
}

/// Get the NAL unit type from a parsed NAL unit
///
/// # Parameters
/// - `nalu`: Pointer to a parsed NAL unit
///
/// # Returns
/// The NAL unit type
///
/// # Safety
/// - `nalu` must be a valid pointer from `vidsynt_hevc_parse_nalu_from_bytes`
#[no_mangle]
pub unsafe extern "C" fn vidsynt_hevc_nalu_get_type(
    nalu: *const VidsyntHevcNalu,
) -> VidsyntHevcNaluType {
    if nalu.is_null() {
        // Return an invalid type as sentinel
        return u8_to_hevc_nalu_type(0xFF);
    }

    let nalu = &*(nalu as *const Nalu);
    u8_to_hevc_nalu_type(nalu.header.nal_unit_type as u8)
}

/// Get the NAL unit header
///
/// # Parameters
/// - `nalu`: Pointer to a parsed NAL unit
/// - `out_header`: Output pointer to receive header
///
/// # Returns
/// - `VIDSYNT_SUCCESS` on success
/// - `VIDSYNT_ERROR_INVALID_PARAMETER` if any pointer is null
///
/// # Safety
/// - `nalu` must be a valid pointer from `vidsynt_hevc_parse_nalu_from_bytes`
/// - `out_header` must be a valid pointer to write to
#[no_mangle]
pub unsafe extern "C" fn vidsynt_hevc_nalu_get_header(
    nalu: *const VidsyntHevcNalu,
    out_header: *mut VidsyntHevcNaluHeader,
) -> VidsyntResult {
    if nalu.is_null() || out_header.is_null() {
        return VidsyntResult::InvalidParameter;
    }

    let nalu = &*(nalu as *const Nalu);

    *out_header = VidsyntHevcNaluHeader {
        nal_unit_type: u8_to_hevc_nalu_type(nalu.header.nal_unit_type as u8),
        nuh_layer_id: nalu.header.nuh_layer_id,
        nuh_temporal_id_plus1: nalu.header.nuh_temporal_id_plus1,
        _reserved: 0,
    };

    VidsyntResult::Success
}

/// Check if a NAL unit type is an IDR picture
///
/// # Parameters
/// - `nal_type`: NAL unit type to check
///
/// # Returns
/// 1 if IDR, 0 otherwise
#[no_mangle]
pub extern "C" fn vidsynt_hevc_nalu_type_is_idr(nal_type: VidsyntHevcNaluType) -> u8 {
    let nal_type_u8 = hevc_nalu_type_to_u8(nal_type);
    let rust_type: Result<NaluType, _> = nal_type_u8.try_into();
    match rust_type {
        Ok(t) => if t.is_idr() { 1 } else { 0 },
        Err(_) => 0,
    }
}

/// Check if a NAL unit type is an IRAP (Intra Random Access Point) picture
///
/// # Parameters
/// - `nal_type`: NAL unit type to check
///
/// # Returns
/// 1 if IRAP, 0 otherwise
#[no_mangle]
pub extern "C" fn vidsynt_hevc_nalu_type_is_irap(nal_type: VidsyntHevcNaluType) -> u8 {
    let nal_type_u8 = hevc_nalu_type_to_u8(nal_type);
    let rust_type: Result<NaluType, _> = nal_type_u8.try_into();
    match rust_type {
        Ok(t) => if t.is_irap() { 1 } else { 0 },
        Err(_) => 0,
    }
}

/// Check if a NAL unit type is a BLA (Broken Link Access) picture
///
/// # Parameters
/// - `nal_type`: NAL unit type to check
///
/// # Returns
/// 1 if BLA, 0 otherwise
#[no_mangle]
pub extern "C" fn vidsynt_hevc_nalu_type_is_bla(nal_type: VidsyntHevcNaluType) -> u8 {
    let nal_type_u8 = hevc_nalu_type_to_u8(nal_type);
    let rust_type: Result<NaluType, _> = nal_type_u8.try_into();
    match rust_type {
        Ok(t) => if t.is_bla() { 1 } else { 0 },
        Err(_) => 0,
    }
}

/// Check if a NAL unit type is a reference picture
///
/// # Parameters
/// - `nal_type`: NAL unit type to check
///
/// # Returns
/// 1 if reference picture, 0 otherwise
#[no_mangle]
pub extern "C" fn vidsynt_hevc_nalu_type_is_reference(nal_type: VidsyntHevcNaluType) -> u8 {
    let nal_type_u8 = hevc_nalu_type_to_u8(nal_type);
    let rust_type: Result<NaluType, _> = nal_type_u8.try_into();
    match rust_type {
        Ok(t) => if t.is_reference() { 1 } else { 0 },
        Err(_) => 0,
    }
}

/// Check if a NAL unit type is a RADL (Random Access Decodable Leading) picture
///
/// # Parameters
/// - `nal_type`: NAL unit type to check
///
/// # Returns
/// 1 if RADL, 0 otherwise
#[no_mangle]
pub extern "C" fn vidsynt_hevc_nalu_type_is_radl(nal_type: VidsyntHevcNaluType) -> u8 {
    let nal_type_u8 = hevc_nalu_type_to_u8(nal_type);
    let rust_type: Result<NaluType, _> = nal_type_u8.try_into();
    match rust_type {
        Ok(t) => if t.is_radl() { 1 } else { 0 },
        Err(_) => 0,
    }
}

/// Check if a NAL unit type is a RASL (Random Access Skipped Leading) picture
///
/// # Parameters
/// - `nal_type`: NAL unit type to check
///
/// # Returns
/// 1 if RASL, 0 otherwise
#[no_mangle]
pub extern "C" fn vidsynt_hevc_nalu_type_is_rasl(nal_type: VidsyntHevcNaluType) -> u8 {
    let nal_type_u8 = hevc_nalu_type_to_u8(nal_type);
    let rust_type: Result<NaluType, _> = nal_type_u8.try_into();
    match rust_type {
        Ok(t) => if t.is_rasl() { 1 } else { 0 },
        Err(_) => 0,
    }
}

/// Check if a NAL unit type is a coded slice segment
///
/// # Parameters
/// - `nal_type`: NAL unit type to check
///
/// # Returns
/// 1 if coded slice segment, 0 otherwise
#[no_mangle]
pub extern "C" fn vidsynt_hevc_nalu_type_is_coded_slice_segment(nal_type: VidsyntHevcNaluType) -> u8 {
    let nal_type_u8 = hevc_nalu_type_to_u8(nal_type);
    let rust_type: Result<NaluType, _> = nal_type_u8.try_into();
    match rust_type {
        Ok(t) => if t.is_coded_slice_segment() { 1 } else { 0 },
        Err(_) => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ffi::context::vidsynt_hevc_context_new;

    #[test]
    fn test_nalu_type_queries() {
        use VidsyntHevcNaluType::*;

        let idr_type = VIDSYNT_HEVC_NALU_IDR_W_RADL;
        assert_eq!(vidsynt_hevc_nalu_type_is_idr(idr_type), 1);
        assert_eq!(vidsynt_hevc_nalu_type_is_irap(idr_type), 1);
        assert_eq!(vidsynt_hevc_nalu_type_is_reference(idr_type), 1);

        let trail_n_type = VIDSYNT_HEVC_NALU_TRAIL_N;
        assert_eq!(vidsynt_hevc_nalu_type_is_idr(trail_n_type), 0);
        assert_eq!(vidsynt_hevc_nalu_type_is_reference(trail_n_type), 0);

        let sps_type = VIDSYNT_HEVC_NALU_SPS_NUT;
        assert_eq!(vidsynt_hevc_nalu_type_is_coded_slice_segment(sps_type), 0);
    }
}
