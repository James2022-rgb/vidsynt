
use std::io::{self, Read, Write};

use bitstream_io::{BigEndian, BitReader, BitWriter};
use bitstream_io::{BitRead as _, BitWrite as _};

use crate::h265::vps::VideoParameterSet;
use crate::h265::sps::SequenceParameterSet;
use crate::h265::pps::PictureParameterSet;
use crate::h265::slice::{SliceSegmentContext, SliceSegmentLayer};

#[derive(Debug, Clone)]
pub struct Nalu {
  pub header: NaluHeader,
  pub value: NaluValue,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NaluHeader {
  /// Specifies the type of RBSP data structure contained in the NAL unit as specified in _Table 7-1_ in the spec.
  pub nal_unit_type: NaluType,
  /// Specifies the identifier of the layer to which a VCL NAL unit belongs or the identifier of a layer to which a non-VCL NAL unit applies.
  pub nuh_layer_id: u8,
  /// Specifies a temporal identifier for the NAL unit.
  pub nuh_temporal_id_plus1: u8,
}

/// See _7.4.2.2 NAL unit header semantics_ in the spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum NaluType {
  /// `TRAIL_N`. _Coded slice segment of a non-TSA, non-STSA trailing picture_.
  ///
  /// `N` signifies a non-reference picture.
  TrailN = 0,
  /// `TRAIL_N`. _Coded slice segment of a non-TSA, non-STSA trailing picture_.
  ///
  /// `R` signifies a reference picture.
  TrailR = 1,
  /// `RADL_N`. _Coded slice segment of a RADL picture_.
  ///
  /// `N` signifies a non-reference picture.
  ///
  /// A RADL(Random Access Decodable Leading) picture.
  RadlN = 6,
  /// `RADL_R`. _Coded slice segment of a RADL picture_.
  ///
  /// `R` signifies a reference picture.
  RadlR = 7,
  /// `RASL_N`. _Coded slice segment of a RASL picture_.
  ///
  /// `N` signifies a non-reference picture.
  ///
  /// A RASL(Random Access Skipped Leading) picture.
  RaslN = 8,
  /// `RASL_R`. _Coded slice segment of a RASL picture_.
  ///
  /// `R` signifies a reference picture.
  RaslR = 9,
  /// Reserved non-IRAP SLNR VCL NAL unit type.
  RsvVclN10 = 10,
  /// Reserved non-IRAP SLNR VCL NAL unit type.
  RsvVclN12 = 12,
  /// Reserved non-IRAP SLNR VCL NAL unit type.
  RsvVclN14 = 14,
  /// `BLA_W_LP`. _Coded slice segment of a BLA picture_.
  ///
  /// `W_LP` signifies both RADL and RADL LPs may be present.
  ///
  /// A BLA(Broken Link Access) picture is an _IRAP picture_ for which each _VCL NAL unit_ has `nal_unit_type` equal to `BLA_W_LP`, `BLA_W_RADL`, or `BLA_N_LP`.
  /// A BLA access unit is an _access unit_ in which the _coded picture_ with `nuh_layer_id` equal to 0 is a BLA picture.
  BlaWLp = 16,
  /// `BLA_W_RADL`. _Coded slice segment of a BLA picture_.
  ///
  /// `W_RADL` signifies only RADL may be present.
  BlaWRadl = 17,
  /// `BLA_N_LP`. _Coded slice segment of a BLA picture_.
  ///
  /// `N_LP` signifies an LP is not present.
  BlaNLp = 18,
  /// `IDR_W_RADL`. _Coded slice segment of an IDR picture_.
  ///
  /// `W_RADL` signifies only RADL may be present.
  ///
  /// An IDR(Instantaneous Decoding Refresh) picture is an _IRAP picture_ for which each _VCL NAL unit_ has `nal_unit_type` equal to `IDR_W_RADL` or `IDR_N_LP`.
  /// An IDR access unit is an _access unit_ in which the _coded picture_ with `nuh_layer_id` equal to 0 is an IDR picture.
  IdrWRadl = 19,
  /// `IDR_N_LP`. _Coded slice segment of an IDR picture_.
  ///
  /// `N_LP` signifies an LP is not present.
  IdrNLp = 20,
  /// `CRA_NUT`. _Coded slice segment of a CRA picture_.
  CraNut = 21,
  /// Reserved IRAP VCL unit type.
  RsvIrapVcl22 = 22,
  /// Reserved IRAP VCL unit type.
  RsvIrapVcl23 = 23,
  /// `VPS_NUT`. _Video parameter set_.
  VpsNut = 32,
  /// `SPS_NUT`. _Sequence parameter set_.
  SpsNut = 33,
  /// `PPS_NUT`. _Picture parameter set_.
  PpsNut = 34,
  /// `AUD_NUT`. _Access unit delimiter_.
  AudNut = 35,
}

