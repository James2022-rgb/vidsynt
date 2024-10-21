
use std::io::Read;

use crate::h265::nalu::{NaluHeader, NaluType};

#[derive(Debug, Clone, Copy)]
pub struct NaluRef {
  /// The offset of the NAL unit header in bytes.
  pub offset: usize,
  pub nal_unit_type: NaluType,
}

impl NaluRef {
  /// Reads a `NaluRef` from a reader.
  ///
  /// Returns the number of bytes consumed and the `NaluRef`.
  pub fn from_reader<R: Read>(reader: &mut R, current_offset: usize) -> Result<(usize, Self), std::io::Error> {
    const HEADER_BYTES: usize = 2;
    let header = NaluHeader::from_reader(reader)?;
    let nalu_ref = Self {
      offset: current_offset,
      nal_unit_type: header.nal_unit_type,
    };

    Ok((HEADER_BYTES, nalu_ref))
  }

  pub fn access_unit_is_irap_picure(access_unit: &[Self]) -> bool {
    assert!(access_unit.len() > 1, "An access unit must contain at least 2 NAL units.");
    // The first NAL unit is assumed to be the AUD.
    assert_eq!(access_unit[0].nal_unit_type, NaluType::AudNut);
    access_unit[1..].iter().all(|nalu_ref| nalu_ref.nal_unit_type.is_irap())
  }

  pub fn access_unit_is_idr_picture(access_unit: &[Self]) -> bool {
    assert!(access_unit.len() > 1, "An access unit must contain at least 2 NAL units.");
    // The first NAL unit is assumed to be the AUD.
    assert_eq!(access_unit[0].nal_unit_type, NaluType::AudNut);
    access_unit[1..].iter().all(|nalu_ref| nalu_ref.nal_unit_type.is_idr())
  }

  pub fn access_unit_is_reference_picture(access_unit: &[Self]) -> bool {
    assert!(access_unit.len() > 1, "An access unit must contain at least 2 NAL units.");
    // The first NAL unit is assumed to be the AUD.
    assert_eq!(access_unit[0].nal_unit_type, NaluType::AudNut);
    access_unit[1..].iter().all(|nalu_ref| nalu_ref.nal_unit_type.is_reference())
  }
}
