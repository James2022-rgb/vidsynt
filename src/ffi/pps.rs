//! Picture Parameter Set accessor functions

use super::context::VidsyntHevcContext;
use super::types::*;
use crate::h265::nalu::{Nalu, NaluValue};
use crate::h265::pps::Tiles as RustTiles;
use crate::h265::pps::DeblockingFilterControl as RustDeblockingFilterControl;
use std::ptr;

/// Get the PPS from a parsed NAL unit
///
/// # Parameters
/// - `ctx`: Context (needed for allocating converted structures)
/// - `nalu`: Pointer to a parsed NAL unit
/// - `out_pps`: Output pointer to receive the PPS
///
/// # Returns
/// - `VIDSYNT_SUCCESS` on success
/// - `VIDSYNT_ERROR_INVALID_PARAMETER` if any pointer is null
/// - `VIDSYNT_ERROR_DATA_NOT_AVAILABLE` if the NAL unit is not a PPS
///
/// # Safety
/// - `ctx` must be a valid context pointer
/// - `nalu` must be a valid pointer from `vidsynt_hevc_parse_nalu_from_bytes`
/// - `out_pps` must be a valid pointer to write to
#[no_mangle]
pub unsafe extern "C" fn vidsynt_hevc_nalu_get_pps(
    ctx: *mut VidsyntHevcContext,
    nalu: *const VidsyntHevcNalu,
    out_pps: *mut *const VidsyntHevcPictureParameterSet,
) -> VidsyntResult {
    if ctx.is_null() || nalu.is_null() || out_pps.is_null() {
        return VidsyntResult::InvalidParameter;
    }

    let ctx = &mut *ctx;
    let nalu = &*(nalu as *const Nalu);

    // Extract PPS from NAL value
    let rust_pps = match &nalu.value {
        NaluValue::PpsNut(pps) => pps,
        _ => return VidsyntResult::DataNotAvailable,
    };

    // Convert Tiles if present
    let tiles_ptr = if let Some(ref tiles) = rust_pps.tiles {
        let converted = convert_tiles(tiles);
        ctx.ffi_tiles.push(Box::new(converted));
        ctx.ffi_tiles.last().unwrap().as_ref() as *const _
    } else {
        ptr::null()
    };

    // Convert DeblockingFilterControl if present
    let deblocking_filter_control_ptr = if let Some(ref dfc) = rust_pps.deblocking_filter_control {
        let converted = convert_deblocking_filter_control(dfc);
        ctx.ffi_deblocking_filter_controls.push(Box::new(converted));
        ctx.ffi_deblocking_filter_controls.last().unwrap().as_ref() as *const _
    } else {
        ptr::null()
    };

    // Create C-compatible PPS
    let c_pps = VidsyntHevcPictureParameterSet {
        nuh_temporal_id_plus1: rust_pps.nuh_temporal_id_plus1,
        pps_pic_parameter_set_id: rust_pps.pps_pic_parameter_set_id,
        pps_seq_parameter_set_id: rust_pps.pps_seq_parameter_set_id,
        dependent_slice_segments_enabled_flag: if rust_pps.dependent_slice_segments_enabled_flag { 1 } else { 0 },
        output_flag_present_flag: if rust_pps.output_flag_present_flag { 1 } else { 0 },
        sign_data_hiding_enabled_flag: if rust_pps.sign_data_hiding_enabled_flag { 1 } else { 0 },
        cabac_init_present_flag: if rust_pps.cabac_init_present_flag { 1 } else { 0 },
        num_extra_slice_header_bits: rust_pps.num_extra_slice_header_bits,
        num_ref_idx_l0_default_active_minus1: rust_pps.num_ref_idx_l0_default_active_minus1,
        num_ref_idx_l1_default_active_minus1: rust_pps.num_ref_idx_l1_default_active_minus1,
        init_qp_minus26: rust_pps.init_qp_minus26,
        constrained_intra_pred_flag: if rust_pps.constrained_intra_pred_flag { 1 } else { 0 },
        transform_skip_enabled_flag: if rust_pps.transform_skip_enabled_flag { 1 } else { 0 },
        cu_qp_delta_enabled_flag: if rust_pps.cu_qp_delta_enabled_flag { 1 } else { 0 },
        diff_cu_qp_delta_depth: rust_pps.diff_cu_qp_delta_depth.unwrap_or(0xFF),
        pps_cb_qp_offset: rust_pps.pps_cb_qp_offset,
        pps_cr_qp_offset: rust_pps.pps_cr_qp_offset,
        pps_slice_chroma_qp_offsets_present_flag: if rust_pps.pps_slice_chroma_qp_offsets_present_flag { 1 } else { 0 },
        weighted_pred_flag: if rust_pps.weighted_pred_flag { 1 } else { 0 },
        weighted_bipred_flag: if rust_pps.weighted_bipred_flag { 1 } else { 0 },
        transquant_bypass_enabled_flag: if rust_pps.transquant_bypass_enabled_flag { 1 } else { 0 },
        entropy_coding_sync_enabled_flag: if rust_pps.entropy_coding_sync_enabled_flag { 1 } else { 0 },
        pps_loop_filter_across_slices_enabled_flag: if rust_pps.pps_loop_filter_across_slices_enabled_flag { 1 } else { 0 },
        pps_scaling_list_data_present_flag: if rust_pps.pps_scaling_list_data_present_flag { 1 } else { 0 },
        lists_modification_present_flag: if rust_pps.lists_modification_present_flag { 1 } else { 0 },
        log2_parallel_merge_level_minus2: rust_pps.log2_parallel_merge_level_minus2,
        slice_segment_header_extension_present_flag: if rust_pps.slice_segment_header_extension_present_flag { 1 } else { 0 },
        pps_extension_present_flag: if rust_pps.pps_extension_present_flag { 1 } else { 0 },
        tiles: tiles_ptr,
        deblocking_filter_control: deblocking_filter_control_ptr,
    };

    // Store in context and return pointer
    ctx.ffi_pps.push(Box::new(c_pps));
    *out_pps = ctx.ffi_pps.last().unwrap().as_ref() as *const _;

    VidsyntResult::Success
}

