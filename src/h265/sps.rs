use std::io::{self, Read};

use bitstream_io::BitRead as _;
use bitstream_io::{BigEndian, BitReader};

use crate::base::read_exp_golomb_ue;
use crate::h265::ptl::{ProfileTierLevel, SubLayerOrderingInfo};
use crate::h265::rps::ShortTermReferencePictureSet;

/// See _7.3.2.2 Sequence parameter set RBSP syntax_ in the spec.
#[derive(Debug, Clone)]
pub struct SequenceParameterSet {
    pub sps_video_parameter_set_id: u8,
    pub sps_max_sub_layers_minus1: u8,
    pub sps_temporal_id_nesting_flag: bool,
    pub profile_tier_level: ProfileTierLevel,
    pub pic_width_in_luma_samples: u32,
    pub pic_height_in_luma_samples: u32,
    pub sps_seq_parameter_set_id: u8,
    pub chroma_format_idc: u8,
    /// `Some` means `chroma_format_idc == 3`.
    pub separate_colour_plane_flag: Option<bool>,
    /// `Some` means `conformance_window_flag == true`.
    pub conformance_window: Option<ConformanceWindow>,
    pub bit_depth_luma_minus8: u8,
    pub bit_depth_chroma_minus8: u8,
    pub log2_max_pic_order_cnt_lsb_minus4: u8,
    /// `Some` means `sps_sub_layer_ordering_info_present_flag == true`.
    pub sub_layer_ordering_info: Option<SubLayerOrderingInfo>,
    pub log2_min_luma_coding_block_size_minus3: u8,
    pub log2_diff_max_min_luma_coding_block_size: u8,
    pub log2_min_luma_transform_block_size_minus2: u8,
    pub log2_diff_max_min_luma_transform_block_size: u8,
    pub max_transform_hierarchy_depth_inter: u8,
    pub max_transform_hierarchy_depth_intra: u8,
    pub scaling_list_enabled_flag: bool,
    pub amp_enabled_flag: bool,
    pub sample_adaptive_offset_enabled_flag: bool,
    pub pcm_enabled_flag: bool,
    pub pcm_loop_filter_disabled_flag: bool,
    /// Specifies short-term reference picture sets.
    pub short_term_ref_pic_sets: Vec<ShortTermReferencePictureSet>,
    /// `true` specifies that long-term reference pictures may be used for inter prediction of one or more coded pictures in the CVS.
    pub long_term_ref_pics_present_flag: bool,
    pub num_long_term_ref_pics_sps: u8,
    pub sps_temporal_mvp_enabled_flag: bool,
    pub strong_intra_smoothing_enabled_flag: bool,
    pub vui: Option<Vui>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ConformanceWindow {
    pub conf_win_left_offset: u32,
    pub conf_win_right_offset: u32,
    pub conf_win_top_offset: u32,
    pub conf_win_bottom_offset: u32,
}

/// See _Annex E.2.1 VUI parameters syntax_ in the spec.
#[derive(Debug, Clone, Copy)]
pub struct Vui {
    pub aspect_ratio_info_present_flag: bool,
    pub aspect_ratio_idc: u8,
    pub sar_width: u16,
    pub sar_height: u16,
    /// `Some` means `video_signal_type_present_flag == true`.
    pub video_signal_type: Option<VideoSignalType>,
    /// `Some` means `chroma_loc_info_present_flag == true`.
    pub chroma_loc_info: Option<ChromaLocInfo>,
    pub neutral_chroma_indication_flag: bool,
    pub field_seq_flag: bool,
    pub frame_field_info_present_flag: bool,
    /// `Some` means `default_display_window_flag == true`.
    pub def_disp_win: Option<DefaultDisplayWindow>,
    /// `Some` means `vui_timing_info_present_flag == true`.
    pub vui_timing_info: Option<VuiTimingInfo>,
    /// `Some` means `bitstream_restriction_flag == true`.
    pub bitstream_restriction: Option<BitstreamRestriction>,
}

#[derive(Debug, Clone, Copy)]
pub struct VideoSignalType {
    pub video_format: u8,
    pub video_full_range_flag: bool,
    /// `Some` means `colour_description_present_flag == true
    pub colour_description: Option<ColourDescription>,
}

impl Default for VideoSignalType {
    fn default() -> Self {
        Self {
            video_format: 5,
            video_full_range_flag: false,
            colour_description: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ColourDescription {
    /// _Table E.3 – Colour primaries interpretation using the colour_primaries syntax element_.
    pub colour_primaries: u8,
    pub transfer_characteristics: u8,
    pub matrix_coeffs: u8,
}

impl Default for ColourDescription {
    fn default() -> Self {
        // 2: Unspecified
        Self {
            colour_primaries: 2,
            transfer_characteristics: 2,
            matrix_coeffs: 2,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ChromaLocInfo {
    pub chroma_sample_loc_type_top_field: u8,
    pub chroma_sample_loc_type_bottom_field: u8,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultDisplayWindow {
    pub def_disp_win_left_offset: u16,
    pub def_disp_win_right_offset: u16,
    pub def_disp_win_top_offset: u16,
    pub def_disp_win_bottom_offset: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct VuiTimingInfo {
    /// When not present, inferred to be equal to `vps_num_units_in_tick` of the VPS referred to by the SPS.
    pub vui_num_units_in_tick: u32,
    /// When not present, inferred to be equal to `vps_time_scale` of the VPS referred to by the SPS.
    pub vui_time_scale: u32,
    /// `Some` means `vui_poc_proportional_to_timing_flag == true`.
    pub vui_num_ticks_poc_diff_one_minus1: Option<u32>,
}

#[derive(Debug, Clone, Copy)]
pub struct BitstreamRestriction {
    pub tiles_fixed_structure_flag: bool,
    pub motion_vectors_over_pic_boundaries_flag: bool,
    pub restricted_ref_pic_lists_flag: bool,
    pub min_spatial_segmentation_idc: u16,
    pub max_bytes_per_pic_denom: u8,
    pub max_bits_per_min_cu_denom: u8,
    pub log2_max_mv_length_horizontal: u8,
    pub log2_max_mv_length_vertical: u8,
}

impl Default for BitstreamRestriction {
    fn default() -> Self {
        Self {
            tiles_fixed_structure_flag: false,
            motion_vectors_over_pic_boundaries_flag: true,
            restricted_ref_pic_lists_flag: false, // ? The spec doesn't seem to specify a default value.
            min_spatial_segmentation_idc: 0,
            max_bytes_per_pic_denom: 2,
            max_bits_per_min_cu_denom: 1,
            log2_max_mv_length_horizontal: 15,
            log2_max_mv_length_vertical: 15,
        }
    }
}

impl SequenceParameterSet {
    pub fn from_rbsp_reader<R: Read>(reader: &mut R) -> Result<Self, io::Error> {
        // See `seq_parameter_set_rbsp` in _7.3.2.2 General sequence parameter set RBSP syntax_.
        let mut bit_reader = BitReader::endian(reader, BigEndian);

        let sps_video_parameter_set_id: u8 = bit_reader.read(4)?;
        let sps_max_sub_layers_minus1: u8 = bit_reader.read(3)?;
        let sps_temporal_id_nesting_flag: bool = bit_reader.read_bit()?;

        let profile_tier_level = ProfileTierLevel::from_reader(
            bit_reader.reader().expect("Byte-alignment expected"),
            true,
            sps_max_sub_layers_minus1,
        )?;

        let sps_seq_parameter_set_id: u8 = read_exp_golomb_ue(&mut bit_reader)? as _;
        let chroma_format_idc: u8 = read_exp_golomb_ue(&mut bit_reader)? as _;
        let separate_colour_plane_flag = if chroma_format_idc == 3 {
            Some(bit_reader.read_bit()?)
        } else {
            None
        };
        let pic_width_in_luma_samples: u32 = read_exp_golomb_ue(&mut bit_reader)? as _;
        let pic_height_in_luma_samples: u32 = read_exp_golomb_ue(&mut bit_reader)? as _;
        let conformance_window_flag = bit_reader.read_bit()?;
        let conformance_window = if conformance_window_flag {
            let conf_win_left_offset: u32 = read_exp_golomb_ue(&mut bit_reader)? as _;
            let conf_win_right_offset: u32 = read_exp_golomb_ue(&mut bit_reader)? as _;
            let conf_win_top_offset: u32 = read_exp_golomb_ue(&mut bit_reader)? as _;
            let conf_win_bottom_offset: u32 = read_exp_golomb_ue(&mut bit_reader)? as _;

            Some(ConformanceWindow {
                conf_win_left_offset,
                conf_win_right_offset,
                conf_win_top_offset,
                conf_win_bottom_offset,
            })
        } else {
            None
        };
        let bit_depth_luma_minus8: u8 = read_exp_golomb_ue(&mut bit_reader)? as _;
        let bit_depth_chroma_minus8: u8 = read_exp_golomb_ue(&mut bit_reader)? as _;
        let log2_max_pic_order_cnt_lsb_minus4: u8 = read_exp_golomb_ue(&mut bit_reader)? as _;

        let sps_sub_layer_ordering_info_present_flag = bit_reader.read_bit()?;
        let sub_layer_ordering_info = if sps_sub_layer_ordering_info_present_flag {
            let mut sub_layer_ordering_info = SubLayerOrderingInfo {
                max_latency_increase_plus1: [0; 7],
                max_dec_pic_buffering_minus1: [0; 7],
                max_num_reorder_pics: [0; 7],
            };
            for i in 0..=sps_max_sub_layers_minus1 {
                sub_layer_ordering_info.max_dec_pic_buffering_minus1[i as usize] =
                    read_exp_golomb_ue(&mut bit_reader)? as u8;
                sub_layer_ordering_info.max_num_reorder_pics[i as usize] =
                    read_exp_golomb_ue(&mut bit_reader)? as u8;
                sub_layer_ordering_info.max_latency_increase_plus1[i as usize] =
                    read_exp_golomb_ue(&mut bit_reader)?;
            }
            Some(sub_layer_ordering_info)
        } else {
            None
        };

        let log2_min_luma_coding_block_size_minus3: u8 = read_exp_golomb_ue(&mut bit_reader)? as _;
        let log2_diff_max_min_luma_coding_block_size: u8 =
            read_exp_golomb_ue(&mut bit_reader)? as _;
        let log2_min_luma_transform_block_size_minus2: u8 =
            read_exp_golomb_ue(&mut bit_reader)? as _;
        let log2_diff_max_min_luma_transform_block_size: u8 =
            read_exp_golomb_ue(&mut bit_reader)? as _;
        let max_transform_hierarchy_depth_inter: u8 = read_exp_golomb_ue(&mut bit_reader)? as _;
        let max_transform_hierarchy_depth_intra: u8 = read_exp_golomb_ue(&mut bit_reader)? as _;

        let scaling_list_enabled_flag = bit_reader.read_bit()?;
        if scaling_list_enabled_flag {
            todo!("scaling_list_enabled_flag == true not supported");
        }

        let amp_enabled_flag = bit_reader.read_bit()?;
        let sample_adaptive_offset_enabled_flag = bit_reader.read_bit()?;
        let pcm_enabled_flag = bit_reader.read_bit()?;
        if pcm_enabled_flag {
            todo!("pcm_enabled_flag == true not supported");
        }
        let pcm_loop_filter_disabled_flag = false;

        let short_term_ref_pic_sets = {
            let num_short_term_ref_pic_sets = read_exp_golomb_ue(&mut bit_reader)?;

            let mut short_term_ref_pic_sets: Vec<ShortTermReferencePictureSet> =
                Vec::with_capacity(num_short_term_ref_pic_sets as _);
            for st_rps_index in 0..num_short_term_ref_pic_sets {
                let mut bit_count: u32 = 0;
                let st_rps = ShortTermReferencePictureSet::from_bit_reader(
                    &mut bit_reader,
                    st_rps_index as usize,
                    num_short_term_ref_pic_sets as usize,
                    &mut bit_count,
                )?;
                short_term_ref_pic_sets.push(st_rps);
            }
            short_term_ref_pic_sets
        };

        let long_term_ref_pics_present_flag = bit_reader.read_bit()?;
        let num_long_term_ref_pics_sps = if long_term_ref_pics_present_flag {
            let num_long_term_ref_pics_sps: u8 = read_exp_golomb_ue(&mut bit_reader)? as _;

            todo!("long_term_ref_pics_present_flag == true not supported");
            num_long_term_ref_pics_sps
        } else {
            0
        };

        let sps_temporal_mvp_enabled_flag = bit_reader.read_bit()?;
        let strong_intra_smoothing_enabled_flag = bit_reader.read_bit()?;

        let vui_parameters_present_flag = bit_reader.read_bit()?;
        let vui = if vui_parameters_present_flag {
            let vui = Vui::from_bit_reader(&mut bit_reader)?;
            Some(vui)
        } else {
            None
        };

        Ok(Self {
            sps_video_parameter_set_id,
            sps_max_sub_layers_minus1,
            sps_temporal_id_nesting_flag,
            profile_tier_level,
            pic_width_in_luma_samples,
            pic_height_in_luma_samples,
            sps_seq_parameter_set_id,
            chroma_format_idc,
            separate_colour_plane_flag,
            conformance_window,
            bit_depth_luma_minus8,
            bit_depth_chroma_minus8,
            log2_max_pic_order_cnt_lsb_minus4,
            sub_layer_ordering_info,
            log2_min_luma_coding_block_size_minus3,
            log2_diff_max_min_luma_coding_block_size,
            log2_min_luma_transform_block_size_minus2,
            log2_diff_max_min_luma_transform_block_size,
            max_transform_hierarchy_depth_inter,
            max_transform_hierarchy_depth_intra,
            amp_enabled_flag,
            sample_adaptive_offset_enabled_flag,
            scaling_list_enabled_flag,
            pcm_enabled_flag,
            pcm_loop_filter_disabled_flag,
            short_term_ref_pic_sets,
            long_term_ref_pics_present_flag,
            num_long_term_ref_pics_sps,
            sps_temporal_mvp_enabled_flag,
            strong_intra_smoothing_enabled_flag,
            vui,
        })
    }
}

impl Vui {
    pub fn from_bit_reader<R: Read>(
        bit_reader: &mut BitReader<R, BigEndian>,
    ) -> Result<Self, io::Error> {
        let aspect_ratio_info_present_flag = bit_reader.read_bit()?;
        let (aspect_ratio_idc, sar_width, sar_height) = if aspect_ratio_info_present_flag {
            const EXTENDED_SAR: u8 = 255;

            let aspect_ratio_idc: u8 = bit_reader.read(8)?;
            let (sar_width, sar_height) = if aspect_ratio_idc == EXTENDED_SAR {
                let sar_width = bit_reader.read(16)?;
                let sar_height = bit_reader.read(16)?;
                (sar_width, sar_height)
            } else {
                // Table E.1 – Interpretation of sample aspect ratio indicator
                const PREDEFINED: [(u16, u16); 17] = [
                    (0, 0),
                    (1, 1),
                    (12, 11),
                    (10, 11),
                    (16, 11),
                    (40, 33),
                    (24, 11),
                    (20, 11),
                    (32, 11),
                    (80, 33),
                    (18, 11),
                    (15, 11),
                    (64, 33),
                    (160, 99),
                    (4, 3),
                    (3, 2),
                    (2, 1),
                ];

                if aspect_ratio_idc < PREDEFINED.len() as _ {
                    PREDEFINED[aspect_ratio_idc as usize]
                } else {
                    (0, 0)
                }
            };

            (aspect_ratio_idc, sar_width, sar_height)
        } else {
            (0, 0, 0)
        };

        let overscan_info_present_flag = bit_reader.read_bit()?;
        if overscan_info_present_flag {
            todo!("overscan_info_present_flag == true not supported");
        }

        let video_signal_type_present_flag = bit_reader.read_bit()?;
        let video_signal_type = if video_signal_type_present_flag {
            let video_format: u8 = bit_reader.read(3)?;
            let video_full_range_flag = bit_reader.read_bit()?;

            let colour_description_present_flag = bit_reader.read_bit()?;
            let colour_description = if colour_description_present_flag {
                let colour_primaries: u8 = bit_reader.read(8)?;
                let transfer_characteristics: u8 = bit_reader.read(8)?;
                let matrix_coeffs: u8 = bit_reader.read(8)?;

                Some(ColourDescription {
                    colour_primaries,
                    transfer_characteristics,
                    matrix_coeffs,
                })
            } else {
                None
            };

            Some(VideoSignalType {
                video_format,
                video_full_range_flag,
                colour_description,
            })
        } else {
            None
        };

        let chroma_loc_info_present_flag = bit_reader.read_bit()?;
        let chroma_loc_info: Option<ChromaLocInfo> = if chroma_loc_info_present_flag {
            let chroma_sample_loc_type_top_field: u8 = read_exp_golomb_ue(bit_reader)? as _;
            let chroma_sample_loc_type_bottom_field: u8 = read_exp_golomb_ue(bit_reader)? as _;

            Some(ChromaLocInfo {
                chroma_sample_loc_type_top_field,
                chroma_sample_loc_type_bottom_field,
            })
        } else {
            None
        };

        let neutral_chroma_indication_flag = bit_reader.read_bit()?;
        let field_seq_flag = bit_reader.read_bit()?;
        let frame_field_info_present_flag = bit_reader.read_bit()?;

        let default_display_window_flag = bit_reader.read_bit()?;
        let def_disp_win: Option<DefaultDisplayWindow> = if default_display_window_flag {
            todo!("default_display_window_flag == true not supported");
        } else {
            None
        };

        let vui_timing_info_present_flag = bit_reader.read_bit()?;
        let vui_timing_info: Option<VuiTimingInfo> = if vui_timing_info_present_flag {
            let vui_num_units_in_tick: u32 = bit_reader.read(32)?;
            let vui_time_scale: u32 = bit_reader.read(32)?;

            let vui_poc_proportional_to_timing_flag = bit_reader.read_bit()?;
            let vui_num_ticks_poc_diff_one_minus1 = if vui_poc_proportional_to_timing_flag {
                let vui_num_ticks_poc_diff_one_minus1: u32 = read_exp_golomb_ue(bit_reader)?;

                Some(vui_num_ticks_poc_diff_one_minus1)
            } else {
                None
            };

            let vui_hrd_parameters_present_flag = bit_reader.read_bit()?;
            if vui_hrd_parameters_present_flag {
                todo!("vui_hrd_parameters_present_flag == true not supported");
            }

            Some(VuiTimingInfo {
                vui_num_units_in_tick,
                vui_time_scale,
                vui_num_ticks_poc_diff_one_minus1,
            })
        } else {
            None
        };

        let bitstream_restriction_flag = bit_reader.read_bit()?;
        let bitstream_restriction: Option<BitstreamRestriction> = if bitstream_restriction_flag {
            todo!("bitstream_restriction_flag == true not supported");
        } else {
            None
        };

        Ok(Self {
            aspect_ratio_info_present_flag,
            aspect_ratio_idc,
            sar_width,
            sar_height,
            video_signal_type,
            chroma_loc_info,
            neutral_chroma_indication_flag,
            field_seq_flag,
            frame_field_info_present_flag,
            def_disp_win,
            vui_timing_info,
            bitstream_restriction,
        })
    }
}
