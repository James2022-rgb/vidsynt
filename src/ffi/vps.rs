//! Video Parameter Set accessor functions

use super::context::VidsyntHevcContext;
use super::types::*;
use crate::h265::nalu::{Nalu, NaluValue};
use crate::h265::ptl::ProfileTierLevel as RustProfileTierLevel;
use crate::h265::ptl::SubLayerOrderingInfo as RustSubLayerOrderingInfo;
use crate::h265::vps::TimingInfo as RustTimingInfo;
use std::ptr;

/// Get the VPS from a parsed NAL unit
///
/// # Parameters
/// - `ctx`: Context (needed for allocating converted structures)
/// - `nalu`: Pointer to a parsed NAL unit
/// - `out_vps`: Output pointer to receive the VPS
///
/// # Returns
/// - `VIDSYNT_SUCCESS` on success
/// - `VIDSYNT_ERROR_INVALID_PARAMETER` if any pointer is null
/// - `VIDSYNT_ERROR_DATA_NOT_AVAILABLE` if the NAL unit is not a VPS
///
/// # Safety
/// - `ctx` must be a valid context pointer
/// - `nalu` must be a valid pointer from `vidsynt_hevc_parse_nalu_from_bytes`
/// - `out_vps` must be a valid pointer to write to
#[no_mangle]
pub unsafe extern "C" fn vidsynt_hevc_nalu_get_vps(
    ctx: *mut VidsyntHevcContext,
    nalu: *const VidsyntHevcNalu,
    out_vps: *mut *const VidsyntHevcVideoParameterSet,
) -> VidsyntResult {
    if ctx.is_null() || nalu.is_null() || out_vps.is_null() {
        return VidsyntResult::InvalidParameter;
    }

    let ctx = &mut *ctx;
    let nalu = &*(nalu as *const Nalu);

    // Extract VPS from NAL value
    let rust_vps = match &nalu.value {
        NaluValue::VpsNut(vps) => vps,
        _ => return VidsyntResult::DataNotAvailable,
    };

    // Convert ProfileTierLevel
    let ptl = convert_profile_tier_level(&rust_vps.profile_tier_level);
    ctx.ffi_profile_tier_levels.push(Box::new(ptl));
    let ptl_ptr = ctx.ffi_profile_tier_levels.last().unwrap().as_ref() as *const _;

    // Convert SubLayerOrderingInfo if present
    let sub_layer_ordering_info_ptr = if let Some(ref info) = rust_vps.sub_layer_ordering_info {
        let converted = VidsyntHevcSubLayerOrderingInfo {
            max_latency_increase_plus1: info.max_latency_increase_plus1,
            max_dec_pic_buffering_minus1: info.max_dec_pic_buffering_minus1,
            max_num_reorder_pics: info.max_num_reorder_pics,
        };
        ctx.ffi_sub_layer_ordering_infos.push(Box::new(converted));
        ctx.ffi_sub_layer_ordering_infos.last().unwrap().as_ref() as *const _
    } else {
        ptr::null()
    };

    // Convert TimingInfo if present
    let timing_info_ptr = if let Some(ref info) = rust_vps.timing_info {
        let converted = VidsyntHevcTimingInfo {
            vps_num_units_in_tick: info.vps_num_units_in_tick,
            vps_time_scale: info.vps_time_scale,
            vps_num_ticks_poc_diff_one_minus1: info.vps_num_ticks_poc_diff_one_minus1.unwrap_or(0xFFFFFFFF),
        };
        ctx.ffi_timing_infos.push(Box::new(converted));
        ctx.ffi_timing_infos.last().unwrap().as_ref() as *const _
    } else {
        ptr::null()
    };

    // Create C-compatible VPS
    let c_vps = VidsyntHevcVideoParameterSet {
        vps_video_parameter_set_id: rust_vps.vps_video_parameter_set_id,
        vps_base_layer_internal_flag: if rust_vps.vps_base_layer_internal_flag { 1 } else { 0 },
        vps_base_layer_available_flag: if rust_vps.vps_base_layer_available_flag { 1 } else { 0 },
        vps_max_layers_minus1: rust_vps.vps_max_layers_minus1,
        vps_max_sub_layers_minus1: rust_vps.vps_max_sub_layers_minus1,
        vps_temporal_id_nesting_flag: if rust_vps.vps_temporal_id_nesting_flag { 1 } else { 0 },
        vps_max_layer_id: rust_vps.vps_max_layer_id,
        vps_num_layer_sets_minus1: rust_vps.vps_num_layer_sets_minus1,
        profile_tier_level: ptl_ptr,
        sub_layer_ordering_info: sub_layer_ordering_info_ptr,
        timing_info: timing_info_ptr,
    };

    // Store in context and return pointer
    ctx.ffi_vps.push(Box::new(c_vps));
    *out_vps = ctx.ffi_vps.last().unwrap().as_ref() as *const _;

    VidsyntResult::Success
}

/// Helper function to convert Rust ProfileTierLevel to C type
fn convert_profile_tier_level(rust_ptl: &RustProfileTierLevel) -> VidsyntHevcProfileTierLevel {
    let general = &rust_ptl.general;

    VidsyntHevcProfileTierLevel {
        general_profile_space: general.profile_space,
        general_tier_flag: if general.tier_flag { 1 } else { 0 },
        general_profile_idc: general.profile_idc,
        general_profile_compatibility_flag: general.profile_compatibility_flags.map(|b| if b { 1 } else { 0 }),
        general_progressive_source_flag: if general.progressive_source_flag { 1 } else { 0 },
        general_interlaced_source_flag: if general.interlaced_source_flag { 1 } else { 0 },
        general_non_packed_constraint_flag: if general.non_packed_constraint_flag { 1 } else { 0 },
        general_frame_only_constraint_flag: if general.frame_only_constraint_flag { 1 } else { 0 },
        general_level_idc: general.level_idc.unwrap_or(0xFF),
    }
}
