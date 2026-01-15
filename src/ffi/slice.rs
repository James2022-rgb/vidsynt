//! Slice segment header accessor functions

use super::context::VidsyntHevcContext;
use super::types::*;
use crate::h265::nalu::{Nalu, NaluValue};

/// Get the slice segment header from a parsed NAL unit
///
/// # Parameters
/// - `ctx`: Context (needed for allocating converted structures)
/// - `nalu`: Pointer to a parsed NAL unit
/// - `out_slice_header`: Output pointer to receive the slice segment header
///
/// # Returns
/// - `VIDSYNT_SUCCESS` on success
/// - `VIDSYNT_ERROR_INVALID_PARAMETER` if any pointer is null
/// - `VIDSYNT_ERROR_DATA_NOT_AVAILABLE` if the NAL unit is not a slice
///
/// # Safety
/// - `ctx` must be a valid context pointer
/// - `nalu` must be a valid pointer from `vidsynt_hevc_parse_nalu_from_bytes`
/// - `out_slice_header` must be a valid pointer to write to
#[no_mangle]
pub unsafe extern "C" fn vidsynt_hevc_nalu_get_slice_header(
    ctx: *mut VidsyntHevcContext,
    nalu: *const VidsyntHevcNalu,
    out_slice_header: *mut *const VidsyntHevcSliceSegmentHeader,
) -> VidsyntResult {
    if ctx.is_null() || nalu.is_null() || out_slice_header.is_null() {
        return VidsyntResult::InvalidParameter;
    }

    let ctx = &mut *ctx;
    let nalu = &*(nalu as *const Nalu);

    // Extract slice segment header from NAL value
    let rust_slice = match &nalu.value {
        NaluValue::CodedSliceSegment(slice_layer) => &slice_layer.header,
        _ => return VidsyntResult::DataNotAvailable,
    };

    // Create C-compatible slice segment header
    let c_slice_header = VidsyntHevcSliceSegmentHeader {
        nal_unit_type: rust_slice.nal_unit_type as u8,
        first_slice_segment_in_pic_flag: if rust_slice.first_slice_segment_in_pic_flag { 1 } else { 0 },
        no_output_of_prior_pics_flag: match rust_slice.no_output_of_prior_pics_flag {
            Some(true) => 1,
            Some(false) => 0,
            None => 0xFF,
        },
        slice_pic_parameter_set_id: rust_slice.slice_pic_parameter_set_id,
        dependent_slice_segment_flag: match rust_slice.dependent_slice_segment_flag {
            Some(true) => 1,
            Some(false) => 0,
            None => 0xFF,
        },
        slice_segment_address: rust_slice.slice_segment_address.unwrap_or(0),
        slice_pic_order_cnt_lsb: rust_slice.slice_pic_order_cnt_lsb.unwrap_or(0xFFFF),
        short_term_ref_pic_set_idx: rust_slice.short_term_ref_pic_set_idx.unwrap_or(0xFF),
        curr_rps_idx: rust_slice.curr_rps_idx,
    };

    // Store in context and return pointer
    ctx.ffi_slice_headers.push(Box::new(c_slice_header));
    *out_slice_header = ctx.ffi_slice_headers.last().unwrap().as_ref() as *const _;

    VidsyntResult::Success
}
