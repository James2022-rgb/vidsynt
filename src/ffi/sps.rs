//! Sequence Parameter Set accessor functions

use super::context::VidsyntHevcContext;
use super::types::*;
use crate::h265::nalu::{Nalu, NaluValue};
use crate::h265::ptl::ProfileTierLevel as RustProfileTierLevel;
use crate::h265::ptl::SubLayerOrderingInfo as RustSubLayerOrderingInfo;
use crate::h265::sps::ConformanceWindow as RustConformanceWindow;
use crate::h265::sps::Vui as RustVui;
use crate::h265::sps::VideoSignalType as RustVideoSignalType;
use crate::h265::sps::ColourDescription as RustColourDescription;
use crate::h265::sps::ChromaLocInfo as RustChromaLocInfo;
use crate::h265::sps::VuiTimingInfo as RustVuiTimingInfo;
use crate::h265::rps::{
    ShortTermReferencePictureSet as RustShortTermReferencePictureSet,
    ShortTermReferencePictureSetValue as RustShortTermReferencePictureSetValue,
};
use std::ptr;

/// Get the SPS from a parsed NAL unit
///
/// # Parameters
/// - `ctx`: Context (needed for allocating converted structures)
/// - `nalu`: Pointer to a parsed NAL unit
/// - `out_sps`: Output pointer to receive the SPS
///
/// # Returns
/// - `VIDSYNT_SUCCESS` on success
/// - `VIDSYNT_ERROR_INVALID_PARAMETER` if any pointer is null
/// - `VIDSYNT_ERROR_DATA_NOT_AVAILABLE` if the NAL unit is not an SPS
///
/// # Safety
/// - `ctx` must be a valid context pointer
/// - `nalu` must be a valid pointer from `vidsynt_hevc_parse_nalu_from_bytes`
/// - `out_sps` must be a valid pointer to write to
#[no_mangle]
pub unsafe extern "C" fn vidsynt_hevc_nalu_get_sps(
    ctx: *mut VidsyntHevcContext,
    nalu: *const VidsyntHevcNalu,
    out_sps: *mut *const VidsyntHevcSequenceParameterSet,
) -> VidsyntResult {
    if ctx.is_null() || nalu.is_null() || out_sps.is_null() {
        return VidsyntResult::InvalidParameter;
    }

    let ctx = &mut *ctx;
    let nalu = &*(nalu as *const Nalu);

    // Extract SPS from NAL value
    let rust_sps = match &nalu.value {
        NaluValue::SpsNut(sps) => sps,
        _ => return VidsyntResult::DataNotAvailable,
    };

    // Convert ProfileTierLevel
    let ptl = convert_profile_tier_level(&rust_sps.profile_tier_level);
    ctx.ffi_profile_tier_levels.push(Box::new(ptl));
    let ptl_ptr = ctx.ffi_profile_tier_levels.last().unwrap().as_ref() as *const _;

    // Convert ConformanceWindow if present
    let conformance_window_ptr = if let Some(ref win) = rust_sps.conformance_window {
        let converted = convert_conformance_window(win);
        ctx.ffi_conformance_windows.push(Box::new(converted));
        ctx.ffi_conformance_windows.last().unwrap().as_ref() as *const _
    } else {
        ptr::null()
    };

    // Convert SubLayerOrderingInfo if present
    let sub_layer_ordering_info_ptr = if let Some(ref info) = rust_sps.sub_layer_ordering_info {
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

    // Convert VUI if present
    let vui_ptr = if let Some(ref vui) = rust_sps.vui {
        convert_vui(ctx, vui)
    } else {
        ptr::null()
    };

    // Convert short-term reference picture sets
    let (short_term_ref_pic_sets_ptr, short_term_ref_pic_set_count) =
        if rust_sps.short_term_ref_pic_sets.is_empty() {
            (ptr::null(), 0)
        } else {
            let converted: Box<[VidsyntHevcShortTermRefPicSet]> = rust_sps
                .short_term_ref_pic_sets
                .iter()
                .map(convert_short_term_ref_pic_set)
                .collect::<Vec<_>>()
                .into_boxed_slice();
            let count = converted.len();
            ctx.ffi_short_term_ref_pic_sets.push(converted);
            let ptr = ctx.ffi_short_term_ref_pic_sets.last().unwrap().as_ptr();
            (ptr, count)
        };

    // Create C-compatible SPS
    let c_sps = VidsyntHevcSequenceParameterSet {
        sps_video_parameter_set_id: rust_sps.sps_video_parameter_set_id,
        sps_max_sub_layers_minus1: rust_sps.sps_max_sub_layers_minus1,
        sps_temporal_id_nesting_flag: if rust_sps.sps_temporal_id_nesting_flag { 1 } else { 0 },
        sps_seq_parameter_set_id: rust_sps.sps_seq_parameter_set_id,
        chroma_format_idc: rust_sps.chroma_format_idc,
        separate_colour_plane_flag: match rust_sps.separate_colour_plane_flag {
            Some(true) => 1,
            Some(false) => 0,
            None => 0xFF,
        },
        pic_width_in_luma_samples: rust_sps.pic_width_in_luma_samples,
        pic_height_in_luma_samples: rust_sps.pic_height_in_luma_samples,
        bit_depth_luma_minus8: rust_sps.bit_depth_luma_minus8,
        bit_depth_chroma_minus8: rust_sps.bit_depth_chroma_minus8,
        log2_max_pic_order_cnt_lsb_minus4: rust_sps.log2_max_pic_order_cnt_lsb_minus4,
        log2_min_luma_coding_block_size_minus3: rust_sps.log2_min_luma_coding_block_size_minus3,
        log2_diff_max_min_luma_coding_block_size: rust_sps.log2_diff_max_min_luma_coding_block_size,
        log2_min_luma_transform_block_size_minus2: rust_sps.log2_min_luma_transform_block_size_minus2,
        log2_diff_max_min_luma_transform_block_size: rust_sps.log2_diff_max_min_luma_transform_block_size,
        max_transform_hierarchy_depth_inter: rust_sps.max_transform_hierarchy_depth_inter,
        max_transform_hierarchy_depth_intra: rust_sps.max_transform_hierarchy_depth_intra,
        scaling_list_enabled_flag: if rust_sps.scaling_list_enabled_flag { 1 } else { 0 },
        amp_enabled_flag: if rust_sps.amp_enabled_flag { 1 } else { 0 },
        sample_adaptive_offset_enabled_flag: if rust_sps.sample_adaptive_offset_enabled_flag { 1 } else { 0 },
        pcm_enabled_flag: if rust_sps.pcm_enabled_flag { 1 } else { 0 },
        pcm_loop_filter_disabled_flag: if rust_sps.pcm_loop_filter_disabled_flag { 1 } else { 0 },
        long_term_ref_pics_present_flag: if rust_sps.long_term_ref_pics_present_flag { 1 } else { 0 },
        num_long_term_ref_pics_sps: rust_sps.num_long_term_ref_pics_sps,
        sps_temporal_mvp_enabled_flag: if rust_sps.sps_temporal_mvp_enabled_flag { 1 } else { 0 },
        strong_intra_smoothing_enabled_flag: if rust_sps.strong_intra_smoothing_enabled_flag { 1 } else { 0 },
        profile_tier_level: ptl_ptr,
        conformance_window: conformance_window_ptr,
        sub_layer_ordering_info: sub_layer_ordering_info_ptr,
        vui: vui_ptr,
        short_term_ref_pic_sets: short_term_ref_pic_sets_ptr,
        short_term_ref_pic_set_count,
    };

    // Cache the Rust SPS for later activation
    let sps_id = rust_sps.sps_seq_parameter_set_id;
    ctx.sps_cache.insert(sps_id, Box::new(rust_sps.clone()));

    // Store in context and return pointer
    ctx.ffi_sps.push(Box::new(c_sps));
    *out_sps = ctx.ffi_sps.last().unwrap().as_ref() as *const _;

    VidsyntResult::Success
}

/// Get resolution from an SPS
///
/// # Parameters
/// - `sps`: Pointer to an SPS
/// - `out_width`: Output pointer for width in luma samples
/// - `out_height`: Output pointer for height in luma samples
///
/// # Returns
/// - `VIDSYNT_SUCCESS` on success
/// - `VIDSYNT_ERROR_INVALID_PARAMETER` if any pointer is null
///
/// # Safety
/// - `sps` must be a valid pointer from `vidsynt_hevc_nalu_get_sps`
/// - `out_width` and `out_height` must be valid pointers to write to
#[no_mangle]
pub unsafe extern "C" fn vidsynt_hevc_sps_get_resolution(
    sps: *const VidsyntHevcSequenceParameterSet,
    out_width: *mut u32,
    out_height: *mut u32,
) -> VidsyntResult {
    if sps.is_null() || out_width.is_null() || out_height.is_null() {
        return VidsyntResult::InvalidParameter;
    }

    let sps = &*sps;
    *out_width = sps.pic_width_in_luma_samples;
    *out_height = sps.pic_height_in_luma_samples;

    VidsyntResult::Success
}

/// Get bit depth from an SPS
///
/// # Parameters
/// - `sps`: Pointer to an SPS
/// - `out_luma`: Output pointer for luma bit depth
/// - `out_chroma`: Output pointer for chroma bit depth
///
/// # Returns
/// - `VIDSYNT_SUCCESS` on success
/// - `VIDSYNT_ERROR_INVALID_PARAMETER` if any pointer is null
///
/// # Safety
/// - `sps` must be a valid pointer from `vidsynt_hevc_nalu_get_sps`
/// - `out_luma` and `out_chroma` must be valid pointers to write to
#[no_mangle]
pub unsafe extern "C" fn vidsynt_hevc_sps_get_bit_depth(
    sps: *const VidsyntHevcSequenceParameterSet,
    out_luma: *mut u8,
    out_chroma: *mut u8,
) -> VidsyntResult {
    if sps.is_null() || out_luma.is_null() || out_chroma.is_null() {
        return VidsyntResult::InvalidParameter;
    }

    let sps = &*sps;
    *out_luma = sps.bit_depth_luma_minus8 + 8;
    *out_chroma = sps.bit_depth_chroma_minus8 + 8;

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

/// Helper function to convert Rust ConformanceWindow to C type
fn convert_conformance_window(rust_win: &RustConformanceWindow) -> VidsyntHevcConformanceWindow {
    VidsyntHevcConformanceWindow {
        conf_win_left_offset: rust_win.conf_win_left_offset,
        conf_win_right_offset: rust_win.conf_win_right_offset,
        conf_win_top_offset: rust_win.conf_win_top_offset,
        conf_win_bottom_offset: rust_win.conf_win_bottom_offset,
    }
}

/// Helper function to convert Rust VUI to C type (with nested structures)
unsafe fn convert_vui(ctx: &mut VidsyntHevcContext, rust_vui: &RustVui) -> *const VidsyntHevcVui {
    // Convert video_signal_type if present
    let video_signal_type_ptr = if let Some(ref vst) = rust_vui.video_signal_type {
        // Convert colour_description if present within video_signal_type
        let colour_description_ptr = if let Some(ref cd) = vst.colour_description {
            let converted = VidsyntHevcColourDescription {
                colour_primaries: cd.colour_primaries,
                transfer_characteristics: cd.transfer_characteristics,
                matrix_coeffs: cd.matrix_coeffs,
            };
            ctx.ffi_colour_descriptions.push(Box::new(converted));
            ctx.ffi_colour_descriptions.last().unwrap().as_ref() as *const _
        } else {
            ptr::null()
        };

        let converted = VidsyntHevcVideoSignalType {
            video_format: vst.video_format,
            video_full_range_flag: if vst.video_full_range_flag { 1 } else { 0 },
            colour_description: colour_description_ptr,
        };
        ctx.ffi_video_signal_types.push(Box::new(converted));
        ctx.ffi_video_signal_types.last().unwrap().as_ref() as *const _
    } else {
        ptr::null()
    };

    // Convert chroma_loc_info if present
    let chroma_loc_info_ptr = if let Some(ref cli) = rust_vui.chroma_loc_info {
        let converted = VidsyntHevcChromaLocInfo {
            chroma_sample_loc_type_top_field: cli.chroma_sample_loc_type_top_field,
            chroma_sample_loc_type_bottom_field: cli.chroma_sample_loc_type_bottom_field,
        };
        ctx.ffi_chroma_loc_infos.push(Box::new(converted));
        ctx.ffi_chroma_loc_infos.last().unwrap().as_ref() as *const _
    } else {
        ptr::null()
    };

    // Convert vui_timing_info if present
    let vui_timing_info_ptr = if let Some(ref vti) = rust_vui.vui_timing_info {
        let converted = VidsyntHevcVuiTimingInfo {
            vui_num_units_in_tick: vti.vui_num_units_in_tick,
            vui_time_scale: vti.vui_time_scale,
            vui_num_ticks_poc_diff_one_minus1: vti.vui_num_ticks_poc_diff_one_minus1.unwrap_or(0xFFFFFFFF),
        };
        ctx.ffi_vui_timing_infos.push(Box::new(converted));
        ctx.ffi_vui_timing_infos.last().unwrap().as_ref() as *const _
    } else {
        ptr::null()
    };

    // Create the VUI structure
    let converted_vui = VidsyntHevcVui {
        aspect_ratio_info_present_flag: if rust_vui.aspect_ratio_info_present_flag { 1 } else { 0 },
        aspect_ratio_idc: rust_vui.aspect_ratio_idc,
        sar_width: rust_vui.sar_width,
        sar_height: rust_vui.sar_height,
        video_signal_type: video_signal_type_ptr,
        chroma_loc_info: chroma_loc_info_ptr,
        neutral_chroma_indication_flag: if rust_vui.neutral_chroma_indication_flag { 1 } else { 0 },
        field_seq_flag: if rust_vui.field_seq_flag { 1 } else { 0 },
        frame_field_info_present_flag: if rust_vui.frame_field_info_present_flag { 1 } else { 0 },
        vui_timing_info: vui_timing_info_ptr,
    };

    ctx.ffi_vuis.push(Box::new(converted_vui));
    ctx.ffi_vuis.last().unwrap().as_ref() as *const _
}

/// Convert a Rust short-term reference picture set to its C-ABI form.
fn convert_short_term_ref_pic_set(
    rps: &RustShortTermReferencePictureSet,
) -> VidsyntHevcShortTermRefPicSet {
    let inter_pred = rps.inter_ref_pic_set_prediction_flag.unwrap_or(false);

    match &rps.value {
        RustShortTermReferencePictureSetValue::InterRefPicSetPrediction(p) => {
            VidsyntHevcShortTermRefPicSet {
                flags: VidsyntHevcShortTermRefPicSetFlags {
                    inter_ref_pic_set_prediction_flag: if inter_pred { 1 } else { 0 },
                    delta_rps_sign: p.delta_rps_sign as u8,
                },
                delta_idx_minus1: p.delta_idx_minus1.unwrap_or(0),
                use_delta_flag: if p.use_delta_flag { 1 } else { 0 },
                abs_delta_rps_minus1: p.abs_delta_rps_minus1,
                used_by_curr_pic_flag: if p.used_by_curr_pic_flag { 1 } else { 0 },
                used_by_curr_pic_s0_flag: 0,
                used_by_curr_pic_s1_flag: 0,
                num_negative_pics: 0,
                num_positive_pics: 0,
                delta_poc_s0_minus1: [0; 16],
                delta_poc_s1_minus1: [0; 16],
            }
        }
        RustShortTermReferencePictureSetValue::NonInterRefPicSetPrediction(p) => {
            let used_s0 = p
                .used_by_curr_pic_s0_flag
                .iter()
                .enumerate()
                .fold(0u16, |acc, (i, &b)| acc | ((b as u16) << i));
            let used_s1 = p
                .used_by_curr_pic_s1_flag
                .iter()
                .enumerate()
                .fold(0u16, |acc, (i, &b)| acc | ((b as u16) << i));

            VidsyntHevcShortTermRefPicSet {
                flags: VidsyntHevcShortTermRefPicSetFlags {
                    inter_ref_pic_set_prediction_flag: if inter_pred { 1 } else { 0 },
                    delta_rps_sign: 0,
                },
                delta_idx_minus1: 0,
                use_delta_flag: 0,
                abs_delta_rps_minus1: 0,
                used_by_curr_pic_flag: 0,
                used_by_curr_pic_s0_flag: used_s0,
                used_by_curr_pic_s1_flag: used_s1,
                num_negative_pics: p.num_negative_pics,
                num_positive_pics: p.num_positive_pics,
                delta_poc_s0_minus1: p.delta_poc_s0_minus1,
                delta_poc_s1_minus1: p.delta_poc_s1_minus1,
            }
        }
    }
}
