//! C-compatible type definitions for the vidsynt FFI
//!
//! All structures use `#[repr(C)]` to ensure C-compatible memory layout.
//! Rust naming conventions are maintained where possible.

use std::os::raw::{c_int, c_uint};

/// Convert u8 to VidsyntHevcNaluType
///
/// This is safe because VidsyntHevcNaluType is `#[repr(u8)]` and accepts all u8 values.
#[inline]
pub(crate) fn u8_to_hevc_nalu_type(value: u8) -> VidsyntHevcNaluType {
    unsafe { std::mem::transmute(value) }
}

/// Convert VidsyntHevcNaluType to u8
///
/// This is safe because VidsyntHevcNaluType is `#[repr(u8)]`.
#[inline]
pub(crate) fn hevc_nalu_type_to_u8(nal_type: VidsyntHevcNaluType) -> u8 {
    unsafe { std::mem::transmute(nal_type) }
}

/// Result code for FFI functions
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VidsyntResult {
    /// Operation succeeded
    Success = 0,
    /// Invalid parameter (null pointer, invalid size, etc.)
    InvalidParameter = 1,
    /// Failed to parse bitstream data
    ParseFailed = 2,
    /// Out of memory
    OutOfMemory = 3,
    /// Unsupported feature (e.g., scaling lists not implemented)
    UnsupportedFeature = 4,
    /// Invalid NAL unit type
    InvalidNaluType = 5,
    /// The requested data is not available (e.g., getting SPS from a PPS NAL)
    DataNotAvailable = 6,
}

/// HEVC NAL unit type (C-compatible enum)
///
/// H.265/HEVC NAL unit types as defined in Table 7-1 of ITU-T H.265 specification.
/// These values can be compared with the `_0` field of `VidsyntHevcNaluType`.
///
/// All constants are prefixed with VIDSYNT_HEVC_NALU_ to distinguish from potential
/// future H.264 NAL unit types.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VidsyntHevcNaluType {
    /// TRAIL_N - Coded slice segment of a non-TSA, non-STSA trailing picture (non-reference)
    VIDSYNT_HEVC_NALU_TRAIL_N = 0,
    /// TRAIL_R - Coded slice segment of a non-TSA, non-STSA trailing picture (reference)
    VIDSYNT_HEVC_NALU_TRAIL_R = 1,
    /// TSA_N - Coded slice segment of a TSA picture (non-reference)
    VIDSYNT_HEVC_NALU_TSA_N = 2,
    /// TSA_R - Coded slice segment of a TSA picture (reference)
    VIDSYNT_HEVC_NALU_TSA_R = 3,
    /// STSA_N - Coded slice segment of an STSA picture (non-reference)
    VIDSYNT_HEVC_NALU_STSA_N = 4,
    /// STSA_R - Coded slice segment of an STSA picture (reference)
    VIDSYNT_HEVC_NALU_STSA_R = 5,
    /// RADL_N - Coded slice segment of a RADL picture (non-reference)
    VIDSYNT_HEVC_NALU_RADL_N = 6,
    /// RADL_R - Coded slice segment of a RADL picture (reference)
    VIDSYNT_HEVC_NALU_RADL_R = 7,
    /// RASL_N - Coded slice segment of a RASL picture (non-reference)
    VIDSYNT_HEVC_NALU_RASL_N = 8,
    /// RASL_R - Coded slice segment of a RASL picture (reference)
    VIDSYNT_HEVC_NALU_RASL_R = 9,
    /// RSV_VCL_N10 - Reserved non-IRAP SLNR VCL NAL unit type
    VIDSYNT_HEVC_NALU_RSV_VCL_N10 = 10,
    /// RSV_VCL_R11 - Reserved non-IRAP sub-layer reference VCL NAL unit type
    VIDSYNT_HEVC_NALU_RSV_VCL_R11 = 11,
    /// RSV_VCL_N12 - Reserved non-IRAP SLNR VCL NAL unit type
    VIDSYNT_HEVC_NALU_RSV_VCL_N12 = 12,
    /// RSV_VCL_R13 - Reserved non-IRAP sub-layer reference VCL NAL unit type
    VIDSYNT_HEVC_NALU_RSV_VCL_R13 = 13,
    /// RSV_VCL_N14 - Reserved non-IRAP SLNR VCL NAL unit type
    VIDSYNT_HEVC_NALU_RSV_VCL_N14 = 14,
    /// RSV_VCL_R15 - Reserved non-IRAP sub-layer reference VCL NAL unit type
    VIDSYNT_HEVC_NALU_RSV_VCL_R15 = 15,
    /// BLA_W_LP - Coded slice segment of a BLA picture
    VIDSYNT_HEVC_NALU_BLA_W_LP = 16,
    /// BLA_W_RADL - Coded slice segment of a BLA picture
    VIDSYNT_HEVC_NALU_BLA_W_RADL = 17,
    /// BLA_N_LP - Coded slice segment of a BLA picture
    VIDSYNT_HEVC_NALU_BLA_N_LP = 18,
    /// IDR_W_RADL - Coded slice segment of an IDR picture
    VIDSYNT_HEVC_NALU_IDR_W_RADL = 19,
    /// IDR_N_LP - Coded slice segment of an IDR picture
    VIDSYNT_HEVC_NALU_IDR_N_LP = 20,
    /// CRA_NUT - Coded slice segment of a CRA picture
    VIDSYNT_HEVC_NALU_CRA_NUT = 21,
    /// RSV_IRAP_VCL22 - Reserved IRAP VCL NAL unit type
    VIDSYNT_HEVC_NALU_RSV_IRAP_VCL22 = 22,
    /// RSV_IRAP_VCL23 - Reserved IRAP VCL NAL unit type
    VIDSYNT_HEVC_NALU_RSV_IRAP_VCL23 = 23,
    /// VPS_NUT - Video parameter set
    VIDSYNT_HEVC_NALU_VPS_NUT = 32,
    /// SPS_NUT - Sequence parameter set
    VIDSYNT_HEVC_NALU_SPS_NUT = 33,
    /// PPS_NUT - Picture parameter set
    VIDSYNT_HEVC_NALU_PPS_NUT = 34,
    /// AUD_NUT - Access unit delimiter
    VIDSYNT_HEVC_NALU_AUD_NUT = 35,
    /// EOS_NUT - End of sequence
    VIDSYNT_HEVC_NALU_EOS_NUT = 36,
    /// EOB_NUT - End of bitstream
    VIDSYNT_HEVC_NALU_EOB_NUT = 37,
    /// FD_NUT - Filler data
    VIDSYNT_HEVC_NALU_FD_NUT = 38,
    /// PREFIX_SEI_NUT - Supplemental enhancement information (prefix)
    VIDSYNT_HEVC_NALU_PREFIX_SEI_NUT = 39,
    /// SUFFIX_SEI_NUT - Supplemental enhancement information (suffix)
    VIDSYNT_HEVC_NALU_SUFFIX_SEI_NUT = 40,
}

