
pub mod bytestream;
pub mod nalu;
pub mod poc;
pub mod ptl;
pub mod rps;
pub mod vps;
pub mod sps;
pub mod pps;
pub mod slice;
pub mod nalu_ref;

#[cfg(test)]
mod tests {
  use super::*;

  use std::io;

  use nalu::NaluValueContext;
  use slice::SliceSegmentContext;

  use bytestream::{LengthPrefixedByteStreamNaluReader, LengthPrefixedByteStreamNaluRefReader};

  #[test]
  fn read_nalus() {
    let bytes = include_bytes!("../../test_files/sample_1_0.bin");

    {
      let nalu_value_context = make_nalu_value_context();

      let reader = io::Cursor::new(bytes);
      let mut reader = LengthPrefixedByteStreamNaluReader::with_length_size_minus_one(3, reader, nalu_value_context);

      let nalus = reader.read_contents_until_eof()
        .unwrap();
      for nalu in &nalus {
        println!("{:?}", nalu);
      }
    }
  }

  #[test]
  fn read_nalu_refs() {
    let bytes = include_bytes!("../../test_files/sample_1_0.bin");

    {
      let nalu_value_context = make_nalu_value_context();

      let reader = io::Cursor::new(bytes);
      let mut reader = LengthPrefixedByteStreamNaluRefReader::with_length_size_minus_one(3, reader, nalu_value_context);

      let nalu_refs = reader.read_contents_until_eof()
        .unwrap();
      println!("{} NaluRefs:", nalu_refs.len());
      for nalu_ref in &nalu_refs {
        println!("{:?}", nalu_ref);
      }
    }
  }

  #[cfg(feature = "mp4")]
  #[test]
  fn read_nalu_refs_in_mp4() {
    const TRACK_ID: u32 = 1;

    let path = r"F:\DCIM\100GOPRO\GX010002.MP4";

    let mut mp4_reader = {
      let file = std::fs::File::open(path)
      .unwrap();
      let size = file.metadata()
        .unwrap()
        .len();
      let reader = io::BufReader::new(file);

      mp4::Mp4Reader::read_header(reader, size)
        .unwrap()
    };

    let track = mp4_reader.tracks().get(&TRACK_ID)
      .unwrap();

    let mp4_sample = mp4_reader.read_sample(TRACK_ID, 1)
      .unwrap()
      .unwrap();

    println!("start_time: {}, duration: {}", mp4_sample.start_time, mp4_sample.duration);

    {
      let nalu_value_context = NaluValueContext {
        slice_segment_context: Some(SliceSegmentContext {
          // TODO: Use the actual value from the PPS in the MP4.
          dependent_slice_segments_enabled_flag: true,
          pic_width_in_luma_samples: 3840,
          pic_height_in_luma_samples: 2160,
          log2_min_luma_coding_block_size_minus3: 0,
          log2_diff_max_min_luma_coding_block_size: 3,
          num_extra_slice_header_bits: 0,
          output_flag_present_flag: false,
          separate_colour_plane_flag: false,
          log2_max_pic_order_cnt_lsb_minus4: 4,
          num_short_term_ref_pic_sets: 3,
        }),
      };

      let reader = io::Cursor::new(mp4_sample.bytes);
      let mut reader = LengthPrefixedByteStreamNaluRefReader::with_length_size_minus_one(3, reader, nalu_value_context);

      let nalu_refs = reader.read_contents_until_eof()
        .unwrap();
      println!("{} NaluRefs:", nalu_refs.len());
      for nalu_ref in &nalu_refs {
        println!("{:?}", nalu_ref);
      }
    }
  }

  fn make_nalu_value_context() -> NaluValueContext {
    NaluValueContext {
      slice_segment_context: Some(SliceSegmentContext {
        dependent_slice_segments_enabled_flag: true,
        pic_width_in_luma_samples: 3840,
        pic_height_in_luma_samples: 2160,
        log2_min_luma_coding_block_size_minus3: 0,
        log2_diff_max_min_luma_coding_block_size: 3,
        num_extra_slice_header_bits: 0,
        output_flag_present_flag: false,
        separate_colour_plane_flag: false,
        log2_max_pic_order_cnt_lsb_minus4: 4,
        num_short_term_ref_pic_sets: 3,
      }),
    }
  }
}
