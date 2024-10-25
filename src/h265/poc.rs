//! Picture Order Count (`PicOrderCntVal`) computation.
//!
//! Mostly taken from: https://chromium.googlesource.com/chromium/src/+/refs/tags/121.0.6116.2/media/video/h265_poc.cc
//!
//! Picture order counts are used to identify pictures within a CVS(Coded Video Sequence).
//! Each coded picture is associated with a picture order count variable, denoted as `PicOrderCntVal`.

use crate::h265::nalu::NaluType;
use crate::h265::sps::SequenceParameterSet;
use crate::h265::pps::PictureParameterSet;
use crate::h265::slice::SliceSegmentHeader;

#[derive(Debug, Clone)]
pub struct PocComputer {
    is_first_picture: bool,
    ref_pic_order_cnt_msb: i32,
    ref_pic_order_cnt_lsb: i32,
}

impl Default for PocComputer {
    fn default() -> Self {
        Self {
            is_first_picture: true,
            ref_pic_order_cnt_msb: 0,
            ref_pic_order_cnt_lsb: 0,
        }
    }
}

impl PocComputer {
    /// Reset for an IDR picture.
    pub fn reset(&mut self) {
        self.is_first_picture = true;
        self.ref_pic_order_cnt_msb = 0;
        self.ref_pic_order_cnt_lsb = 0;
    }

    pub fn compute_poc(
        &mut self,
        sps: &SequenceParameterSet,
        pps: &PictureParameterSet,
        slice_segment_header: &SliceSegmentHeader,
    ) -> i32 {
        if slice_segment_header.nal_unit_type.is_idr() {
            return 0;
        }

        let slice_pic_order_cnt_lsb = slice_segment_header.slice_pic_order_cnt_lsb
      .expect("Non-IDR expected. All IDR pictures have PicOrderCntVal equal to 0 since slice_pic_order_cnt_lsb is inferred to be 0 for them and prevPicOrderCntLsb and prevPicOrderCntMsb are both set equal to 0.")
      as i32;

        self.compute_poc_ex(
            sps.log2_max_pic_order_cnt_lsb_minus4,
            pps.nuh_temporal_id_plus1,
            slice_segment_header.nal_unit_type,
            slice_pic_order_cnt_lsb,
        )
    }

    pub fn compute_poc_ex(
        &mut self,
        log2_max_pic_order_cnt_lsb_minus4: u8,
        nuh_temporal_id_plus1: u8,
        slice_nal_unit_type: NaluType,
        slice_pic_order_cnt_lsb: i32,
    ) -> i32 {
        if slice_nal_unit_type.is_idr() {
            return 0;
        }

        // MaxPicOrderCntLsb = 2^(log2_max_pic_order_cnt_lsb_minus4 + 4)
        let max_pic_order_cnt_lsb = 1 << (log2_max_pic_order_cnt_lsb_minus4 + 4);

        // 8.1.3 Decoding process for a coded picture with nuh_layer_id equal to 0
        //
        // > When the current picture is an IRAP picture, the following applies:
        // > - If the current picture is an IDR picture, a BLA picture, the first picture in the bitstream in decoding order, or the first
        // >   picture that follows an end of sequence NAL unit in decoding order, the variable NoRaslOutputFlag is set equal to 1.
        // > - Otherwise, if some external means ...
        // > - Otherwise, the variable NoRaslOutputFlag is set equal to 0.
        let no_rasl_output_flag = if slice_nal_unit_type.is_irap() {
            slice_nal_unit_type.is_idr() || slice_nal_unit_type.is_bla() || self.is_first_picture
        } else {
            false
        };

        let pic_order_cnt_msb = if !slice_nal_unit_type.is_irap() || !no_rasl_output_flag {
            // > When the current picture is not an IRAP picture with NoRaslOutputFlag equal to 1,
            // > - The variable prevPicOrderCntLsb is set equal to slice_pic_order_cnt_lsb of prevTid0Pic.
            // > - The variable prevPicOrderCntMsb is set equal to PicOrderCntMsb of prevTid0Pic.
            let prev_pic_order_cnt_lsb = self.ref_pic_order_cnt_lsb;
            let prev_pic_order_cnt_msb = self.ref_pic_order_cnt_msb;

            if (slice_pic_order_cnt_lsb < prev_pic_order_cnt_lsb)
                && ((prev_pic_order_cnt_lsb - slice_pic_order_cnt_lsb)
                    >= (max_pic_order_cnt_lsb / 2))
            {
                prev_pic_order_cnt_msb + max_pic_order_cnt_lsb
            } else if (slice_pic_order_cnt_lsb > prev_pic_order_cnt_lsb)
                && ((slice_pic_order_cnt_lsb - prev_pic_order_cnt_lsb)
                    > (max_pic_order_cnt_lsb / 2))
            {
                prev_pic_order_cnt_msb - max_pic_order_cnt_lsb
            } else {
                prev_pic_order_cnt_msb
            }
        } else {
            // > If the current picture is an IRAP picture with NoRaslOutputFlag equal to 1, PicOrderCntMsb is set equal to 0.
            0
        };

        // > Let prevTid0Pic be the previous picture in decoding order that has TemporalId equal to 0 and that is not a RASL,
        // > RADL or SLNR picture.
        let temporal_id = nuh_temporal_id_plus1 - 1;
        if temporal_id == 0 && !slice_nal_unit_type.is_rasl() && !slice_nal_unit_type.is_radl() {
            self.ref_pic_order_cnt_lsb = slice_pic_order_cnt_lsb;
            self.ref_pic_order_cnt_msb = pic_order_cnt_msb;
        }

        let pic_order_cnt = pic_order_cnt_msb + slice_pic_order_cnt_lsb;
        self.is_first_picture = false;

        pic_order_cnt
    }
}
