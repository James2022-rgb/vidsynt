
use std::io::{self, Read};

use bitstream_io::{BigEndian, BitReader};
use bitstream_io::BitRead as _;

use crate::base::read_exp_golomb_ue;
use crate::h265::nalu::NaluType;
use crate::h265::rps::ShortTermReferencePictureSet;

#[derive(Debug, Clone, Copy)]
pub struct SliceSegmentContext {
  pub dependent_slice_segments_enabled_flag: bool,
  pub pic_width_in_luma_samples: u32,
  pub pic_height_in_luma_samples: u32,
  pub log2_min_luma_coding_block_size_minus3: u8,
  pub log2_diff_max_min_luma_coding_block_size: u8,
  pub num_extra_slice_header_bits: u8,
  pub output_flag_present_flag: bool,
  pub separate_colour_plane_flag: bool,
  pub log2_max_pic_order_cnt_lsb_minus4: u8,
  pub num_short_term_ref_pic_sets: u8,
}

/// See `slice_segment_layer_rbsp()` in _7.3.2.9 Slice segment layer RBSP syntax_ in the spec.
#[derive(Debug, Clone)]
pub struct SliceSegmentLayer {
  pub header: SliceSegmentHeader,
}

/// See `slice_segment_header()` in _7.3.6 Slice segment header syntax_ in the spec.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SliceSegmentHeader {
  pub nal_unit_type: NaluType,
  /// Indicates whether the slice segment is the first slice segment of the picture in decoding order.
  pub first_slice_segment_in_pic_flag: bool,
  /// Affects the output of previously-decoded pictures in the decoded picture buffer after the decoding of an IDR or a BLA picture that is not the first picture in the bitstream.
  pub no_output_of_prior_pics_flag: Option<bool>,
  /// Specifies the value of `pps_pic_parameter_set_id` for the PPS in use. `[0, 63]`.
  pub slice_pic_parameter_set_id: u8,
  pub dependent_slice_segment_flag: Option<bool>,
  pub slice_segment_address: Option<u32>,
  /// `Some` when `dependent_slice_segment_flag != Some(true)`.
  pub short_term_ref_pic_set_sps_flag: Option<bool>,
  /// `Some` when `short_term_ref_pic_set_sps_flag != Some(true)`.
  pub short_term_ref_pic_set: Option<ShortTermReferencePictureSet>,
  /// Number of bits used to encode `short_term_ref_pic_set`.
  ///
  /// `Some` when `short_term_ref_pic_set_sps_flag == Some(true)` for non-IDR slices.
  pub short_term_ref_pic_set_size: Option<u16>,
  /// `Some` for non-IDR slices, `None` for IDR slices.
  pub slice_pic_order_cnt_lsb: Option<u16>,
  /// `Some` when all of the following holds true:
  /// - `dependent_slice_segment_flag != Some(true)`
  /// - not an IDR slice
  /// - `short_term_ref_pic_set_sps_flag == true`
  /// - `SliceSegmentContext::num_short_term_ref_pic_sets > 1`
  pub short_term_ref_pic_set_idx: Option<u8>,
  /// `CurrRpsIdx`.
  pub curr_rps_idx: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SliceType {
  /// B slice.
  B = 0,
  /// P slice.
  P = 1,
  /// I slice,
  I = 2,
}

impl TryFrom<u8> for SliceType {
  type Error = String;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0 => Ok(Self::B),
      1 => Ok(Self::P),
      2 => Ok(Self::I),
      _ => Err(format!("Invalid value for SliceType: {}", value)),
    }
  }
}

impl SliceSegmentHeader {
  /// `NumDeltaPocs[RefRpsIdx]`.
  ///
  /// Returns `Some` if this slice segment header has a `ShortTermReferencePictureSet`, otherwise `None`.
  pub fn rps_idx_num_delta_pocs(&self) -> Option<u8> {
    if let Some(rps) = self.short_term_ref_pic_set.as_ref() {
      Some(rps.num_positive_pics() + rps.num_negative_pics())
    }
    else {
      None
    }
  }