/// HEVC NAL unit header
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VidsyntHevcNaluHeader {
    pub nal_unit_type: VidsyntHevcNaluType,
    pub nuh_layer_id: u8,
    pub nuh_temporal_id_plus1: u8,
    pub _reserved: u8,  // Padding for alignment
}

/// Opaque handle to a parsed NAL unit
///
/// Use accessor functions to extract data from this handle.
#[repr(C)]
pub struct VidsyntHevcNalu {
    _private: [u8; 0],
}

/// Profile, tier, and level information
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VidsyntHevcProfileTierLevel {
    pub general_profile_space: u8,
    pub general_tier_flag: u8,  // bool as u8
    pub general_profile_idc: u8,
    pub general_profile_compatibility_flag: [u8; 32],
    pub general_progressive_source_flag: u8,  // bool
    pub general_interlaced_source_flag: u8,  // bool
    pub general_non_packed_constraint_flag: u8,  // bool
    pub general_frame_only_constraint_flag: u8,  // bool
    pub general_level_idc: u8,
}

/// Conformance window (cropping)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VidsyntHevcConformanceWindow {
    pub conf_win_left_offset: u32,
    pub conf_win_right_offset: u32,
    pub conf_win_top_offset: u32,
    pub conf_win_bottom_offset: u32,
}

/// Video signal type information (from VUI)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VidsyntHevcVideoSignalType {
    pub video_format: u8,
    pub video_full_range_flag: u8,  // bool
    /// Null if colour_description_present_flag is false
    pub colour_description: *const VidsyntHevcColourDescription,
}

/// Colour description (from VUI)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VidsyntHevcColourDescription {
    pub colour_primaries: u8,
    pub transfer_characteristics: u8,
    pub matrix_coeffs: u8,
}

/// Chroma location info (from VUI)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VidsyntHevcChromaLocInfo {
    pub chroma_sample_loc_type_top_field: u8,
    pub chroma_sample_loc_type_bottom_field: u8,
}

