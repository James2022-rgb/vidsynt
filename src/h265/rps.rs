use std::io::{self, Read};

use bitstream_io::{BigEndian, BitReader};
use bitstream_io::BitRead as _;

use crate::base::{read_exp_golomb_ue, read_exp_golomb_ue_count_bits};

/// See _7.3.7 Short-term reference picture set syntax_ in the spec.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ShortTermReferencePictureSet {
    /// `Some(true)` specifies that this candidate short-term RPS is predicted from another candidate short-term RPS.
    ///
    /// `None` for the first `ShortTermReferencePictureSet`, `Some` for the rest.
    pub inter_ref_pic_set_prediction_flag: Option<bool>,
    pub value: ShortTermReferencePictureSetValue,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ShortTermReferencePictureSetValue {
    InterRefPicSetPrediction(InterRefPicSetPrediction),
    NonInterRefPicSetPrediction(NonInterRefPicSetPrediction),
}

/// For `inter_ref_pic_set_prediction_flag == true`,
/// i.e. when the current `ShortTermReferencePictureSet` is predicted from another.
/// > the stRpsIdx-th candidate short-term RPS is predicted from another candidate short-term RPS
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InterRefPicSetPrediction {
    /// `Some` for an RPS in a slice header.
    pub delta_idx_minus1: Option<u32>,
    pub delta_rps_sign: u32,
    pub abs_delta_rps_minus1: u16,
    /// `NumDeltaPocs[RefRpsIdx]`.
    pub rps_idx_num_delta_pocs: Option<u8>,
    pub used_by_curr_pic_flag: bool,
    pub use_delta_flag: bool,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NonInterRefPicSetPrediction {
    pub num_negative_pics: u8,
    pub num_positive_pics: u8,
    pub delta_poc_s0_minus1: [u16; 16],
    pub used_by_curr_pic_s0_flag: [bool; 16],
    pub delta_poc_s1_minus1: [u16; 16],
    pub used_by_curr_pic_s1_flag: [bool; 16],
}

impl ShortTermReferencePictureSet {
    pub fn as_inter_ref_pic_set_prediction(&self) -> Option<&InterRefPicSetPrediction> {
        match &self.value {
            ShortTermReferencePictureSetValue::InterRefPicSetPrediction(value) => Some(value),
            ShortTermReferencePictureSetValue::NonInterRefPicSetPrediction(_) => None,
        }
    }

    pub fn as_non_inter_ref_pic_set_prediction(&self) -> Option<&NonInterRefPicSetPrediction> {
        match &self.value {
            ShortTermReferencePictureSetValue::InterRefPicSetPrediction(_) => None,
            ShortTermReferencePictureSetValue::NonInterRefPicSetPrediction(value) => Some(value),
        }
    }

    /// `NumDeltaPocs[RefRpsIdx]`.
    ///
    /// Returns `Some` for an `InterRefPicSetPrediction` signalled in a slice segment header, otherwise `None`.
    pub fn rps_idx_num_delta_pocs(&self) -> Option<u8> {
        match &self.value {
            ShortTermReferencePictureSetValue::InterRefPicSetPrediction(value) => {
                value.rps_idx_num_delta_pocs
            }
            ShortTermReferencePictureSetValue::NonInterRefPicSetPrediction(_) => None,
        }
    }

    /// Calculates the variable `NumDeltaPocs[stRpsIdx]` as defined in _7.4.8 Short-term reference picture set semantics_ of the spec.
    /// ```
    /// NumDeltaPocs[stRpsIdx] = NumNegativePics[stRpsIdx] + NumPositivePics[stRpsIdx]
    /// ```
    pub fn num_delta_pocs(&self) -> u8 {
        self.num_negative_pics() + self.num_positive_pics()
    }

    pub fn num_negative_pics(&self) -> u8 {
        match &self.value {
            ShortTermReferencePictureSetValue::InterRefPicSetPrediction(_) => todo!(),
            ShortTermReferencePictureSetValue::NonInterRefPicSetPrediction(value) => {
                value.num_negative_pics
            }
        }
    }
    pub fn num_positive_pics(&self) -> u8 {
        match &self.value {
            ShortTermReferencePictureSetValue::InterRefPicSetPrediction(_) => todo!(),
            ShortTermReferencePictureSetValue::NonInterRefPicSetPrediction(value) => {
                value.num_positive_pics
            }
        }
    }

    pub fn delta_poc_s0_minus1(&self) -> [u16; 16] {
        match &self.value {
            ShortTermReferencePictureSetValue::InterRefPicSetPrediction(_) => todo!(),
            ShortTermReferencePictureSetValue::NonInterRefPicSetPrediction(value) => {
                value.delta_poc_s0_minus1
            }
        }
    }
    pub fn used_by_curr_pic_s0_flag(&self) -> [bool; 16] {
        match &self.value {
            ShortTermReferencePictureSetValue::InterRefPicSetPrediction(_) => todo!(),
            ShortTermReferencePictureSetValue::NonInterRefPicSetPrediction(value) => {
                value.used_by_curr_pic_s0_flag
            }
        }
    }
    pub fn delta_poc_s1_minus1(&self) -> [u16; 16] {
        match &self.value {
            ShortTermReferencePictureSetValue::InterRefPicSetPrediction(_) => todo!(),
            ShortTermReferencePictureSetValue::NonInterRefPicSetPrediction(value) => {
                value.delta_poc_s1_minus1
            }
        }
    }
    pub fn used_by_curr_pic_s1_flag(&self) -> [bool; 16] {
        match &self.value {
            ShortTermReferencePictureSetValue::InterRefPicSetPrediction(_) => todo!(),
            ShortTermReferencePictureSetValue::NonInterRefPicSetPrediction(value) => {
                value.used_by_curr_pic_s1_flag
            }
        }
    }

    pub fn bitmask_used_by_curr_pic_s0_flag(&self) -> u16 {
        // Convert the `used_by_curr_pic_s0_flag` array to a bitmask.
        self.used_by_curr_pic_s0_flag()
            .iter()
            .enumerate()
            .fold(0, |acc, (i, &flag)| acc | ((flag as u16) << i))
    }
    pub fn bitmask_used_by_curr_pic_s1_flag(&self) -> u16 {
        // Convert the `used_by_curr_pic_s0_flag` array to a bitmask.
        self.used_by_curr_pic_s0_flag()
            .iter()
            .enumerate()
            .fold(0, |acc, (i, &flag)| acc | ((flag as u16) << i))
    }

    /// * `st_rps_index`: `stRpsIdx`; the index of the current `ShortTermReferencePictureSet`.
    pub fn from_bit_reader<R: Read>(
        bit_reader: &mut BitReader<R, BigEndian>,
        st_rps_index: usize,
        num_short_term_ref_pic_sets: usize,
        bit_count: &mut u32,
    ) -> Result<Self, io::Error> {
        Self::from_bit_reader_impl(
            bit_reader,
            st_rps_index,
            num_short_term_ref_pic_sets,
            None,
            bit_count,
        )
    }

    /// * `st_rps_index`: `stRpsIdx`; the index of the current `ShortTermReferencePictureSet`.
    /// * `sps_st_ref_pic_sets`: Required for when `inter_ref_pic_set_prediction_flag == true`.
    /// * `slice_sps_st_ref_pic_sets`: Required for when parsing a slice segment header.
    fn from_bit_reader_impl<R: Read>(
        bit_reader: &mut BitReader<R, BigEndian>,
        st_rps_index: usize,
        num_short_term_ref_pic_sets: usize,
        slice_sps_st_ref_pic_sets: Option<&[ShortTermReferencePictureSet]>,
        bit_count: &mut u32,
    ) -> Result<Self, io::Error> {
        let inter_ref_pic_set_prediction_flag = if st_rps_index != 0 {
            *bit_count += 1;
            Some(bit_reader.read_bit()?)
        } else {
            None
        };

        let value = if inter_ref_pic_set_prediction_flag.unwrap_or(false) {
            // A `st_ref_pic_set()` syntax structure directly signalled in the slice headers of a current picture
            // has an index equal to `num_short_term_ref_pic_sets`.
            let delta_idx_minus1 = if st_rps_index == num_short_term_ref_pic_sets {
                Some(read_exp_golomb_ue_count_bits(bit_reader, bit_count)?)
            } else {
                None
            };

            *bit_count += 1;
            let delta_rps_sign: u32 = if bit_reader.read_bit()? { 1 } else { 0 };
            let abs_delta_rps_minus1: u16 =
                read_exp_golomb_ue_count_bits(bit_reader, bit_count)? as _;

            let rps_idx_num_delta_pocs = if let Some(delta_idx_minus1) = delta_idx_minus1 {
                let slice_sps_st_ref_pic_sets = slice_sps_st_ref_pic_sets.expect(
                    "st_ref_pic_set() in slice header. slice_sps_st_ref_pic_sets must be Some",
                );

                // refRpsIdx = stRpsIdx - (delta_idx_minus1 + 1)
                let ref_rps_idx = st_rps_index - (delta_idx_minus1 as usize + 1);
                let ref_rps = slice_sps_st_ref_pic_sets[ref_rps_idx];
                Some(ref_rps.num_delta_pocs())
            } else {
                None
            };

            todo!("inter_ref_pic_set_prediction_flag == true not supported");

            ShortTermReferencePictureSetValue::InterRefPicSetPrediction(InterRefPicSetPrediction {
                delta_idx_minus1,
                delta_rps_sign,
                abs_delta_rps_minus1,
                rps_idx_num_delta_pocs,
                use_delta_flag: true,
                used_by_curr_pic_flag: false,
            })
        } else {
            let num_negative_pics: u8 = read_exp_golomb_ue_count_bits(bit_reader, bit_count)? as _;
            let num_positive_pics: u8 = read_exp_golomb_ue_count_bits(bit_reader, bit_count)? as _;

            let mut delta_poc_s0_minus1 = [0u16; 16];
            let mut used_by_curr_pic_s0_flag = [false; 16];
            for i in 0..num_negative_pics {
                delta_poc_s0_minus1[i as usize] =
                    read_exp_golomb_ue_count_bits(bit_reader, bit_count)? as _;
                *bit_count += 1;
                used_by_curr_pic_s0_flag[i as usize] = bit_reader.read_bit()?;
            }
            let mut delta_poc_s1_minus1 = [0u16; 16];
            let mut used_by_curr_pic_s1_flag = [false; 16];
            for i in 0..num_positive_pics {
                delta_poc_s1_minus1[i as usize] =
                    read_exp_golomb_ue_count_bits(bit_reader, bit_count)? as _;
                *bit_count += 1;
                used_by_curr_pic_s1_flag[i as usize] = bit_reader.read_bit()?;
            }

            ShortTermReferencePictureSetValue::NonInterRefPicSetPrediction(
                NonInterRefPicSetPrediction {
                    num_negative_pics,
                    num_positive_pics,
                    delta_poc_s0_minus1,
                    used_by_curr_pic_s0_flag,
                    delta_poc_s1_minus1,
                    used_by_curr_pic_s1_flag,
                },
            )
        };

        Ok(Self {
            inter_ref_pic_set_prediction_flag,
            value,
        })
    }
}
