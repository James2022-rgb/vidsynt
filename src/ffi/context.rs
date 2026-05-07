//! Context lifecycle management for the FFI
//!
//! The context owns all allocated memory and provides a clean way to manage
//! the lifetime of parsed objects.

use super::types::*;
use crate::h265::nalu::Nalu;
use crate::h265::vps::VideoParameterSet;
use crate::h265::sps::SequenceParameterSet;
use crate::h265::pps::PictureParameterSet;
use crate::h265::poc::PocComputer;
use crate::h265::slice::SliceSegmentContext;
use std::collections::HashMap;

/// Context that owns all allocated FFI objects
///
/// All parsing operations allocate memory that is owned by this context.
/// Free everything at once by calling `vidsynt_hevc_context_free()`.
pub struct VidsyntHevcContext {
    /// Parsed NAL units
    pub(crate) nalus: Vec<Box<Nalu>>,

    /// Cached Video Parameter Sets (indexed by vps_id)
    pub(crate) vps_cache: HashMap<u8, Box<VideoParameterSet>>,

    /// Cached Sequence Parameter Sets (indexed by sps_id)
    pub(crate) sps_cache: HashMap<u8, Box<SequenceParameterSet>>,

    /// Cached Picture Parameter Sets (indexed by pps_id)
    pub(crate) pps_cache: HashMap<u8, Box<PictureParameterSet>>,

    /// Active SPS (last parsed or explicitly set)
    pub(crate) active_sps: Option<SequenceParameterSet>,

    /// Active PPS (last parsed or explicitly set)
    pub(crate) active_pps: Option<PictureParameterSet>,

    /// POC computers (one per sequence)
    pub(crate) poc_computers: Vec<Box<PocComputer>>,

    /// FFI-specific allocated structures
    /// These are C-compatible representations of Rust structures
    pub(crate) ffi_nalus: Vec<Box<VidsyntHevcNalu>>,
    pub(crate) ffi_vps: Vec<Box<VidsyntHevcVideoParameterSet>>,
    pub(crate) ffi_sps: Vec<Box<VidsyntHevcSequenceParameterSet>>,
    pub(crate) ffi_pps: Vec<Box<VidsyntHevcPictureParameterSet>>,
    pub(crate) ffi_slice_headers: Vec<Box<VidsyntHevcSliceSegmentHeader>>,

    /// Auxiliary FFI structures (for nested data)
    pub(crate) ffi_profile_tier_levels: Vec<Box<VidsyntHevcProfileTierLevel>>,
    pub(crate) ffi_conformance_windows: Vec<Box<VidsyntHevcConformanceWindow>>,
    pub(crate) ffi_vuis: Vec<Box<VidsyntHevcVui>>,
    pub(crate) ffi_video_signal_types: Vec<Box<VidsyntHevcVideoSignalType>>,
    pub(crate) ffi_colour_descriptions: Vec<Box<VidsyntHevcColourDescription>>,
    pub(crate) ffi_chroma_loc_infos: Vec<Box<VidsyntHevcChromaLocInfo>>,
    pub(crate) ffi_vui_timing_infos: Vec<Box<VidsyntHevcVuiTimingInfo>>,
    pub(crate) ffi_sub_layer_ordering_infos: Vec<Box<VidsyntHevcSubLayerOrderingInfo>>,
    pub(crate) ffi_timing_infos: Vec<Box<VidsyntHevcTimingInfo>>,
    pub(crate) ffi_tiles: Vec<Box<VidsyntHevcTiles>>,
    pub(crate) ffi_deblocking_filter_controls: Vec<Box<VidsyntHevcDeblockingFilterControl>>,
    pub(crate) ffi_short_term_ref_pic_sets: Vec<Box<[VidsyntHevcShortTermRefPicSet]>>,

    /// Buffers for format conversion results
    pub(crate) conversion_buffers: Vec<Vec<u8>>,
}

impl VidsyntHevcContext {
    /// Build a SliceSegmentContext from active SPS and PPS
    pub(crate) fn build_slice_context(&self) -> Option<SliceSegmentContext> {
        let sps = self.active_sps.as_ref()?;
        let pps = self.active_pps.as_ref()?;

        Some(SliceSegmentContext {
            dependent_slice_segments_enabled_flag: pps.dependent_slice_segments_enabled_flag,
            pic_width_in_luma_samples: sps.pic_width_in_luma_samples,
            pic_height_in_luma_samples: sps.pic_height_in_luma_samples,
            log2_min_luma_coding_block_size_minus3: sps.log2_min_luma_coding_block_size_minus3,
            log2_diff_max_min_luma_coding_block_size: sps.log2_diff_max_min_luma_coding_block_size,
            num_extra_slice_header_bits: pps.num_extra_slice_header_bits,
            output_flag_present_flag: pps.output_flag_present_flag,
            separate_colour_plane_flag: sps.separate_colour_plane_flag.unwrap_or(false),
            log2_max_pic_order_cnt_lsb_minus4: sps.log2_max_pic_order_cnt_lsb_minus4,
            num_short_term_ref_pic_sets: sps.short_term_ref_pic_sets.len() as u8,
        })
    }