/// VUI timing info
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VidsyntHevcVuiTimingInfo {
    pub vui_num_units_in_tick: u32,
    pub vui_time_scale: u32,
    /// Set to 0xFFFFFFFF if vui_poc_proportional_to_timing_flag is false
    pub vui_num_ticks_poc_diff_one_minus1: u32,
}

/// Video Usability Information
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VidsyntHevcVui {
    pub aspect_ratio_info_present_flag: u8,  // bool
    pub aspect_ratio_idc: u8,
    pub sar_width: u16,
    pub sar_height: u16,
    /// Null if video_signal_type_present_flag is false
    pub video_signal_type: *const VidsyntHevcVideoSignalType,
    /// Null if chroma_loc_info_present_flag is false
    pub chroma_loc_info: *const VidsyntHevcChromaLocInfo,
    pub neutral_chroma_indication_flag: u8,  // bool
    pub field_seq_flag: u8,  // bool
    pub frame_field_info_present_flag: u8,  // bool
    /// Null if vui_timing_info_present_flag is false
    pub vui_timing_info: *const VidsyntHevcVuiTimingInfo,
}

/// Sub-layer ordering info (used in VPS and SPS)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VidsyntHevcSubLayerOrderingInfo {
    pub max_latency_increase_plus1: [u32; 7],
    pub max_dec_pic_buffering_minus1: [u8; 7],
    pub max_num_reorder_pics: [u8; 7],
}

/// VPS timing info
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VidsyntHevcTimingInfo {
    pub vps_num_units_in_tick: u32,
    pub vps_time_scale: u32,
    /// Set to 0xFFFFFFFF if vps_poc_proportional_to_timing_flag is false
    pub vps_num_ticks_poc_diff_one_minus1: u32,
}

/// Video Parameter Set
#[repr(C)]
#[derive(Debug)]
pub struct VidsyntHevcVideoParameterSet {
    pub vps_video_parameter_set_id: u8,
    pub vps_base_layer_internal_flag: u8,  // bool
    pub vps_base_layer_available_flag: u8,  // bool
    pub vps_max_layers_minus1: u8,
    pub vps_max_sub_layers_minus1: u8,
    pub vps_temporal_id_nesting_flag: u8,  // bool
    pub vps_max_layer_id: u8,
    pub vps_num_layer_sets_minus1: u16,
    pub profile_tier_level: *const VidsyntHevcProfileTierLevel,
    /// Null if vps_sub_layer_ordering_info_present_flag is false
    pub sub_layer_ordering_info: *const VidsyntHevcSubLayerOrderingInfo,
    /// Null if vps_timing_info_present_flag is false
    pub timing_info: *const VidsyntHevcTimingInfo,
}

/// Flags for short-term reference picture set
///
/// Mirrors the bits of Vulkan Video's `StdVideoH265ShortTermRefPicSetFlags`,
/// but exposed as separate `u8` booleans instead of a packed `uint32_t` bitfield.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VidsyntHevcShortTermRefPicSetFlags {
    pub inter_ref_pic_set_prediction_flag: u8,  // bool
    pub delta_rps_sign: u8,  // bool
}

/// Short-term reference picture set
///
/// Carries the same fields as Vulkan Video's `StdVideoH265ShortTermRefPicSet`
/// for direct field-by-field copy when populating decode parameters. The byte
/// layout differs (no `reserved*` padding fields, packed flag bytes), so use
/// per-field assignment rather than `memcpy`.
///
/// `delta_idx_minus1`, `use_delta_flag`, `abs_delta_rps_minus1`, and
/// `used_by_curr_pic_flag` are only meaningful when
/// `flags.inter_ref_pic_set_prediction_flag` is set; they are zero otherwise.
/// `num_negative_pics`/`num_positive_pics` and the `delta_poc_s{0,1}_minus1`
/// arrays are populated when `inter_ref_pic_set_prediction_flag` is unset.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VidsyntHevcShortTermRefPicSet {
    pub flags: VidsyntHevcShortTermRefPicSetFlags,
    pub delta_idx_minus1: u32,
    /// Bitmask of `use_delta_flag[j]` values for j in 0..NumDeltaPocs[RefRpsIdx]
    pub use_delta_flag: u16,
    pub abs_delta_rps_minus1: u16,
    /// Bitmask of `used_by_curr_pic_flag[j]` values
    pub used_by_curr_pic_flag: u16,
    /// Bitmask of `used_by_curr_pic_s0_flag[i]` values for i in 0..num_negative_pics
    pub used_by_curr_pic_s0_flag: u16,
    /// Bitmask of `used_by_curr_pic_s1_flag[i]` values for i in 0..num_positive_pics
    pub used_by_curr_pic_s1_flag: u16,
    pub num_negative_pics: u8,
    pub num_positive_pics: u8,
    /// Indexed by i in 0..num_negative_pics; entries beyond are zero.
    pub delta_poc_s0_minus1: [u16; 16],
    /// Indexed by i in 0..num_positive_pics; entries beyond are zero.
    pub delta_poc_s1_minus1: [u16; 16],
}

