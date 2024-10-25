use std::io::{self, Read};

use bitstream_io::BitRead as _;
use bitstream_io::{BigEndian, BitReader};

/// See _7.3.3 Profile, tier and level syntax_ in the spec.
#[derive(Debug, Clone, Copy)]
pub struct ProfileTierLevel {
    pub general: ProfileTierLevelCommon,
    pub sub_layers: [Option<ProfileTierLevelCommon>; 6],
}

#[derive(Debug, Clone, Copy)]
pub struct ProfileTierLevelCommon {
    /// `general_profile_space`: Specifies the context for the interpretation of `general_profile_idc` and `general_profile_compatibility_flags`. Shall be equal to 0.
    pub profile_space: u8,
    /// `general_tier_flag`: Indicates a profile to which the CVS conforms as specified in _Annex A_.
    pub tier_flag: bool,
    pub profile_idc: u8,
    pub profile_compatibility_flags: [bool; 32],
    pub progressive_source_flag: bool,
    pub interlaced_source_flag: bool,
    pub non_packed_constraint_flag: bool,
    pub frame_only_constraint_flag: bool,
    pub level_idc: Option<u8>,
}

#[derive(Debug, Clone, Copy)]
pub struct SubLayerOrderingInfo {
    pub max_dec_pic_buffering_minus1: [u8; 7],
    pub max_num_reorder_pics: [u8; 7],
    pub max_latency_increase_plus1: [u32; 7],
}

