/**
 * vidsynt C API
 * 
 * H.265/HEVC bitstream parser library
 * 
 * This is an auto-generated header file from the Rust vidsynt library.
 */

#ifndef VIDSYNT_H
#define VIDSYNT_H

/* Generated with cbindgen:0.27.0 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>

// HEVC NAL unit type (C-compatible enum)
//
// H.265/HEVC NAL unit types as defined in Table 7-1 of ITU-T H.265 specification.
// These values can be compared with the `_0` field of `VidsyntHevcNaluType`.
//
// All constants are prefixed with VIDSYNT_HEVC_NALU_ to distinguish from potential
// future H.264 NAL unit types.
enum VidsyntHevcNaluType
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
    // TRAIL_N - Coded slice segment of a non-TSA, non-STSA trailing picture (non-reference)
    VIDSYNT_HEVC_NALU_TRAIL_N = 0,
    // TRAIL_R - Coded slice segment of a non-TSA, non-STSA trailing picture (reference)
    VIDSYNT_HEVC_NALU_TRAIL_R = 1,
    // TSA_N - Coded slice segment of a TSA picture (non-reference)
    VIDSYNT_HEVC_NALU_TSA_N = 2,
    // TSA_R - Coded slice segment of a TSA picture (reference)
    VIDSYNT_HEVC_NALU_TSA_R = 3,
    // STSA_N - Coded slice segment of an STSA picture (non-reference)
    VIDSYNT_HEVC_NALU_STSA_N = 4,
    // STSA_R - Coded slice segment of an STSA picture (reference)
    VIDSYNT_HEVC_NALU_STSA_R = 5,
    // RADL_N - Coded slice segment of a RADL picture (non-reference)
    VIDSYNT_HEVC_NALU_RADL_N = 6,
    // RADL_R - Coded slice segment of a RADL picture (reference)
    VIDSYNT_HEVC_NALU_RADL_R = 7,
    // RASL_N - Coded slice segment of a RASL picture (non-reference)
    VIDSYNT_HEVC_NALU_RASL_N = 8,
    // RASL_R - Coded slice segment of a RASL picture (reference)
    VIDSYNT_HEVC_NALU_RASL_R = 9,
    // RSV_VCL_N10 - Reserved non-IRAP SLNR VCL NAL unit type
    VIDSYNT_HEVC_NALU_RSV_VCL_N10 = 10,
    // RSV_VCL_R11 - Reserved non-IRAP sub-layer reference VCL NAL unit type
    VIDSYNT_HEVC_NALU_RSV_VCL_R11 = 11,
    // RSV_VCL_N12 - Reserved non-IRAP SLNR VCL NAL unit type
    VIDSYNT_HEVC_NALU_RSV_VCL_N12 = 12,
    // RSV_VCL_R13 - Reserved non-IRAP sub-layer reference VCL NAL unit type
    VIDSYNT_HEVC_NALU_RSV_VCL_R13 = 13,
    // RSV_VCL_N14 - Reserved non-IRAP SLNR VCL NAL unit type
    VIDSYNT_HEVC_NALU_RSV_VCL_N14 = 14,
    // RSV_VCL_R15 - Reserved non-IRAP sub-layer reference VCL NAL unit type
    VIDSYNT_HEVC_NALU_RSV_VCL_R15 = 15,
    // BLA_W_LP - Coded slice segment of a BLA picture
    VIDSYNT_HEVC_NALU_BLA_W_LP = 16,
    // BLA_W_RADL - Coded slice segment of a BLA picture
    VIDSYNT_HEVC_NALU_BLA_W_RADL = 17,
    // BLA_N_LP - Coded slice segment of a BLA picture
    VIDSYNT_HEVC_NALU_BLA_N_LP = 18,
    // IDR_W_RADL - Coded slice segment of an IDR picture
    VIDSYNT_HEVC_NALU_IDR_W_RADL = 19,
    // IDR_N_LP - Coded slice segment of an IDR picture
    VIDSYNT_HEVC_NALU_IDR_N_LP = 20,
    // CRA_NUT - Coded slice segment of a CRA picture
    VIDSYNT_HEVC_NALU_CRA_NUT = 21,
    // RSV_IRAP_VCL22 - Reserved IRAP VCL NAL unit type
    VIDSYNT_HEVC_NALU_RSV_IRAP_VCL22 = 22,
    // RSV_IRAP_VCL23 - Reserved IRAP VCL NAL unit type
    VIDSYNT_HEVC_NALU_RSV_IRAP_VCL23 = 23,
    // VPS_NUT - Video parameter set
    VIDSYNT_HEVC_NALU_VPS_NUT = 32,
    // SPS_NUT - Sequence parameter set
    VIDSYNT_HEVC_NALU_SPS_NUT = 33,
    // PPS_NUT - Picture parameter set
    VIDSYNT_HEVC_NALU_PPS_NUT = 34,
    // AUD_NUT - Access unit delimiter
    VIDSYNT_HEVC_NALU_AUD_NUT = 35,
    // EOS_NUT - End of sequence
    VIDSYNT_HEVC_NALU_EOS_NUT = 36,
    // EOB_NUT - End of bitstream
    VIDSYNT_HEVC_NALU_EOB_NUT = 37,
    // FD_NUT - Filler data
    VIDSYNT_HEVC_NALU_FD_NUT = 38,
    // PREFIX_SEI_NUT - Supplemental enhancement information (prefix)
    VIDSYNT_HEVC_NALU_PREFIX_SEI_NUT = 39,
    // SUFFIX_SEI_NUT - Supplemental enhancement information (suffix)
    VIDSYNT_HEVC_NALU_SUFFIX_SEI_NUT = 40,
};
#ifndef __cplusplus
typedef uint8_t VidsyntHevcNaluType;
#endif // __cplusplus

// Result code for FFI functions
typedef enum VidsyntResult {
    // Operation succeeded
    Success = 0,
    // Invalid parameter (null pointer, invalid size, etc.)
    InvalidParameter = 1,
    // Failed to parse bitstream data
    ParseFailed = 2,
    // Out of memory
    OutOfMemory = 3,
    // Unsupported feature (e.g., scaling lists not implemented)
    UnsupportedFeature = 4,
    // Invalid NAL unit type
    InvalidNaluType = 5,
    // The requested data is not available (e.g., getting SPS from a PPS NAL)
    DataNotAvailable = 6,
} VidsyntResult;

// Context that owns all allocated FFI objects
//
// All parsing operations allocate memory that is owned by this context.
// Free everything at once by calling `vidsynt_hevc_context_free()`.
typedef struct VidsyntHevcContext VidsyntHevcContext;

// Profile, tier, and level information
typedef struct VidsyntHevcProfileTierLevel {
    uint8_t general_profile_space;
    uint8_t general_tier_flag;
    uint8_t general_profile_idc;
    uint8_t general_profile_compatibility_flag[32];
    uint8_t general_progressive_source_flag;
    uint8_t general_interlaced_source_flag;
    uint8_t general_non_packed_constraint_flag;
    uint8_t general_frame_only_constraint_flag;
    uint8_t general_level_idc;
} VidsyntHevcProfileTierLevel;

// Conformance window (cropping)
typedef struct VidsyntHevcConformanceWindow {
    uint32_t conf_win_left_offset;
    uint32_t conf_win_right_offset;
    uint32_t conf_win_top_offset;
    uint32_t conf_win_bottom_offset;
} VidsyntHevcConformanceWindow;

// Sub-layer ordering info (used in VPS and SPS)
typedef struct VidsyntHevcSubLayerOrderingInfo {
    uint32_t max_latency_increase_plus1[7];
    uint8_t max_dec_pic_buffering_minus1[7];
    uint8_t max_num_reorder_pics[7];
} VidsyntHevcSubLayerOrderingInfo;

// Colour description (from VUI)
typedef struct VidsyntHevcColourDescription {
    uint8_t colour_primaries;
    uint8_t transfer_characteristics;
    uint8_t matrix_coeffs;
} VidsyntHevcColourDescription;

// Video signal type information (from VUI)
typedef struct VidsyntHevcVideoSignalType {
    uint8_t video_format;
    uint8_t video_full_range_flag;
    // Null if colour_description_present_flag is false
    const struct VidsyntHevcColourDescription *colour_description;
} VidsyntHevcVideoSignalType;

// Chroma location info (from VUI)
typedef struct VidsyntHevcChromaLocInfo {
    uint8_t chroma_sample_loc_type_top_field;
    uint8_t chroma_sample_loc_type_bottom_field;
} VidsyntHevcChromaLocInfo;

// VUI timing info
typedef struct VidsyntHevcVuiTimingInfo {
    uint32_t vui_num_units_in_tick;
    uint32_t vui_time_scale;
    // Set to 0xFFFFFFFF if vui_poc_proportional_to_timing_flag is false
    uint32_t vui_num_ticks_poc_diff_one_minus1;
} VidsyntHevcVuiTimingInfo;

// Video Usability Information
typedef struct VidsyntHevcVui {
    uint8_t aspect_ratio_info_present_flag;
    uint8_t aspect_ratio_idc;
    uint16_t sar_width;
    uint16_t sar_height;
    // Null if video_signal_type_present_flag is false
    const struct VidsyntHevcVideoSignalType *video_signal_type;
    // Null if chroma_loc_info_present_flag is false
    const struct VidsyntHevcChromaLocInfo *chroma_loc_info;
    uint8_t neutral_chroma_indication_flag;
    uint8_t field_seq_flag;
    uint8_t frame_field_info_present_flag;
    // Null if vui_timing_info_present_flag is false
    const struct VidsyntHevcVuiTimingInfo *vui_timing_info;
} VidsyntHevcVui;

// Short-term reference picture set
typedef struct VidsyntHevcShortTermRefPicSet {
    uint8_t _private[0];
} VidsyntHevcShortTermRefPicSet;

// Sequence Parameter Set
typedef struct VidsyntHevcSequenceParameterSet {
    uint8_t sps_video_parameter_set_id;
    uint8_t sps_max_sub_layers_minus1;
    uint8_t sps_temporal_id_nesting_flag;
    uint8_t sps_seq_parameter_set_id;
    uint8_t chroma_format_idc;
    // 0xFF if chroma_format_idc != 3
    uint8_t separate_colour_plane_flag;
    uint32_t pic_width_in_luma_samples;
    uint32_t pic_height_in_luma_samples;
    uint8_t bit_depth_luma_minus8;
    uint8_t bit_depth_chroma_minus8;
    uint8_t log2_max_pic_order_cnt_lsb_minus4;
    uint8_t log2_min_luma_coding_block_size_minus3;
    uint8_t log2_diff_max_min_luma_coding_block_size;
    uint8_t log2_min_luma_transform_block_size_minus2;
    uint8_t log2_diff_max_min_luma_transform_block_size;
    uint8_t max_transform_hierarchy_depth_inter;
    uint8_t max_transform_hierarchy_depth_intra;
    uint8_t scaling_list_enabled_flag;
    uint8_t amp_enabled_flag;
    uint8_t sample_adaptive_offset_enabled_flag;
    uint8_t pcm_enabled_flag;
    uint8_t pcm_loop_filter_disabled_flag;
    uint8_t long_term_ref_pics_present_flag;
    uint8_t num_long_term_ref_pics_sps;
    uint8_t sps_temporal_mvp_enabled_flag;
    uint8_t strong_intra_smoothing_enabled_flag;
    const struct VidsyntHevcProfileTierLevel *profile_tier_level;
    // Null if conformance_window_flag is false
    const struct VidsyntHevcConformanceWindow *conformance_window;
    // Null if sps_sub_layer_ordering_info_present_flag is false
    const struct VidsyntHevcSubLayerOrderingInfo *sub_layer_ordering_info;
    // Null if vui_parameters_present_flag is false
    const struct VidsyntHevcVui *vui;
    // Array of short-term reference picture sets
    const struct VidsyntHevcShortTermRefPicSet *short_term_ref_pic_sets;
    uintptr_t short_term_ref_pic_set_count;
} VidsyntHevcSequenceParameterSet;

// Tiles configuration
typedef struct VidsyntHevcTiles {
    uint8_t num_tile_columns_minus1;
    uint8_t num_tile_rows_minus1;
    uint8_t uniform_spacing_flag;
    uint8_t loop_filter_across_tiles_enabled_flag;
} VidsyntHevcTiles;

// Deblocking filter control
typedef struct VidsyntHevcDeblockingFilterControl {
    uint8_t pps_deblocking_filter_disabled_flag;
    // Only valid if pps_deblocking_filter_disabled_flag is false, otherwise ignored
    int8_t pps_beta_offset_div2;
    // Only valid if pps_deblocking_filter_disabled_flag is false, otherwise ignored
    int8_t pps_tc_offset_div2;
} VidsyntHevcDeblockingFilterControl;

// Picture Parameter Set
typedef struct VidsyntHevcPictureParameterSet {
    uint8_t nuh_temporal_id_plus1;
    uint8_t pps_pic_parameter_set_id;
    uint8_t pps_seq_parameter_set_id;
    uint8_t dependent_slice_segments_enabled_flag;
    uint8_t output_flag_present_flag;
    uint8_t sign_data_hiding_enabled_flag;
    uint8_t cabac_init_present_flag;
    uint8_t num_extra_slice_header_bits;
    uint8_t num_ref_idx_l0_default_active_minus1;
    uint8_t num_ref_idx_l1_default_active_minus1;
    int8_t init_qp_minus26;
    uint8_t constrained_intra_pred_flag;
    uint8_t transform_skip_enabled_flag;
    uint8_t cu_qp_delta_enabled_flag;
    // 0xFF if cu_qp_delta_enabled_flag is false
    uint8_t diff_cu_qp_delta_depth;
    int8_t pps_cb_qp_offset;
    int8_t pps_cr_qp_offset;
    uint8_t pps_slice_chroma_qp_offsets_present_flag;
    uint8_t weighted_pred_flag;
    uint8_t weighted_bipred_flag;
    uint8_t transquant_bypass_enabled_flag;
    uint8_t entropy_coding_sync_enabled_flag;
    uint8_t pps_loop_filter_across_slices_enabled_flag;
    uint8_t pps_scaling_list_data_present_flag;
    uint8_t lists_modification_present_flag;
    uint8_t log2_parallel_merge_level_minus2;
    uint8_t slice_segment_header_extension_present_flag;
    uint8_t pps_extension_present_flag;
    // Null if tiles_enabled_flag is false
    const struct VidsyntHevcTiles *tiles;
    // Null if deblocking_filter_control_present_flag is false
    const struct VidsyntHevcDeblockingFilterControl *deblocking_filter_control;
} VidsyntHevcPictureParameterSet;

// Opaque handle to a parsed NAL unit
//
// Use accessor functions to extract data from this handle.
typedef struct VidsyntHevcNalu {
    uint8_t _private[0];
} VidsyntHevcNalu;

// HEVC NAL unit header
typedef struct VidsyntHevcNaluHeader {
    VidsyntHevcNaluType nal_unit_type;
    uint8_t nuh_layer_id;
    uint8_t nuh_temporal_id_plus1;
    uint8_t _reserved;
} VidsyntHevcNaluHeader;

// VPS timing info
typedef struct VidsyntHevcTimingInfo {
    uint32_t vps_num_units_in_tick;
    uint32_t vps_time_scale;
    // Set to 0xFFFFFFFF if vps_poc_proportional_to_timing_flag is false
    uint32_t vps_num_ticks_poc_diff_one_minus1;
} VidsyntHevcTimingInfo;

// Video Parameter Set
typedef struct VidsyntHevcVideoParameterSet {
    uint8_t vps_video_parameter_set_id;
    uint8_t vps_base_layer_internal_flag;
    uint8_t vps_base_layer_available_flag;
    uint8_t vps_max_layers_minus1;
    uint8_t vps_max_sub_layers_minus1;
    uint8_t vps_temporal_id_nesting_flag;
    uint8_t vps_max_layer_id;
    uint16_t vps_num_layer_sets_minus1;
    const struct VidsyntHevcProfileTierLevel *profile_tier_level;
    // Null if vps_sub_layer_ordering_info_present_flag is false
    const struct VidsyntHevcSubLayerOrderingInfo *sub_layer_ordering_info;
    // Null if vps_timing_info_present_flag is false
    const struct VidsyntHevcTimingInfo *timing_info;
} VidsyntHevcVideoParameterSet;

// Slice segment header
typedef struct VidsyntHevcSliceSegmentHeader {
    VidsyntHevcNaluType nal_unit_type;
    uint8_t first_slice_segment_in_pic_flag;
    // 0xFF if not present (for non-IRAP pictures)
    uint8_t no_output_of_prior_pics_flag;
    uint8_t slice_pic_parameter_set_id;
    // 0xFF if not present (first slice in pic)
    uint8_t dependent_slice_segment_flag;
    // Valid only if dependent_slice_segment_flag is false
    uint32_t slice_segment_address;
    // Valid only for non-IDR slices, 0xFFFF otherwise
    uint16_t slice_pic_order_cnt_lsb;
    // Index of the short-term reference picture set in the SPS, 0xFF if not from SPS
    uint8_t short_term_ref_pic_set_idx;
    // Current RPS index
    uint8_t curr_rps_idx;
} VidsyntHevcSliceSegmentHeader;

// POC (Picture Order Count) Computer
//
// Stateful object for computing picture order counts.
typedef struct VidsyntHevcPocComputer {
    uint8_t _private[0];
} VidsyntHevcPocComputer;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

// Create a new vidsynt context
//
// All parsing operations require a context. Free it with `vidsynt_hevc_context_free()`.
//
// # Returns
// Pointer to the new context, or null on allocation failure.
struct VidsyntHevcContext *vidsynt_hevc_context_new(void);

// Free a vidsynt context and all associated memory
//
// After calling this function, all pointers obtained from this context
// become invalid and must not be used.
//
// # Safety
// - `ctx` must be a valid pointer returned from `vidsynt_hevc_context_new()`
// - `ctx` must not be used after this call
// - `ctx` must not be freed more than once
void vidsynt_hevc_context_free(struct VidsyntHevcContext *ctx);

// Set the active SPS for slice parsing
//
// This SPS will be used to provide context when parsing slice segments.
// You must set an active SPS before parsing VCL NAL units (coded slices).
//
// # Parameters
// - `ctx`: Context to update
// - `sps`: SPS to set as active (from `vidsynt_hevc_nalu_get_sps`)
//
// # Returns
// - `VIDSYNT_SUCCESS` on success
// - `VIDSYNT_ERROR_INVALID_PARAMETER` if any pointer is null
//
// # Safety
// - `ctx` must be a valid context pointer
// - `sps` must be a valid pointer from `vidsynt_hevc_nalu_get_sps`
enum VidsyntResult vidsynt_hevc_context_set_active_sps(struct VidsyntHevcContext *ctx,
                                                       const struct VidsyntHevcSequenceParameterSet *sps);

// Set the active PPS for slice parsing
//
// This PPS will be used to provide context when parsing slice segments.
// You must set an active PPS before parsing VCL NAL units (coded slices).
//
// # Parameters
// - `ctx`: Context to update
// - `pps`: PPS to set as active (from `vidsynt_hevc_nalu_get_pps`)
//
// # Returns
// - `VIDSYNT_SUCCESS` on success
// - `VIDSYNT_ERROR_INVALID_PARAMETER` if any pointer is null
//
// # Safety
// - `ctx` must be a valid context pointer
// - `pps` must be a valid pointer from `vidsynt_hevc_nalu_get_pps`
enum VidsyntResult vidsynt_hevc_context_set_active_pps(struct VidsyntHevcContext *ctx,
                                                       const struct VidsyntHevcPictureParameterSet *pps);

// Parse a single NAL unit from raw bytes
//
// This function parses parameter sets (VPS, SPS, PPS) without requiring context.
// For slice segments, you need to parse the parameter sets first.
//
// # Parameters
// - `ctx`: Context that will own the parsed NAL unit
// - `data`: Pointer to NAL unit data (including NAL header)
// - `len`: Length of NAL unit in bytes
// - `out_nalu`: Output pointer to receive the parsed NAL unit
//
// # Returns
// - `VIDSYNT_SUCCESS` on success
// - `VIDSYNT_ERROR_INVALID_PARAMETER` if any pointer is null or len is 0
// - `VIDSYNT_ERROR_PARSE_FAILED` if parsing fails
//
// # Safety
// - `ctx` must be a valid context pointer
// - `data` must point to `len` valid bytes
// - `out_nalu` must be a valid pointer to write to
enum VidsyntResult vidsynt_hevc_parse_nalu_from_bytes(struct VidsyntHevcContext *ctx,
                                                      const uint8_t *data,
                                                      uintptr_t len,
                                                      const struct VidsyntHevcNalu **out_nalu);

// Get the NAL unit type from a parsed NAL unit
//
// # Parameters
// - `nalu`: Pointer to a parsed NAL unit
//
// # Returns
// The NAL unit type
//
// # Safety
// - `nalu` must be a valid pointer from `vidsynt_hevc_parse_nalu_from_bytes`
VidsyntHevcNaluType vidsynt_hevc_nalu_get_type(const struct VidsyntHevcNalu *nalu);

// Get the NAL unit header
//
// # Parameters
// - `nalu`: Pointer to a parsed NAL unit
// - `out_header`: Output pointer to receive header
//
// # Returns
// - `VIDSYNT_SUCCESS` on success
// - `VIDSYNT_ERROR_INVALID_PARAMETER` if any pointer is null
//
// # Safety
// - `nalu` must be a valid pointer from `vidsynt_hevc_parse_nalu_from_bytes`
// - `out_header` must be a valid pointer to write to
enum VidsyntResult vidsynt_hevc_nalu_get_header(const struct VidsyntHevcNalu *nalu,
                                                struct VidsyntHevcNaluHeader *out_header);

// Check if a NAL unit type is an IDR picture
//
// # Parameters
// - `nal_type`: NAL unit type to check
//
// # Returns
// 1 if IDR, 0 otherwise
uint8_t vidsynt_hevc_nalu_type_is_idr(VidsyntHevcNaluType nal_type);

// Check if a NAL unit type is an IRAP (Intra Random Access Point) picture
//
// # Parameters
// - `nal_type`: NAL unit type to check
//
// # Returns
// 1 if IRAP, 0 otherwise
uint8_t vidsynt_hevc_nalu_type_is_irap(VidsyntHevcNaluType nal_type);

// Check if a NAL unit type is a BLA (Broken Link Access) picture
//
// # Parameters
// - `nal_type`: NAL unit type to check
//
// # Returns
// 1 if BLA, 0 otherwise
uint8_t vidsynt_hevc_nalu_type_is_bla(VidsyntHevcNaluType nal_type);

// Check if a NAL unit type is a reference picture
//
// # Parameters
// - `nal_type`: NAL unit type to check
//
// # Returns
// 1 if reference picture, 0 otherwise
uint8_t vidsynt_hevc_nalu_type_is_reference(VidsyntHevcNaluType nal_type);

// Check if a NAL unit type is a RADL (Random Access Decodable Leading) picture
//
// # Parameters
// - `nal_type`: NAL unit type to check
//
// # Returns
// 1 if RADL, 0 otherwise
uint8_t vidsynt_hevc_nalu_type_is_radl(VidsyntHevcNaluType nal_type);

// Check if a NAL unit type is a RASL (Random Access Skipped Leading) picture
//
// # Parameters
// - `nal_type`: NAL unit type to check
//
// # Returns
// 1 if RASL, 0 otherwise
uint8_t vidsynt_hevc_nalu_type_is_rasl(VidsyntHevcNaluType nal_type);

// Check if a NAL unit type is a coded slice segment
//
// # Parameters
// - `nal_type`: NAL unit type to check
//
// # Returns
// 1 if coded slice segment, 0 otherwise
uint8_t vidsynt_hevc_nalu_type_is_coded_slice_segment(VidsyntHevcNaluType nal_type);

// Get the VPS from a parsed NAL unit
//
// # Parameters
// - `ctx`: Context (needed for allocating converted structures)
// - `nalu`: Pointer to a parsed NAL unit
// - `out_vps`: Output pointer to receive the VPS
//
// # Returns
// - `VIDSYNT_SUCCESS` on success
// - `VIDSYNT_ERROR_INVALID_PARAMETER` if any pointer is null
// - `VIDSYNT_ERROR_DATA_NOT_AVAILABLE` if the NAL unit is not a VPS
//
// # Safety
// - `ctx` must be a valid context pointer
// - `nalu` must be a valid pointer from `vidsynt_hevc_parse_nalu_from_bytes`
// - `out_vps` must be a valid pointer to write to
enum VidsyntResult vidsynt_hevc_nalu_get_vps(struct VidsyntHevcContext *ctx,
                                             const struct VidsyntHevcNalu *nalu,
                                             const struct VidsyntHevcVideoParameterSet **out_vps);

// Get the SPS from a parsed NAL unit
//
// # Parameters
// - `ctx`: Context (needed for allocating converted structures)
// - `nalu`: Pointer to a parsed NAL unit
// - `out_sps`: Output pointer to receive the SPS
//
// # Returns
// - `VIDSYNT_SUCCESS` on success
// - `VIDSYNT_ERROR_INVALID_PARAMETER` if any pointer is null
// - `VIDSYNT_ERROR_DATA_NOT_AVAILABLE` if the NAL unit is not an SPS
//
// # Safety
// - `ctx` must be a valid context pointer
// - `nalu` must be a valid pointer from `vidsynt_hevc_parse_nalu_from_bytes`
// - `out_sps` must be a valid pointer to write to
enum VidsyntResult vidsynt_hevc_nalu_get_sps(struct VidsyntHevcContext *ctx,
                                             const struct VidsyntHevcNalu *nalu,
                                             const struct VidsyntHevcSequenceParameterSet **out_sps);

// Get resolution from an SPS
//
// # Parameters
// - `sps`: Pointer to an SPS
// - `out_width`: Output pointer for width in luma samples
// - `out_height`: Output pointer for height in luma samples
//
// # Returns
// - `VIDSYNT_SUCCESS` on success
// - `VIDSYNT_ERROR_INVALID_PARAMETER` if any pointer is null
//
// # Safety
// - `sps` must be a valid pointer from `vidsynt_hevc_nalu_get_sps`
// - `out_width` and `out_height` must be valid pointers to write to
enum VidsyntResult vidsynt_hevc_sps_get_resolution(const struct VidsyntHevcSequenceParameterSet *sps,
                                                   uint32_t *out_width,
                                                   uint32_t *out_height);

// Get bit depth from an SPS
//
// # Parameters
// - `sps`: Pointer to an SPS
// - `out_luma`: Output pointer for luma bit depth
// - `out_chroma`: Output pointer for chroma bit depth
//
// # Returns
// - `VIDSYNT_SUCCESS` on success
// - `VIDSYNT_ERROR_INVALID_PARAMETER` if any pointer is null
//
// # Safety
// - `sps` must be a valid pointer from `vidsynt_hevc_nalu_get_sps`
// - `out_luma` and `out_chroma` must be valid pointers to write to
enum VidsyntResult vidsynt_hevc_sps_get_bit_depth(const struct VidsyntHevcSequenceParameterSet *sps,
                                                  uint8_t *out_luma,
                                                  uint8_t *out_chroma);

// Get the PPS from a parsed NAL unit
//
// # Parameters
// - `ctx`: Context (needed for allocating converted structures)
// - `nalu`: Pointer to a parsed NAL unit
// - `out_pps`: Output pointer to receive the PPS
//
// # Returns
// - `VIDSYNT_SUCCESS` on success
// - `VIDSYNT_ERROR_INVALID_PARAMETER` if any pointer is null
// - `VIDSYNT_ERROR_DATA_NOT_AVAILABLE` if the NAL unit is not a PPS
//
// # Safety
// - `ctx` must be a valid context pointer
// - `nalu` must be a valid pointer from `vidsynt_hevc_parse_nalu_from_bytes`
// - `out_pps` must be a valid pointer to write to
enum VidsyntResult vidsynt_hevc_nalu_get_pps(struct VidsyntHevcContext *ctx,
                                             const struct VidsyntHevcNalu *nalu,
                                             const struct VidsyntHevcPictureParameterSet **out_pps);

// Get the slice segment header from a parsed NAL unit
//
// # Parameters
// - `ctx`: Context (needed for allocating converted structures)
// - `nalu`: Pointer to a parsed NAL unit
// - `out_slice_header`: Output pointer to receive the slice segment header
//
// # Returns
// - `VIDSYNT_SUCCESS` on success
// - `VIDSYNT_ERROR_INVALID_PARAMETER` if any pointer is null
// - `VIDSYNT_ERROR_DATA_NOT_AVAILABLE` if the NAL unit is not a slice
//
// # Safety
// - `ctx` must be a valid context pointer
// - `nalu` must be a valid pointer from `vidsynt_hevc_parse_nalu_from_bytes`
// - `out_slice_header` must be a valid pointer to write to
enum VidsyntResult vidsynt_hevc_nalu_get_slice_header(struct VidsyntHevcContext *ctx,
                                                      const struct VidsyntHevcNalu *nalu,
                                                      const struct VidsyntHevcSliceSegmentHeader **out_slice_header);

// Create a new POC (Picture Order Count) computer
//
// # Parameters
// - `ctx`: Context that will own the POC computer
//
// # Returns
// Pointer to the new POC computer, or null on allocation failure
//
// # Safety
// - `ctx` must be a valid context pointer
struct VidsyntHevcPocComputer *vidsynt_hevc_poc_computer_new(struct VidsyntHevcContext *ctx);

// Compute the Picture Order Count (POC) value for a picture
//
// # Parameters
// - `poc_computer`: Pointer to a POC computer
// - `sps`: Pointer to the SPS referenced by the picture
// - `pps`: Pointer to the PPS referenced by the picture
// - `slice_header`: Pointer to the slice segment header
// - `out_poc`: Output pointer to receive the computed POC value
//
// # Returns
// - `VIDSYNT_SUCCESS` on success
// - `VIDSYNT_ERROR_INVALID_PARAMETER` if any pointer is null
//
// # Safety
// - `poc_computer` must be a valid pointer from `vidsynt_hevc_poc_computer_new`
// - `sps` must be a valid pointer from `vidsynt_hevc_nalu_get_sps`
// - `pps` must be a valid pointer from `vidsynt_hevc_nalu_get_pps`
// - `slice_header` must be a valid pointer from `vidsynt_hevc_nalu_get_slice_header`
// - `out_poc` must be a valid pointer to write to
enum VidsyntResult vidsynt_hevc_poc_compute(struct VidsyntHevcPocComputer *poc_computer,
                                            const struct VidsyntHevcSequenceParameterSet *sps,
                                            const struct VidsyntHevcPictureParameterSet *pps,
                                            const struct VidsyntHevcSliceSegmentHeader *slice_header,
                                            int32_t *out_poc);

// Reset the POC computer for IDR or random access
//
// Call this when starting a new coded video sequence (CVS), such as after
// seeking to an IDR or CRA picture.
//
// # Parameters
// - `poc_computer`: Pointer to a POC computer
//
// # Returns
// - `VIDSYNT_SUCCESS` on success
// - `VIDSYNT_ERROR_INVALID_PARAMETER` if the pointer is null
//
// # Safety
// - `poc_computer` must be a valid pointer from `vidsynt_hevc_poc_computer_new`
enum VidsyntResult vidsynt_hevc_poc_reset(struct VidsyntHevcPocComputer *poc_computer);

// Convert length-prefixed NAL units to Annex B format
//
// Length-prefixed format (used in MP4 containers):
// - Each NAL unit is preceded by N bytes (1-4) indicating its length
//
// Annex B format (used for streaming/decoders):
// - NAL units are separated by start codes (0x000001 or 0x00000001)
//
// # Parameters
// - `ctx`: Context that will own the allocated output buffer
// - `data`: Pointer to length-prefixed NAL unit data
// - `len`: Length of input data in bytes
// - `length_size_minus_one`: Length field size minus 1 (0=1 byte, 1=2 bytes, 2=3 bytes, 3=4 bytes)
// - `out_data`: Output pointer to receive converted Annex B data
// - `out_len`: Output pointer to receive length of converted data
//
// # Returns
// - `VIDSYNT_SUCCESS` on success
// - `VIDSYNT_ERROR_INVALID_PARAMETER` if any pointer is null or length_size_minus_one > 3
// - `VIDSYNT_ERROR_PARSE_FAILED` if data is malformed
//
// # Safety
// - `ctx` must be a valid context pointer
// - `data` must point to `len` valid bytes
// - `out_data` and `out_len` must be valid pointers to write to
enum VidsyntResult vidsynt_hevc_convert_length_prefixed_to_annex_b(struct VidsyntHevcContext *ctx,
                                                                   const uint8_t *data,
                                                                   uintptr_t len,
                                                                   uint8_t length_size_minus_one,
                                                                   const uint8_t **out_data,
                                                                   uintptr_t *out_len);

// Convert Annex B format NAL units to length-prefixed format
//
// Annex B format (used for streaming/decoders):
// - NAL units are separated by start codes (0x000001 or 0x00000001)
//
// Length-prefixed format (used in MP4 containers):
// - Each NAL unit is preceded by N bytes (1-4) indicating its length
//
// # Parameters
// - `ctx`: Context that will own the allocated output buffer
// - `data`: Pointer to Annex B NAL unit data
// - `len`: Length of input data in bytes
// - `length_size_minus_one`: Desired length field size minus 1 (0=1 byte, 1=2 bytes, 2=3 bytes, 3=4 bytes)
// - `out_data`: Output pointer to receive converted length-prefixed data
// - `out_len`: Output pointer to receive length of converted data
//
// # Returns
// - `VIDSYNT_SUCCESS` on success
// - `VIDSYNT_ERROR_INVALID_PARAMETER` if any pointer is null or length_size_minus_one > 3
// - `VIDSYNT_ERROR_PARSE_FAILED` if data is malformed or NAL unit too large for length field
//
// # Safety
// - `ctx` must be a valid context pointer
// - `data` must point to `len` valid bytes
// - `out_data` and `out_len` must be valid pointers to write to
enum VidsyntResult vidsynt_hevc_convert_annex_b_to_length_prefixed(struct VidsyntHevcContext *ctx,
                                                                   const uint8_t *data,
                                                                   uintptr_t len,
                                                                   uint8_t length_size_minus_one,
                                                                   const uint8_t **out_data,
                                                                   uintptr_t *out_len);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* VIDSYNT_H */