#[derive(Debug, Clone)]
pub enum NaluValue {
  CodedSliceSegment(SliceSegmentLayer),
  AudNut(AccessUnitDelimiter),
  VpsNut(VideoParameterSet),
  SpsNut(SequenceParameterSet),
  PpsNut(PictureParameterSet),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NaluValueContext {
  pub slice_segment_context: Option<SliceSegmentContext>,
}

/// See `access_unit_delimiter_rbsp()` in _7.3.2.5 Access unit delimiter RBSP syntax_ in the spec.
#[derive(Debug, Clone, Copy)]
pub struct AccessUnitDelimiter {
  pub pic_type: PicType,
}

/// Indicates the `slice_type` values that may be present in all slices of the _coded pictures_ in the access unit contained in the _access unit delimiter_ NAL unit.
///
/// See _7.4.3.5 Access unit delimiter RBSP semantics_ in the spec.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum PicType {
  /// I(intra) slices.
  I = 0,
  /// P(predictive) and I(intra) slices.
  PI = 1,
  /// B(bi-predictive), P(predictive), and I(intra) slices.
  BPI = 2,
}

impl NaluType {
  /// IRAP(Intra Random Access Point) type ?
  pub fn is_irap(&self) -> bool {
    matches!(self, Self::BlaWLp | Self::BlaWRadl | Self::BlaNLp | Self::IdrWRadl | Self::IdrNLp | Self::CraNut | Self::RsvIrapVcl22 | Self::RsvIrapVcl23)
  }

  /// RADL(Random Access Decodable Leading) type ?
  pub fn is_radl(&self) -> bool {
    matches!(self, Self::RadlN | Self::RadlR)
  }

  /// RASL(Random Access Skipped Leading) type ?
  pub fn is_rasl(&self) -> bool {
    matches!(self, Self::RaslN | Self::RaslR)
  }

  /// BLA(Broken Link Access) type ?
  pub fn is_bla(&self) -> bool {
    matches!(self, Self::BlaWLp | Self::BlaWRadl | Self::BlaNLp)
  }

  /// IDR(Instantaneous Decoding Refresh) type ?
  pub fn is_idr(&self) -> bool {
    matches!(self, Self::IdrWRadl | Self::IdrNLp)
  }

  pub fn is_reference(&self) -> bool {
    if (*self as u8) < 16 {
      (*self as u8) & 1 == 1
    }
    else {
      true
    }
  }

  pub fn is_coded_slice_segment(&self) -> bool {
    matches!(
      self,
      Self::TrailN | Self::TrailR | Self::RadlN | Self::RadlR | Self::RaslN | Self::RaslR | Self::BlaWLp | Self::BlaWRadl | Self::BlaNLp | Self::IdrWRadl | Self::IdrNLp | Self::CraNut | Self::RsvIrapVcl22 | Self::RsvIrapVcl23
    )
  }
}

impl Nalu {
  pub fn from_bytes(bytes: &[u8], nalue_value_context: NaluValueContext) -> Result<Self, io::Error> {
    let mut reader = io::Cursor::new(bytes);
    Self::from_reader(&mut reader, bytes.len(), nalue_value_context)
  }

