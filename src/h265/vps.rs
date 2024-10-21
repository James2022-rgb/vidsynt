
use std::io::{self, Read};

use bitstream_io::BitRead as _;
use bitstream_io::{BigEndian, BitReader};

use crate::base::read_exp_golomb_ue;
use crate::h265::ptl::{ProfileTierLevel, SubLayerOrderingInfo};

/// See _7.3.2.1 Video parameter set RBSP syntax_ in the spec.
#[derive(Debug, Clone, Copy)]
pub struct VideoParameterSet {
  /// Identifies the VPS for reference by other syntax elements.
  pub vps_video_parameter_set_id: u8,
  pub vps_base_layer_internal_flag: bool,
  pub vps_base_layer_available_flag: bool,
  /// Specifies the maximum allowed number of layers in each CVS referring to the VPS.
  pub vps_max_layers_minus1: u8,
  /// Specifies the maximum number of temporal sub-layers that may be present in each CVS referring to the VPS.
  pub vps_max_sub_layers_minus1: u8,
  pub vps_temporal_id_nesting_flag: bool,
  // `vps_reserved_0xffff_16bits`: 16 bits
  pub profile_tier_level: ProfileTierLevel,
  pub vps_max_layer_id: u8,
  pub vps_num_layer_sets_minus1: u16,
  /// `Some` means `vps_sub_layer_ordering_info_present_flag == true`.
  pub sub_layer_ordering_info: Option<SubLayerOrderingInfo>,
  /// `Some` means `vps_timing_info_present_flag == true`.
  pub timing_info: Option<TimingInfo>,
}

#[derive(Debug, Clone, Copy)]
pub struct TimingInfo {
  pub vps_num_units_in_tick: u32,
  pub vps_time_scale: u32,
  /// `Some` means `vps_poc_proportional_to_timing_flag == true`.
  pub vps_num_ticks_poc_diff_one_minus1: Option<u32>,
}

impl VideoParameterSet {
  pub fn from_rbsp_reader<R: Read>(reader: &mut R) -> Result<Self, io::Error> {
    // See `video_parameter_set_rbsp()` in _7.3.2.1 Video parameter set RBSP syntax_.
    let mut bit_reader = BitReader::endian(reader, BigEndian);

    let vps_video_parameter_set_id = bit_reader.read(4)?;
    let vps_base_layer_internal_flag = bit_reader.read_bit()?;
    let vps_base_layer_available_flag = bit_reader.read_bit()?;
    let vps_max_layers_minus1 = bit_reader.read(6)?;
    let vps_max_sub_layers_minus1 = bit_reader.read(3)?;
    let vps_temporal_id_nesting_flag = bit_reader.read_bit()?;
    // `vps_reserved_0xffff_16bits`: 16 bits
    bit_reader.read::<u32>(16)?;

    let profile_tier_level = ProfileTierLevel::from_reader(
      bit_reader
        .reader()
        .expect("Byte-alignment expected"),
      true,
      vps_max_sub_layers_minus1,
    )?;

    let vps_sub_layer_ordering_info_present_flag = bit_reader.read_bit()?;
    let sub_layer_ordering_info = if vps_sub_layer_ordering_info_present_flag {
      let mut sub_layer_ordering_info = SubLayerOrderingInfo {
        max_latency_increase_plus1: [0; 7],
        max_dec_pic_buffering_minus1: [0; 7],
        max_num_reorder_pics: [0; 7],
      };
      for i in 0..=vps_max_sub_layers_minus1 {
        sub_layer_ordering_info.max_dec_pic_buffering_minus1[i as usize] = read_exp_golomb_ue(&mut bit_reader)? as u8;
        sub_layer_ordering_info.max_num_reorder_pics[i as usize] = read_exp_golomb_ue(&mut bit_reader)? as u8;
        sub_layer_ordering_info.max_latency_increase_plus1[i as usize] = read_exp_golomb_ue(&mut bit_reader)?;
      }
      Some(sub_layer_ordering_info)
    }
    else {
      None
    };

    let vps_max_layer_id: u8 = bit_reader.read(6)?;
    let vps_num_layer_sets_minus1 = read_exp_golomb_ue(&mut bit_reader)? as u16;

    for i in 1..=vps_num_layer_sets_minus1 {
      for j in 0..=vps_max_layer_id {
        let layer_id_included_flag = bit_reader.read_bit()?;
      }
    }

    let vps_timing_info_present_flag = bit_reader.read_bit()?;
    let timing_info = if vps_timing_info_present_flag {
      let vps_num_units_in_tick: u32 = bit_reader.read(32)?;
      let vps_time_scale: u32 = bit_reader.read(32)?;
      let vps_poc_proportional_to_timing_flag = bit_reader.read_bit()?;
      let vps_num_ticks_poc_diff_one_minus1 = if vps_poc_proportional_to_timing_flag {
        let vps_num_ticks_poc_diff_one_minus1 = read_exp_golomb_ue(&mut bit_reader)?;
        Some(vps_num_ticks_poc_diff_one_minus1)
      }
      else {
        None
      };

      let vps_num_hrd_parameters = read_exp_golomb_ue(&mut bit_reader)?;
      if vps_num_hrd_parameters > 0 {
        todo!("vps_num_hrd_parameters > 0 not supported");
      }

      Some(TimingInfo {
        vps_num_units_in_tick,
        vps_time_scale,
        vps_num_ticks_poc_diff_one_minus1,
      })
    }
    else {
      None
    };

    let vps_extension_flag = bit_reader.read_bit()?;

    // Skip VPS extension.

    // `rbsp_trailing_bits()`
    bit_reader.read_unary1()?;
    bit_reader.byte_align();

    Ok(Self {
      vps_video_parameter_set_id,
      vps_base_layer_internal_flag,
      vps_base_layer_available_flag,
      vps_max_layers_minus1,
      vps_max_sub_layers_minus1,
      vps_temporal_id_nesting_flag,
      profile_tier_level,
      vps_max_layer_id,
      vps_num_layer_sets_minus1,
      sub_layer_ordering_info,
      timing_info,
    })
  }
}