/// Helper function to convert Rust Tiles to C type
fn convert_tiles(rust_tiles: &RustTiles) -> VidsyntHevcTiles {
    VidsyntHevcTiles {
        num_tile_columns_minus1: rust_tiles.num_tile_columns_minus1,
        num_tile_rows_minus1: rust_tiles.num_tile_rows_minus1,
        uniform_spacing_flag: if rust_tiles.uniform_spacing_flag { 1 } else { 0 },
        loop_filter_across_tiles_enabled_flag: if rust_tiles.loop_filter_across_tiles_enabled_flag { 1 } else { 0 },
        column_width_minus1: rust_tiles.column_width_minus1,
        row_height_minus1:   rust_tiles.row_height_minus1,
    }
}

/// Helper function to convert Rust DeblockingFilterControl to C type
fn convert_deblocking_filter_control(rust_dfc: &RustDeblockingFilterControl) -> VidsyntHevcDeblockingFilterControl {
    VidsyntHevcDeblockingFilterControl {
        deblocking_filter_override_enabled_flag:
            if rust_dfc.deblocking_filter_override_enabled_flag { 1 } else { 0 },
        pps_deblocking_filter_disabled_flag: if rust_dfc.pps_deblocking_filter_disabled_flag { 1 } else { 0 },
        // For Option<i8> fields, use 0 as sentinel when None (since valid range is typically -12 to +12)
        pps_beta_offset_div2: rust_dfc.pps_beta_offset_div2.unwrap_or(0),
        pps_tc_offset_div2: rust_dfc.pps_tc_offset_div2.unwrap_or(0),
    }
}