  pub fn from_reader<R: Read>(reader: &mut R, length: usize, nalue_value_context: NaluValueContext) -> Result<Self, io::Error> {
    let header = NaluHeader::from_reader(reader)?;

    let value_length = length - 2;
    let value = NaluValue::from_ebsp_reader(reader, header, value_length, nalue_value_context)?;

    Ok(Self {
      header,
      value,
    })
  }
}

impl NaluHeader {
  /// Reads exactly 2 bytes.
  pub fn from_reader<R: Read>(reader: &mut R) -> Result<Self, io::Error> {
    let mut bit_reader = BitReader::endian(reader, BigEndian);

    bit_reader.read_bit()?; // `forbidden_zero_bit`
    let nal_unit_type: u8 = bit_reader.read(6)?;
    let nuh_layer_id: u8 = bit_reader.read(6)?;
    let nuh_temporal_id_plus1: u8 = bit_reader.read(3)?;

    let nal_unit_type: NaluType = nal_unit_type.try_into()
      .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;

    Ok(Self {
      nal_unit_type,
      nuh_layer_id,
      nuh_temporal_id_plus1,
    })
  }

  pub fn to_writer<W: Write>(&self, writer: &mut W) -> Result<(), io::Error> {
    let mut bit_writer = BitWriter::endian(writer, BigEndian);

    bit_writer.write_bit(false)?; // `forbidden_zero_bit`
    bit_writer.write(6, self.nal_unit_type as u8)?;
    bit_writer.write(6, self.nuh_layer_id)?;
    bit_writer.write(3, self.nuh_temporal_id_plus1)?;
    Ok(())
  }
}

impl TryFrom<u8> for NaluType {
  type Error = String;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0 => Ok(Self::TrailN),
      1 => Ok(Self::TrailR),
      8 => Ok(Self::RaslN),
      9 => Ok(Self::RaslR),
      16 => Ok(Self::BlaWLp),
      17 => Ok(Self::BlaWRadl),
      18 => Ok(Self::BlaNLp),
      19 => Ok(Self::IdrWRadl),
      20 => Ok(Self::IdrNLp),
      21 => Ok(Self::CraNut),
      22 => Ok(Self::RsvIrapVcl22),
      23 => Ok(Self::RsvIrapVcl23),
      32 => Ok(Self::VpsNut),
      33 => Ok(Self::SpsNut),
      34 => Ok(Self::PpsNut),
      35 => Ok(Self::AudNut),
      _ => Err(format!("Unknown NAL unit type: {}", value)),
    }
  }
}

impl NaluValue {
  pub fn as_coded_slice_segment(&self) -> Option<&SliceSegmentLayer> {
    match self {
      Self::CodedSliceSegment(value) => Some(value),
      _ => None,
    }
  }

  pub fn as_vps_nut(&self) -> Option<&VideoParameterSet> {
    match self {
      Self::VpsNut(value) => Some(value),
      _ => None,
    }
  }

  pub fn as_sps_nut(&self) -> Option<&SequenceParameterSet> {
    match self {
      Self::SpsNut(value) => Some(value),
      _ => None,
    }
  }

  pub fn as_pps_nut(&self) -> Option<&PictureParameterSet> {
    match self {
      Self::PpsNut(value) => Some(value),
      _ => None,
    }
  }