impl ProfileTierLevel {
    /// See `profile_tier_level()` in _7.3.3 Profile, tier and level syntax_.
    pub fn from_reader<R: Read>(
        reader: &mut R,
        profile_present_flag: bool,
        max_num_sub_layers_minus1: u8,
    ) -> Result<Self, io::Error> {
        let mut bit_reader = BitReader::endian(reader, BigEndian);

        // VV Formally `if (profile_present_flag) {`.

        let general_profile_space = bit_reader.read(2)?;
        let general_tier_flag = bit_reader.read_bit()?;
        let general_profile_idc = bit_reader.read(5)?;
        let general_profile_compatibility_flags = {
            let mut flags = [false; 32];
            for flag in flags.iter_mut() {
                *flag = bit_reader.read_bit()?;
            }
            flags
        };
        let general_progressive_source_flag = bit_reader.read_bit()?;
        let general_interlaced_source_flag = bit_reader.read_bit()?;
        let general_non_packed_constraint_flag = bit_reader.read_bit()?;
        let general_frame_only_constraint_flag = bit_reader.read_bit()?;

        let if_profile = |idc: u8| -> bool {
            general_profile_idc == idc || general_profile_compatibility_flags[idc as usize]
        };

        if if_profile(4)
            || if_profile(5)
            || if_profile(6)
            || if_profile(7)
            || if_profile(8)
            || if_profile(9)
            || if_profile(10)
            || if_profile(11)
        {
            let general_max_12bit_constraint_flag = bit_reader.read_bit()?;
            let general_max_10bit_constraint_flag = bit_reader.read_bit()?;
            let general_max_8bit_constraint_flag = bit_reader.read_bit()?;
            let general_max_422chroma_constraint_flag = bit_reader.read_bit()?;
            let general_max_420chroma_constraint_flag = bit_reader.read_bit()?;
            let general_max_monochrome_constraint_flag = bit_reader.read_bit()?;
            let general_intra_constraint_flag = bit_reader.read_bit()?;
            let general_one_picture_only_constraint_flag = bit_reader.read_bit()?;
            let general_lower_bit_rate_constraint_flag = bit_reader.read_bit()?;

            if if_profile(5) || if_profile(9) || if_profile(10) || if_profile(11) {
                let general_max_14bit_constraint_flag = bit_reader.read_bit()?;
                // `vps_reserved_zero_33bits`: 33 bits
                bit_reader.read::<u32>(32)?;
                bit_reader.read_bit()?;
            } else {
                // `vps_reserved_zero_34bits`: 34 bits
                bit_reader.read::<u32>(32)?;
                bit_reader.read::<u32>(2)?;
            }
        } else if if_profile(2) {
            // `general_reserved_zero_7bits`: 7 bits
            bit_reader.read::<u32>(7)?;
            let general_one_picture_only_constraint_flag = bit_reader.read_bit()?;
            // `general_reserved_zero_35bits`: 35 bits
            bit_reader.read::<u32>(32)?;
            bit_reader.read::<u32>(3)?;
        } else {
            // `general_reserved_zero_43bits`: 43 bits
            bit_reader.read::<u32>(32)?;
            bit_reader.read::<u32>(11)?;
        }

        // > The number of bits in this syntax structure is not affected by this condition.
        if if_profile(1)
            || if_profile(2)
            || if_profile(3)
            || if_profile(4)
            || if_profile(5)
            || if_profile(9)
            || if_profile(11)
        {
            let general_inbld_flag = bit_reader.read_bit()?;
        } else {
            // `general_reserved_zero_bit`: 1 bit
            bit_reader.read_bit()?;
        }

        // ^^ Formally `if (profile_present_flag) {`.

        let general_level_idc = bit_reader.read::<u8>(8)?;

        let general = ProfileTierLevelCommon {
            profile_space: general_profile_space,
            tier_flag: general_tier_flag,
            profile_idc: general_profile_idc,
            profile_compatibility_flags: general_profile_compatibility_flags,
            progressive_source_flag: general_progressive_source_flag,
            interlaced_source_flag: general_interlaced_source_flag,
            non_packed_constraint_flag: general_non_packed_constraint_flag,
            frame_only_constraint_flag: general_frame_only_constraint_flag,
            level_idc: Some(general_level_idc),
        };

        let mut sub_layer_profile_present_flags = [false; 7];
        let mut sub_layer_level_present_flags = [false; 7];
        for i in 0..max_num_sub_layers_minus1 {
            sub_layer_profile_present_flags[i as usize] = bit_reader.read_bit()?;
            sub_layer_level_present_flags[i as usize] = bit_reader.read_bit()?;
        }
        if max_num_sub_layers_minus1 > 0 {
            for _ in max_num_sub_layers_minus1..8 {
                // `reserved_zero_2bits`: 2 bits
                bit_reader.read::<u8>(2)?;
            }
        }

        let mut sub_layers = [None; 6];
        for i in 0..max_num_sub_layers_minus1 {
            sub_layers[i as usize] = if sub_layer_profile_present_flags[i as usize] {
                let sub_layer_profile_space: u8 = bit_reader.read(2)?;
                let sub_layer_tier_flag: bool = bit_reader.read_bit()?;
                let sub_layer_profile_idc: u8 = bit_reader.read(5)?;
                let sub_layer_profile_compatibility_flags: [bool; 32] = {
                    let mut flags = [false; 32];
                    for flag in flags.iter_mut() {
                        *flag = bit_reader.read_bit()?;
                    }
                    flags
                };
                let sub_layer_progressive_source_flag: bool = bit_reader.read_bit()?;
                let sub_layer_interlaced_source_flag: bool = bit_reader.read_bit()?;
                let sub_layer_non_packed_constraint_flag: bool = bit_reader.read_bit()?;
                let sub_layer_frame_only_constraint_flag: bool = bit_reader.read_bit()?;

                let if_profile = |idc: u8| -> bool {
                    sub_layer_profile_idc == idc
                        || sub_layer_profile_compatibility_flags[idc as usize]
                };

                // > The number of bits in this syntax structure is not affected by this condition
                if if_profile(4)
                    || if_profile(5)
                    || if_profile(6)
                    || if_profile(7)
                    || if_profile(8)
                    || if_profile(9)
                    || if_profile(10)
                    || if_profile(11)
                {
                    let sub_layer_max_12bit_constraint_flag = bit_reader.read_bit()?;
                    let sub_layer_max_10bit_constraint_flag = bit_reader.read_bit()?;
                    let sub_layer_max_8bit_constraint_flag = bit_reader.read_bit()?;
                    let sub_layer_max_422chroma_constraint_flag = bit_reader.read_bit()?;
                    let sub_layer_max_420chroma_constraint_flag = bit_reader.read_bit()?;
                    let sub_layer_max_monochrome_constraint_flag = bit_reader.read_bit()?;
                    let sub_layer_intra_constraint_flag = bit_reader.read_bit()?;
                    let sub_layer_one_picture_only_constraint_flag = bit_reader.read_bit()?;
                    let sub_layer_lower_bit_rate_constraint_flag = bit_reader.read_bit()?;

                    if if_profile(5) || if_profile(9) || if_profile(10) || if_profile(11) {
                        let sub_layer_max_14bit_constraint_flag = bit_reader.read_bit()?;
                        // `sub_layer_reserved_zero_33bits`: 33 bits
                        bit_reader.read::<u32>(32)?;
                        bit_reader.read_bit()?;
                    } else {
                        // `sub_layer_reserved_zero_34bits`: 34 bits
                        bit_reader.read::<u32>(32)?;
                        bit_reader.read::<u32>(2)?;
                    }
                } else if if_profile(2) {
                    // `sub_layer_reserved_zero_7bits`: 7 bits
                    bit_reader.read::<u32>(7)?;
                    let sub_layer_one_picture_only_constraint_flag = bit_reader.read_bit()?;
                    // `general_reserved_zero_35bits`: 35 bits
                    bit_reader.read::<u32>(32)?;
                    bit_reader.read::<u32>(3)?;
                } else {
                    // `sub_layer_reserved_zero_43bits`: 43 bits
                    bit_reader.read::<u32>(32)?;
                    bit_reader.read::<u32>(11)?;
                }

                // > The number of bits in this syntax structure is not affected by this condition.
                if if_profile(1)
                    || if_profile(2)
                    || if_profile(3)
                    || if_profile(4)
                    || if_profile(5)
                    || if_profile(9)
                    || if_profile(11)
                {
                    let sub_layer_inbld_flag = bit_reader.read_bit()?;
                } else {
                    // `sub_layer_reserved_zero_bit`: 1 bit
                    bit_reader.read_bit()?;
                }

                let sub_layer_level_idc = if sub_layer_level_present_flags[i as usize] {
                    Some(bit_reader.read::<u8>(8)?)
                } else {
                    None
                };

                Some(ProfileTierLevelCommon {
                    profile_space: sub_layer_profile_space,
                    tier_flag: sub_layer_tier_flag,
                    profile_idc: sub_layer_profile_idc,
                    profile_compatibility_flags: sub_layer_profile_compatibility_flags,
                    progressive_source_flag: sub_layer_progressive_source_flag,
                    interlaced_source_flag: sub_layer_interlaced_source_flag,
                    non_packed_constraint_flag: sub_layer_non_packed_constraint_flag,
                    frame_only_constraint_flag: sub_layer_frame_only_constraint_flag,
                    level_idc: sub_layer_level_idc,
                })
            } else {
                None
            };
        }

        // 1: Main
        // 2: Main 10, Main 10 Still Picture
        // 3: Main Still Picture
        // 4: Format Range Extensions
        // 5: High Throughput
        // 9: Screen Content Coding Extensions
        // 11: High Throughput Screen Content Coding Extensions

        Ok(Self {
            general,
            sub_layers,
        })
    }
}
