//! Picture Order Count (POC) computation functions

use super::context::VidsyntHevcContext;
use super::types::*;
use crate::h265::poc::PocComputer as RustPocComputer;
use crate::h265::nalu::NaluType;

/// Create a new POC (Picture Order Count) computer
///
/// # Parameters
/// - `ctx`: Context that will own the POC computer
///
/// # Returns
/// Pointer to the new POC computer, or null on allocation failure
///
/// # Safety
/// - `ctx` must be a valid context pointer
#[no_mangle]
pub unsafe extern "C" fn vidsynt_hevc_poc_computer_new(
    ctx: *mut VidsyntHevcContext,
) -> *mut VidsyntHevcPocComputer {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }

    let ctx = &mut *ctx;

    // Create new POC computer
    let poc = Box::new(RustPocComputer::default());
    ctx.poc_computers.push(poc);

    // Return pointer to the last POC computer (cast to opaque type)
    ctx.poc_computers.last_mut().unwrap().as_mut() as *mut RustPocComputer as *mut VidsyntHevcPocComputer
}

/// Compute the Picture Order Count (POC) value for a picture
///
/// # Parameters
/// - `poc_computer`: Pointer to a POC computer
/// - `sps`: Pointer to the SPS referenced by the picture
/// - `pps`: Pointer to the PPS referenced by the picture
/// - `slice_header`: Pointer to the slice segment header
/// - `out_poc`: Output pointer to receive the computed POC value
///
/// # Returns
/// - `VIDSYNT_SUCCESS` on success
/// - `VIDSYNT_ERROR_INVALID_PARAMETER` if any pointer is null
///
/// # Safety
/// - `poc_computer` must be a valid pointer from `vidsynt_hevc_poc_computer_new`
/// - `sps` must be a valid pointer from `vidsynt_hevc_nalu_get_sps`
/// - `pps` must be a valid pointer from `vidsynt_hevc_nalu_get_pps`
/// - `slice_header` must be a valid pointer from `vidsynt_hevc_nalu_get_slice_header`
/// - `out_poc` must be a valid pointer to write to
#[no_mangle]
pub unsafe extern "C" fn vidsynt_hevc_poc_compute(
    poc_computer: *mut VidsyntHevcPocComputer,
    sps: *const VidsyntHevcSequenceParameterSet,
    pps: *const VidsyntHevcPictureParameterSet,
    slice_header: *const VidsyntHevcSliceSegmentHeader,
    out_poc: *mut i32,
) -> VidsyntResult {
    if poc_computer.is_null() || sps.is_null() || pps.is_null() || slice_header.is_null() || out_poc.is_null() {
        return VidsyntResult::InvalidParameter;
    }

    let poc_computer = &mut *(poc_computer as *mut RustPocComputer);
    let sps = &*sps;
    let pps = &*pps;
    let slice_header = &*slice_header;

    // Convert NAL unit type (slice_header.nal_unit_type is VidsyntHevcNaluType)
    let nal_type_u8 = hevc_nalu_type_to_u8(slice_header.nal_unit_type);
    let nal_type: NaluType = match nal_type_u8.try_into() {
        Ok(t) => t,
        Err(_) => return VidsyntResult::InvalidNaluType,
    };

    // Compute POC using the extended function that doesn't require full Rust types
    let poc = poc_computer.compute_poc_ex(
        sps.log2_max_pic_order_cnt_lsb_minus4,
        pps.nuh_temporal_id_plus1,
        nal_type,
        slice_header.slice_pic_order_cnt_lsb as i32,
    );

    *out_poc = poc;
    VidsyntResult::Success
}

/// Reset the POC computer for IDR or random access
///
/// Call this when starting a new coded video sequence (CVS), such as after
/// seeking to an IDR or CRA picture.
///
/// # Parameters
/// - `poc_computer`: Pointer to a POC computer
///
/// # Returns
/// - `VIDSYNT_SUCCESS` on success
/// - `VIDSYNT_ERROR_INVALID_PARAMETER` if the pointer is null
///
/// # Safety
/// - `poc_computer` must be a valid pointer from `vidsynt_hevc_poc_computer_new`
#[no_mangle]
pub unsafe extern "C" fn vidsynt_hevc_poc_reset(
    poc_computer: *mut VidsyntHevcPocComputer,
) -> VidsyntResult {
    if poc_computer.is_null() {
        return VidsyntResult::InvalidParameter;
    }

    let poc_computer = &mut *(poc_computer as *mut RustPocComputer);
    poc_computer.reset_for_idr_or_random_access();

    VidsyntResult::Success
}