/// Sequence Parameter Set
#[repr(C)]
#[derive(Debug)]
pub struct VidsyntHevcSequenceParameterSet {
    pub sps_video_parameter_set_id: u8,
    pub sps_max_sub_layers_minus1: u8,
    pub sps_temporal_id_nesting_flag: u8,  // bool
    pub sps_seq_parameter_set_id: u8,
    pub chroma_format_idc: u8,
    /// 0xFF if chroma_format_idc != 3
    pub separate_colour_plane_flag: u8,
    pub pic_width_in_luma_samples: u32,
    pub pic_height_in_luma_samples: u32,
    pub bit_depth_luma_minus8: u8,
    pub bit_depth_chroma_minus8: u8,
    pub log2_max_pic_order_cnt_lsb_minus4: u8,
    pub log2_min_luma_coding_block_size_minus3: u8,
    pub log2_diff_max_min_luma_coding_block_size: u8,
    pub log2_min_luma_transform_block_size_minus2: u8,
    pub log2_diff_max_min_luma_transform_block_size: u8,
    pub max_transform_hierarchy_depth_inter: u8,
    pub max_transform_hierarchy_depth_intra: u8,
    pub scaling_list_enabled_flag: u8,  // bool
    pub amp_enabled_flag: u8,  // bool
    pub sample_adaptive_offset_enabled_flag: u8,  // bool
    pub pcm_enabled_flag: u8,  // bool
    pub pcm_loop_filter_disabled_flag: u8,  // bool
    pub long_term_ref_pics_present_flag: u8,  // bool
    pub num_long_term_ref_pics_sps: u8,
    pub sps_temporal_mvp_enabled_flag: u8,  // bool
    pub strong_intra_smoothing_enabled_flag: u8,  // bool

    pub profile_tier_level: *const VidsyntHevcProfileTierLevel,
    /// Null if conformance_window_flag is false
    pub conformance_window: *const VidsyntHevcConformanceWindow,
    /// Null if sps_sub_layer_ordering_info_present_flag is false
    pub sub_layer_ordering_info: *const VidsyntHevcSubLayerOrderingInfo,
    /// Null if vui_parameters_present_flag is false
    pub vui: *const VidsyntHevcVui,

    /// Array of short-term reference picture sets
    pub short_term_ref_pic_sets: *const VidsyntHevcShortTermRefPicSet,
    pub short_term_ref_pic_set_count: usize,
}

/// Tiles configuration
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VidsyntHevcTiles {
    pub num_tile_columns_minus1: u8,
    pub num_tile_rows_minus1: u8,
    pub uniform_spacing_flag: u8,  // bool
    pub loop_filter_across_tiles_enabled_flag: u8,  // bool
    /// Per-column widths in CTBs, each minus 1. The first
    /// `num_tile_columns_minus1` entries are valid; the rest are 0.
    /// All-zero when `uniform_spacing_flag != 0`. Sized to the std
    /// H.265 max (19) to match `StdVideoH265PictureParameterSet`.
    pub column_width_minus1: [u16; 19],
    /// Per-row heights in CTBs, each minus 1. Same semantics as
    /// `column_width_minus1` but for rows. Max 21 entries per std H.265.
    pub row_height_minus1: [u16; 21],
}

/// Deblocking filter control
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VidsyntHevcDeblockingFilterControl {
    /// When true, slice headers MAY override the PPS deblocking parameters
    /// via their own `deblocking_filter_override_flag`. Carries no
    /// additional syntax in the PPS itself, but consumers MUST forward
    /// this flag when reconstructing a `StdVideoH265PictureParameterSet`
    /// for hardware video decode -- otherwise the driver will mis-parse
    /// the slice headers.
    pub deblocking_filter_override_enabled_flag: u8,  // bool
    pub pps_deblocking_filter_disabled_flag: u8,  // bool
    /// Only valid if pps_deblocking_filter_disabled_flag is false, otherwise ignored
    pub pps_beta_offset_div2: i8,
    /// Only valid if pps_deblocking_filter_disabled_flag is false, otherwise ignored
    pub pps_tc_offset_div2: i8,
}