  /// Reads from _RBSP(Raw Byte Sequence Payload)_.
  ///
  /// ## Remarks
  /// Currently does *NOT* consume the whole bytes for the slice segment header.
  pub fn from_rbsp_reader<R: Read>(
    reader: &mut R,
    nal_unit_type: NaluType,
    slice_segment_context: SliceSegmentContext,
  ) -> Result<Self, io::Error> {
    let mut bit_reader = BitReader::endian(reader, BigEndian);

    let first_slice_segment_in_pic_flag = bit_reader.read_bit()?;

    // Coded slice segment of a BLA, IDR, or CRA picture, or Reserved IRAP VCL NAL unit types ?
    let no_output_of_prior_pics_flag = if nal_unit_type  >=  NaluType::BlaWLp  &&  nal_unit_type  <=  NaluType::RsvIrapVcl23 {
      Some(bit_reader.read_bit()?)
    } else {
      None
    };

    let slice_pic_parameter_set_id = read_exp_golomb_ue(&mut bit_reader)? as u8;

    let (dependent_slice_segment_flag, slice_segment_address) = if !first_slice_segment_in_pic_flag {
      let dependent_slice_segment_flag = if slice_segment_context.dependent_slice_segments_enabled_flag {
        Some(bit_reader.read_bit()?)
      }
      else {
        None
      };

      // Length is Ceil(Log2(PicSizeInCtbsY)) bits.
      let slice_segment_address = {
        // TODO: Precalculate these values.
        let min_cb_log2_size_y = slice_segment_context.log2_min_luma_coding_block_size_minus3 + 3;
        let ctb_log2_size_y = min_cb_log2_size_y + slice_segment_context.log2_diff_max_min_luma_coding_block_size;
        let ctb_size_y = 1 << ctb_log2_size_y;
        let pic_width_in_ctbs_y = (slice_segment_context.pic_width_in_luma_samples + ctb_size_y - 1) / ctb_size_y;
        let pic_height_in_ctbs_y = (slice_segment_context.pic_height_in_luma_samples + ctb_size_y - 1) / ctb_size_y;
        let pic_size_in_ctbs_y = pic_width_in_ctbs_y * pic_height_in_ctbs_y;

        let length_in_bits = (pic_size_in_ctbs_y as f64).log2().ceil() as u32;
        bit_reader.read::<u32>(length_in_bits)?
      };

      (dependent_slice_segment_flag, Some(slice_segment_address))
    }
    else {
      (None, None)
    };

    let mut slice_pic_order_cnt_lsb: Option<u16> = None;
    let mut short_term_ref_pic_set_sps_flag: Option<bool> = None;
    let mut short_term_ref_pic_set: Option<ShortTermReferencePictureSet> = None;
    let mut short_term_ref_pic_set_size: Option<u16> = None;
    let mut short_term_ref_pic_set_idx: Option<u8> = None;
    let mut curr_rps_idx: u8 = 0;

    if !dependent_slice_segment_flag.unwrap_or(false) {
      for _ in 0..slice_segment_context.num_extra_slice_header_bits {
        // slice_reserved_flag[_]
        bit_reader.read_bit()?;
      }

      let slice_type: SliceType =
        (read_exp_golomb_ue(&mut bit_reader)? as u8)
          .try_into()
          .unwrap();

      let pic_output_flag = if slice_segment_context.output_flag_present_flag {
        Some(bit_reader.read_bit()?)
      }
      else {
        None
      };
      let colour_plane_id = if slice_segment_context.separate_colour_plane_flag {
        Some(bit_reader.read::<u8>(2)?)
      }
      else {
        None
      };

      // 7.4.7.1  General slice segment header semantics:
      // > The variable CurrRpsIdx is derived as follows:
      // > - If short_term_ref_pic_set_sps_flag is equal to 1, CurrRpsIdx is set equal to short_term_ref_pic_set_idx.
      // > - Otherwise, CurrRpsIdx is set equal to num_short_term_ref_pic_sets.

      // Not an IDR slice ?
      if !nal_unit_type.is_idr() {
        // log2_max_pic_order_cnt_lsb_minus4 + 4  bits.
        slice_pic_order_cnt_lsb = Some(bit_reader.read((slice_segment_context.log2_max_pic_order_cnt_lsb_minus4 + 4) as u32)?);
        let short_term_ref_pic_set_sps_flag = *short_term_ref_pic_set_sps_flag.insert(bit_reader.read_bit()?);

        if !short_term_ref_pic_set_sps_flag {
          curr_rps_idx = slice_segment_context.num_short_term_ref_pic_sets;

          let mut bit_count: u32 = 0;
          short_term_ref_pic_set = Some(ShortTermReferencePictureSet::from_bit_reader(
            &mut bit_reader,
            slice_segment_context.num_short_term_ref_pic_sets as usize,
            slice_segment_context.num_short_term_ref_pic_sets as usize,
            &mut bit_count,
          )?);

          short_term_ref_pic_set_size = Some(bit_count as u16);
        }
        else if slice_segment_context.num_short_term_ref_pic_sets > 1 {
          // Ceil(Log2(num_short_term_ref_pic_sets)) bits.
          let length_in_bits = (slice_segment_context.num_short_term_ref_pic_sets as f64).log2().ceil() as u32;
          let value = bit_reader.read::<u8>(length_in_bits)?;
          short_term_ref_pic_set_idx = Some(value);
          curr_rps_idx = value;
        }
      }
    }

    Ok(Self {
      nal_unit_type,
      first_slice_segment_in_pic_flag,
      no_output_of_prior_pics_flag,
      slice_pic_parameter_set_id,
      dependent_slice_segment_flag,
      slice_segment_address,
      short_term_ref_pic_set_sps_flag,
      short_term_ref_pic_set,
      short_term_ref_pic_set_size,
      slice_pic_order_cnt_lsb,
      short_term_ref_pic_set_idx,
      curr_rps_idx,
    })
  }
}

impl SliceSegmentLayer {
  /// Reads from _RBSP(Raw Byte Sequence Payload)_.
  ///
  /// Reads exactly `value_length` bytes.
  pub fn from_rbsp_reader<R: Read>(
    reader: &mut R,
    value_length: usize,
    nal_unit_type: NaluType,
    slice_segment_context: SliceSegmentContext,
  ) -> Result<Self, io::Error> {
    // Consume `value_length` bytes here, as `SliceSegmentHeader::from_reader` currently does not consume the whole bytes for the slice segment header.
    let bytes = {
      let mut bytes = vec![0; value_length];
      reader.read_exact(&mut bytes)?;
      bytes
    };

    let mut reader = io::Cursor::new(bytes);

    let header = SliceSegmentHeader::from_rbsp_reader(&mut reader, nal_unit_type, slice_segment_context)?;
    Ok(Self { header })
  }
}