  /// Reads from _EBSP(Encapsulated Byte Sequence Payload)_.
  ///
  /// Reads exactly `value_length` bytes.
  pub fn from_ebsp_reader<R: Read>(
    reader: &mut R,
    nalu_header: NaluHeader,
    value_length: usize,
    nalu_value_context: NaluValueContext,
  ) -> Result<Self, io::Error> {
    // EBSP(Encapsulated Byte Sequence Payload).
    let ebsp = {
      let mut ebsp: Vec<u8> = Vec::with_capacity(value_length);
      reader.read_exact(
        unsafe { std::slice::from_raw_parts_mut(ebsp.as_mut_ptr(), value_length) }
      )?;
      // SAFETY:
      // 1. `ebsp` is initialized with `value_length` capacity.
      // 2. its values are initialized by `read_exact()`.
      unsafe {
        ebsp.set_len(value_length);
      }
      ebsp
    };

    // RBSP(Raw Byte Sequence Payload) i.e. EBSP without emulation prevention bytes.
    let rbsp = {
      let mut rbsp: Vec<u8> = Vec::with_capacity(value_length);

      let mut i = 0;
      while i < ebsp.len() {
        if i + 2 < ebsp.len() && ebsp[i] == 0 && ebsp[i + 1] == 0 && ebsp[i + 2] == 3 {
          rbsp.push(0);
          rbsp.push(0);
          i += 3;
        }
        else {
          rbsp.push(ebsp[i]);
          i += 1;
        }
      }
      rbsp
    };

    let rbsp_length = rbsp.len();

    let mut rbsp_reader = io::Cursor::new(rbsp);
    let rbsp_reader = &mut rbsp_reader;

    match nalu_header.nal_unit_type {
      NaluType::TrailR | NaluType::TrailN | NaluType::IdrWRadl | NaluType::IdrNLp | NaluType::CraNut | NaluType::RaslN | NaluType::RaslR => {
        let value = SliceSegmentLayer::from_rbsp_reader(
          rbsp_reader,
          rbsp_length,
          nalu_header.nal_unit_type,
          nalu_value_context.slice_segment_context
            .expect("SliceSegmentContext is required for coded slice segments"),
        )?;

        Ok(Self::CodedSliceSegment(value))
      },
      NaluType::VpsNut => {
        let value = VideoParameterSet::from_rbsp_reader(rbsp_reader)?;
        Ok(Self::VpsNut(value))
      },
      NaluType::SpsNut => {
        let value = SequenceParameterSet::from_rbsp_reader(rbsp_reader)?;
        Ok(Self::SpsNut(value))
      },
      NaluType::PpsNut => {
        let value = PictureParameterSet::from_rbsp_reader(rbsp_reader, nalu_header.nuh_temporal_id_plus1)?;
        Ok(Self::PpsNut(value))
      },
      NaluType::AudNut => {
        let value = AccessUnitDelimiter::from_rbso_reader(rbsp_reader)?;
        Ok(Self::AudNut(value))
      },
      nal_unit_type => panic!("Unsupported NAL unit type: {:?}", nal_unit_type),
    }
  }
}

impl From<u8> for PicType {
  fn from(value: u8) -> Self {
    match value {
      0 => Self::I,
      1 => Self::PI,
      2 => Self::BPI,
      _ => panic!("Unknown pic_type: {}", value),
    }
  }
}

impl AccessUnitDelimiter {
  /// Reads exactly 1 byte.
  pub fn from_rbso_reader<R: Read>(reader: &mut R) -> Result<Self, io::Error> {
    let mut bit_reader = BitReader::endian(reader, BigEndian);

    let pic_type: u8 = bit_reader.read(3)?;

    // `rbsp_trailing_bits()`

    Ok(Self {
      pic_type: pic_type.into(),
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_vps_file() {
    let bytes = include_bytes!("../../../test_files/hvc_array_32.bin");
    let mut reader = io::Cursor::new(bytes);

    let nalu = Nalu::from_reader(&mut reader, bytes.len(), Default::default())
      .unwrap();
    println!("{:?}", nalu);
  }

  #[test]
  fn test_sps_file() {
    let bytes = include_bytes!("../../../test_files/hvc_array_33.bin");
    let mut reader = io::Cursor::new(bytes);

    let nalu = Nalu::from_reader(&mut reader, bytes.len(), Default::default())
      .unwrap();
    println!("{:?}", nalu);
  }

  #[test]
  fn test_pps_file() {
    let bytes = include_bytes!("../../../test_files/hvc_array_34.bin");
    let mut reader = io::Cursor::new(bytes);

    let nalu = Nalu::from_reader(&mut reader, bytes.len(), Default::default())
      .unwrap();
    println!("{:?}", nalu);
  }
}