/// Picture Parameter Set
#[repr(C)]
#[derive(Debug)]
pub struct VidsyntHevcPictureParameterSet {
    pub nuh_temporal_id_plus1: u8,
    pub pps_pic_parameter_set_id: u8,
    pub pps_seq_parameter_set_id: u8,
    pub dependent_slice_segments_enabled_flag: u8,  // bool
    pub output_flag_present_flag: u8,  // bool
    pub sign_data_hiding_enabled_flag: u8,  // bool
    pub cabac_init_present_flag: u8,  // bool
    pub num_extra_slice_header_bits: u8,
    pub num_ref_idx_l0_default_active_minus1: u8,
    pub num_ref_idx_l1_default_active_minus1: u8,
    pub init_qp_minus26: i8,
    pub constrained_intra_pred_flag: u8,  // bool
    pub transform_skip_enabled_flag: u8,  // bool
    pub cu_qp_delta_enabled_flag: u8,  // bool
    /// 0xFF if cu_qp_delta_enabled_flag is false
    pub diff_cu_qp_delta_depth: u8,
    pub pps_cb_qp_offset: i8,
    pub pps_cr_qp_offset: i8,
    pub pps_slice_chroma_qp_offsets_present_flag: u8,  // bool
    pub weighted_pred_flag: u8,  // bool
    pub weighted_bipred_flag: u8,  // bool
    pub transquant_bypass_enabled_flag: u8,  // bool
    pub entropy_coding_sync_enabled_flag: u8,  // bool
    pub pps_loop_filter_across_slices_enabled_flag: u8,  // bool
    pub pps_scaling_list_data_present_flag: u8,  // bool
    pub lists_modification_present_flag: u8,  // bool
    pub log2_parallel_merge_level_minus2: u8,
    pub slice_segment_header_extension_present_flag: u8,  // bool
    pub pps_extension_present_flag: u8,  // bool

    /// Null if tiles_enabled_flag is false
    pub tiles: *const VidsyntHevcTiles,
    /// Null if deblocking_filter_control_present_flag is false
    pub deblocking_filter_control: *const VidsyntHevcDeblockingFilterControl,
}

/// Slice segment header
#[repr(C)]
#[derive(Debug)]
pub struct VidsyntHevcSliceSegmentHeader {
    pub nal_unit_type: VidsyntHevcNaluType,
    pub first_slice_segment_in_pic_flag: u8,  // bool
    /// 0xFF if not present (for non-IRAP pictures)
    pub no_output_of_prior_pics_flag: u8,
    pub slice_pic_parameter_set_id: u8,
    /// 0xFF if not present (first slice in pic)
    pub dependent_slice_segment_flag: u8,
    /// Valid only if dependent_slice_segment_flag is false
    pub slice_segment_address: u32,
    /// Valid only for non-IDR slices, 0xFFFF otherwise
    pub slice_pic_order_cnt_lsb: u16,
    /// 1 if the current picture's RPS is taken from the SPS (indexed by
    /// `short_term_ref_pic_set_idx`); 0 if it is parsed inline from the
    /// slice header (`short_term_ref_pic_set` is then populated and
    /// `short_term_ref_pic_set_idx == 0xFF`); 0xFF for IDR slices.
    pub short_term_ref_pic_set_sps_flag: u8,
    /// Index of the short-term reference picture set in the SPS, 0xFF if not from SPS
    pub short_term_ref_pic_set_idx: u8,
    /// Current RPS index
    pub curr_rps_idx: u8,
    /// Inline short-term reference picture set, valid only when
    /// `short_term_ref_pic_set_sps_flag == 0` AND non-IDR. Zeroed
    /// otherwise.
    pub short_term_ref_pic_set: VidsyntHevcShortTermRefPicSet,
    /// Number of bits the inline `short_term_ref_pic_set` consumed in
    /// the slice header. 0 when the inline path was not taken.
    pub num_bits_for_st_ref_pic_set_in_slice: u16,
}

/// POC (Picture Order Count) Computer
///
/// Stateful object for computing picture order counts.
#[repr(C)]
pub struct VidsyntHevcPocComputer {
    _private: [u8; 0],
}
