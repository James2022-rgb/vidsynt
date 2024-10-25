//! PPS(Picture Parameter Set)

use std::io::{self, Read};

use bitstream_io::BitRead as _;
use bitstream_io::{BigEndian, BitReader};

use crate::base::{read_exp_golomb_ue, read_exp_golomb_se};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PictureParameterSet {
    /// From the NAL unit header.
    pub nuh_temporal_id_plus1: u8,
    pub pps_pic_parameter_set_id: u8,
    pub pps_seq_parameter_set_id: u8,
    /// Specifies the presence of `dependent_slice_segment_flag` in the slice segment headers for coded pictures referring to the PPS.
    pub dependent_slice_segments_enabled_flag: bool,
    pub output_flag_present_flag: bool,
    pub sign_data_hiding_enabled_flag: bool,
    pub cabac_init_present_flag: bool,
    pub num_extra_slice_header_bits: u8,
    pub num_ref_idx_l0_default_active_minus1: u8,
    pub num_ref_idx_l1_default_active_minus1: u8,
    pub init_qp_minus26: i8,
    pub constrained_intra_pred_flag: bool,
    pub transform_skip_enabled_flag: bool,
    pub cu_qp_delta_enabled_flag: bool,
    /// `Some` means `cu_qp_delta_enabled_flag == true`.
    pub diff_cu_qp_delta_depth: Option<u8>,
    pub pps_cb_qp_offset: i8,
    pub pps_cr_qp_offset: i8,
    pub pps_slice_chroma_qp_offsets_present_flag: bool,
    pub weighted_pred_flag: bool,
    pub weighted_bipred_flag: bool,
    pub transquant_bypass_enabled_flag: bool,
    pub entropy_coding_sync_enabled_flag: bool,
    /// `Some` means `tiles_enabled_flag == true`.
    pub tiles: Option<Tiles>,
    pub pps_loop_filter_across_slices_enabled_flag: bool,
    /// `Some` means `deblocking_filter_control_present_flag == true`.
    pub deblocking_filter_control: Option<DeblockingFilterControl>,
    pub pps_scaling_list_data_present_flag: bool,
    pub lists_modification_present_flag: bool,
    pub log2_parallel_merge_level_minus2: u8,
    pub slice_segment_header_extension_present_flag: bool,
    pub pps_extension_present_flag: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tiles {
    pub num_tile_columns_minus1: u8,
    pub num_tile_rows_minus1: u8,
    pub uniform_spacing_flag: bool,
    pub loop_filter_across_tiles_enabled_flag: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeblockingFilterControl {
    /// Specifies that the deblocking filter is disabled for pictures referring to the PPS unless overriden by information present in the slice header.
    pub pps_deblocking_filter_disabled_flag: bool,
    /// Specifies the default deblocking parameter offset for β that is applied for slices referring to the PPS, unless overriden by information present in the slice header.
    ///
    /// `Some` means `pps_deblocking_filter_disabled_flag == false`.
    pub pps_beta_offset_div2: Option<i8>,
    /// Specifies the default deblocking parameter offset for tC that is applied for slices referring to the PPS, unless overriden by information present in the slice header.
    ///
    /// `Some` means `pps_deblocking_filter_disabled_flag == false`.
    pub pps_tc_offset_div2: Option<i8>,
}

impl Default for Tiles {
    fn default() -> Self {
        Self {
            num_tile_columns_minus1: 0,
            num_tile_rows_minus1: 0,
            uniform_spacing_flag: true,
            loop_filter_across_tiles_enabled_flag: true,
        }
    }
}

impl PictureParameterSet {
    pub fn from_rbsp_reader<R: Read>(
        reader: &mut R,
        nuh_temporal_id_plus1: u8,
    ) -> Result<Self, io::Error> {
        // See `pic_parameter_set_rbsp()` in _7.3.2.3 Picture parameter set RBSP syntax_.
        let mut bit_reader = BitReader::endian(reader, BigEndian);

        let pps_pic_parameter_set_id: u8 = read_exp_golomb_ue(&mut bit_reader)? as _;
        let pps_seq_parameter_set_id: u8 = read_exp_golomb_ue(&mut bit_reader)? as _;
        let dependent_slice_segments_enabled_flag = bit_reader.read_bit()?;

        let output_flag_present_flag = bit_reader.read_bit()?;

        let num_extra_slice_header_bits: u8 = bit_reader.read(3)?;
        let sign_data_hiding_enabled_flag = bit_reader.read_bit()?;
        let cabac_init_present_flag = bit_reader.read_bit()?;

        let num_ref_idx_l0_default_active_minus1: u8 = read_exp_golomb_ue(&mut bit_reader)? as _;
        let num_ref_idx_l1_default_active_minus1: u8 = read_exp_golomb_ue(&mut bit_reader)? as _;

        let init_qp_minus26: i8 = read_exp_golomb_se(&mut bit_reader)? as _;

        let constrained_intra_pred_flag = bit_reader.read_bit()?;
        let transform_skip_enabled_flag = bit_reader.read_bit()?;

        let cu_qp_delta_enabled_flag = bit_reader.read_bit()?;
        let diff_cu_qp_delta_depth: Option<u8> = if cu_qp_delta_enabled_flag {
            let diff_cu_qp_delta_depth: u8 = read_exp_golomb_ue(&mut bit_reader)? as _;
            Some(diff_cu_qp_delta_depth)
        } else {
            None
        };

        let pps_cb_qp_offset: i8 = read_exp_golomb_se(&mut bit_reader)? as _;
        let pps_cr_qp_offset: i8 = read_exp_golomb_se(&mut bit_reader)? as _;

        let pps_slice_chroma_qp_offsets_present_flag = bit_reader.read_bit()?;
        let weighted_pred_flag = bit_reader.read_bit()?;
        let weighted_bipred_flag = bit_reader.read_bit()?;
        let transquant_bypass_enabled_flag = bit_reader.read_bit()?;
        let tiles_enabled_flag = bit_reader.read_bit()?;
        let entropy_coding_sync_enabled_flag = bit_reader.read_bit()?;

        let tiles: Option<Tiles> = if tiles_enabled_flag {
            let uniform_spacing_flag = bit_reader.read_bit()?;
            let num_tile_columns_minus1: u8 = read_exp_golomb_ue(&mut bit_reader)? as _;
            let num_tile_rows_minus1: u8 = read_exp_golomb_ue(&mut bit_reader)? as _;

            todo!("tiles_enabled_flag == true not supported");
        } else {
            None
        };

        let pps_loop_filter_across_slices_enabled_flag = bit_reader.read_bit()?;

        let deblocking_filter_control_present_flag = bit_reader.read_bit()?;
        let deblocking_filter_control = if deblocking_filter_control_present_flag {
            let deblocking_filter_override_enabled_flag = bit_reader.read_bit()?;
            if deblocking_filter_override_enabled_flag {
                todo!("deblocking_filter_override_enabled_flag == true not supported");
            }

            let pps_deblocking_filter_disabled_flag = bit_reader.read_bit()?;
            let pps_deblocking_filter_params = if !pps_deblocking_filter_disabled_flag {
                let pps_beta_offset_div2: i8 = read_exp_golomb_se(&mut bit_reader)? as _;
                let pps_tc_offset_div2: i8 = read_exp_golomb_se(&mut bit_reader)? as _;
                Some((pps_beta_offset_div2, pps_tc_offset_div2))
            } else {
                None
            };

            Some(DeblockingFilterControl {
                pps_deblocking_filter_disabled_flag,
                pps_beta_offset_div2: pps_deblocking_filter_params.map(|x| x.0),
                pps_tc_offset_div2: pps_deblocking_filter_params.map(|x| x.1),
            })
        } else {
            None
        };

        let pps_scaling_list_data_present_flag = bit_reader.read_bit()?;
        if pps_scaling_list_data_present_flag {
            todo!("pps_scaling_list_data_present_flag == true not supported");
        }

        let lists_modification_present_flag = bit_reader.read_bit()?;
        let log2_parallel_merge_level_minus2: u8 = read_exp_golomb_ue(&mut bit_reader)? as _;

        let slice_segment_header_extension_present_flag = bit_reader.read_bit()?;
        let pps_extension_present_flag = bit_reader.read_bit()?;
        if pps_extension_present_flag {
            todo!("pps_extension_present_flag == true not supported");
        }

        Ok(Self {
            nuh_temporal_id_plus1,
            pps_pic_parameter_set_id,
            pps_seq_parameter_set_id,
            dependent_slice_segments_enabled_flag,
            output_flag_present_flag,
            sign_data_hiding_enabled_flag,
            cabac_init_present_flag,
            num_extra_slice_header_bits,
            num_ref_idx_l0_default_active_minus1,
            num_ref_idx_l1_default_active_minus1,
            init_qp_minus26,
            constrained_intra_pred_flag,
            transform_skip_enabled_flag,
            cu_qp_delta_enabled_flag,
            diff_cu_qp_delta_depth,
            pps_cb_qp_offset,
            pps_cr_qp_offset,
            pps_slice_chroma_qp_offsets_present_flag,
            weighted_pred_flag,
            weighted_bipred_flag,
            transquant_bypass_enabled_flag,
            entropy_coding_sync_enabled_flag,
            tiles,
            pps_loop_filter_across_slices_enabled_flag,
            deblocking_filter_control,
            pps_scaling_list_data_present_flag,
            lists_modification_present_flag,
            log2_parallel_merge_level_minus2,
            slice_segment_header_extension_present_flag,
            pps_extension_present_flag,
        })
    }
}