    /// Create a new empty context
    pub fn new() -> Self {
        Self {
            nalus: Vec::new(),
            vps_cache: HashMap::new(),
            sps_cache: HashMap::new(),
            pps_cache: HashMap::new(),
            active_sps: None,
            active_pps: None,
            poc_computers: Vec::new(),
            ffi_nalus: Vec::new(),
            ffi_vps: Vec::new(),
            ffi_sps: Vec::new(),
            ffi_pps: Vec::new(),
            ffi_slice_headers: Vec::new(),
            ffi_profile_tier_levels: Vec::new(),
            ffi_conformance_windows: Vec::new(),
            ffi_vuis: Vec::new(),
            ffi_video_signal_types: Vec::new(),
            ffi_colour_descriptions: Vec::new(),
            ffi_chroma_loc_infos: Vec::new(),
            ffi_vui_timing_infos: Vec::new(),
            ffi_sub_layer_ordering_infos: Vec::new(),
            ffi_timing_infos: Vec::new(),
            ffi_tiles: Vec::new(),
            ffi_deblocking_filter_controls: Vec::new(),
            ffi_short_term_ref_pic_sets: Vec::new(),
            conversion_buffers: Vec::new(),
        }
    }
}

/// Create a new vidsynt context
///
/// All parsing operations require a context. Free it with `vidsynt_hevc_context_free()`.
///
/// # Returns
/// Pointer to the new context, or null on allocation failure.
#[no_mangle]
pub extern "C" fn vidsynt_hevc_context_new() -> *mut VidsyntHevcContext {
    let context = Box::new(VidsyntHevcContext::new());
    Box::into_raw(context)
}

/// Free a vidsynt context and all associated memory
///
/// After calling this function, all pointers obtained from this context
/// become invalid and must not be used.
///
/// # Safety
/// - `ctx` must be a valid pointer returned from `vidsynt_hevc_context_new()`
/// - `ctx` must not be used after this call
/// - `ctx` must not be freed more than once
#[no_mangle]
pub unsafe extern "C" fn vidsynt_hevc_context_free(ctx: *mut VidsyntHevcContext) {
    if !ctx.is_null() {
        // Reconstruct the Box and let it drop, freeing all memory
        let _ = Box::from_raw(ctx);
    }
}

/// Set the active SPS for slice parsing
///
/// This SPS will be used to provide context when parsing slice segments.
/// You must set an active SPS before parsing VCL NAL units (coded slices).
///
/// # Parameters
/// - `ctx`: Context to update
/// - `sps`: SPS to set as active (from `vidsynt_hevc_nalu_get_sps`)
///
/// # Returns
/// - `VIDSYNT_SUCCESS` on success
/// - `VIDSYNT_ERROR_INVALID_PARAMETER` if any pointer is null
///
/// # Safety
/// - `ctx` must be a valid context pointer
/// - `sps` must be a valid pointer from `vidsynt_hevc_nalu_get_sps`
#[no_mangle]
pub unsafe extern "C" fn vidsynt_hevc_context_set_active_sps(
    ctx: *mut VidsyntHevcContext,
    sps: *const VidsyntHevcSequenceParameterSet,
) -> VidsyntResult {
    if ctx.is_null() || sps.is_null() {
        return VidsyntResult::InvalidParameter;
    }

    let ctx = &mut *ctx;
    let sps_ref = &*sps;

    // Find the corresponding Rust SPS in the cache
    // We need to search for it by matching fields since we can't reverse the pointer
    let sps_id = sps_ref.sps_seq_parameter_set_id;

    if let Some(cached_sps) = ctx.sps_cache.get(&sps_id) {
        ctx.active_sps = Some((**cached_sps).clone());
        VidsyntResult::Success
    } else {
        // If not in cache, we can't set it as active
        VidsyntResult::DataNotAvailable
    }
}

/// Set the active PPS for slice parsing
///
/// This PPS will be used to provide context when parsing slice segments.
/// You must set an active PPS before parsing VCL NAL units (coded slices).
///
/// # Parameters
/// - `ctx`: Context to update
/// - `pps`: PPS to set as active (from `vidsynt_hevc_nalu_get_pps`)
///
/// # Returns
/// - `VIDSYNT_SUCCESS` on success
/// - `VIDSYNT_ERROR_INVALID_PARAMETER` if any pointer is null
///
/// # Safety
/// - `ctx` must be a valid context pointer
/// - `pps` must be a valid pointer from `vidsynt_hevc_nalu_get_pps`
#[no_mangle]
pub unsafe extern "C" fn vidsynt_hevc_context_set_active_pps(
    ctx: *mut VidsyntHevcContext,
    pps: *const VidsyntHevcPictureParameterSet,
) -> VidsyntResult {
    if ctx.is_null() || pps.is_null() {
        return VidsyntResult::InvalidParameter;
    }

    let ctx = &mut *ctx;
    let pps_ref = &*pps;

    // Find the corresponding Rust PPS in the cache
    let pps_id = pps_ref.pps_pic_parameter_set_id;

    if let Some(cached_pps) = ctx.pps_cache.get(&pps_id) {
        ctx.active_pps = Some((**cached_pps).clone());
        VidsyntResult::Success
    } else {
        // If not in cache, we can't set it as active
        VidsyntResult::DataNotAvailable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_lifecycle() {
        // Create context
        let ctx = vidsynt_hevc_context_new();
        assert!(!ctx.is_null());

        // Free context
        unsafe {
            vidsynt_hevc_context_free(ctx);
        }
    }

    #[test]
    fn test_context_free_null() {
        // Freeing null should be safe (no-op)
        unsafe {
            vidsynt_hevc_context_free(std::ptr::null_mut());
        }
    }
}
