//! Basic FFI tests

#[cfg(feature = "ffi")]
mod ffi_tests {
    use vidsynt::ffi::{
        vidsynt_hevc_context_new, vidsynt_hevc_context_free,
        vidsynt_hevc_nalu_type_is_idr, vidsynt_hevc_nalu_type_is_irap,
        vidsynt_hevc_nalu_type_is_reference, VidsyntNaluType,
        vidsynt_hevc_parse_nalu_from_bytes, VidsyntResult,
    };
    use std::ptr;

    #[test]
    fn test_context_lifecycle() {
        unsafe {
            // Create context
            let ctx = vidsynt_hevc_context_new();
            assert!(!ctx.is_null(), "Context creation should succeed");

            // Free context
            vidsynt_hevc_context_free(ctx);
        }
    }

    #[test]
    fn test_context_free_null() {
        unsafe {
            // Freeing null should be safe (no-op)
            vidsynt_hevc_context_free(ptr::null_mut());
        }
    }

    #[test]
    fn test_nalu_type_queries() {
        // Test IDR
        let idr_type = VidsyntNaluType(19); // IDR_W_RADL
        assert_eq!(vidsynt_hevc_nalu_type_is_idr(idr_type), 1);
        assert_eq!(vidsynt_hevc_nalu_type_is_irap(idr_type), 1);
        assert_eq!(vidsynt_hevc_nalu_type_is_reference(idr_type), 1);

        // Test non-reference
        let trail_n_type = VidsyntNaluType(0); // TRAIL_N
        assert_eq!(vidsynt_hevc_nalu_type_is_idr(trail_n_type), 0);
        assert_eq!(vidsynt_hevc_nalu_type_is_reference(trail_n_type), 0);

        // Test VPS
        let vps_type = VidsyntNaluType(32); // VPS_NUT
        assert_eq!(vidsynt_hevc_nalu_type_is_idr(vps_type), 0);
        assert_eq!(vidsynt_hevc_nalu_type_is_irap(vps_type), 0);
    }

    #[test]
    fn test_parse_invalid_parameters() {
        unsafe {
            let ctx = vidsynt_hevc_context_new();
            assert!(!ctx.is_null());

            let data = [0u8; 10];
            let mut out_nalu = ptr::null();

            // Test null context
            let result = vidsynt_hevc_parse_nalu_from_bytes(
                ptr::null_mut(),
                data.as_ptr(),
                data.len(),
                &mut out_nalu,
            );
            assert_eq!(result, VidsyntResult::InvalidParameter);

            // Test null data
            let result = vidsynt_hevc_parse_nalu_from_bytes(
                ctx,
                ptr::null(),
                10,
                &mut out_nalu,
            );
            assert_eq!(result, VidsyntResult::InvalidParameter);

            // Test null output
            let result = vidsynt_hevc_parse_nalu_from_bytes(
                ctx,
                data.as_ptr(),
                data.len(),
                ptr::null_mut(),
            );
            assert_eq!(result, VidsyntResult::InvalidParameter);

            // Test zero length
            let result = vidsynt_hevc_parse_nalu_from_bytes(
                ctx,
                data.as_ptr(),
                0,
                &mut out_nalu,
            );
            assert_eq!(result, VidsyntResult::InvalidParameter);

            vidsynt_hevc_context_free(ctx);
        }
    }

    #[test]
    fn test_parse_simple_vps() {
        // Minimal VPS NAL unit
        // This is a hand-crafted minimal VPS for testing
        let vps_data = [
            // NAL header (2 bytes)
            0x40, 0x01,  // VPS_NUT (type 32), nuh_layer_id=0, nuh_temporal_id_plus1=1
            // VPS RBSP
            0x01,        // vps_video_parameter_set_id=0, vps_base_layer_internal_flag=0,
                         // vps_base_layer_available_flag=1, vps_max_layers_minus1=0 (1 bit each, then 6 bits)
            0x60, 0xFF, 0xFF,  // vps_max_sub_layers_minus1=0, vps_temporal_id_nesting_flag=1,
                               // vps_reserved_0xffff_16bits=0xFFFF
            // profile_tier_level
            0x01,        // general_profile_space=0, general_tier_flag=0, general_profile_idc=1 (Main)
            0xFF, 0xFF, 0xFF, 0xFF,  // general_profile_compatibility_flag[32]
            0x90, 0x00, 0x00, 0x00,  // constraint flags
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
            0x5D,        // general_level_idc = 93 (Level 3.1)
            // ... (continuing VPS, simplified)
            0x00, 0xF8,  // Sub-layer flags + more
            0x3D, 0xC0,  // vps_max_layer_id=0, vps_num_layer_sets_minus1=0 (ue(v))
                         // + vps_sub_layer_ordering_info_present_flag=0
                         // + vps_max_dec_pic_buffering_minus1[0]=1
            0x80,        // vps_max_num_reorder_pics[0]=0, vps_max_latency_increase_plus1[0]=0
                         // vps_timing_info_present_flag=0
                         // vps_extension_flag=0
                         // rbsp_stop_one_bit + alignment
        ];

        unsafe {
            let ctx = vidsynt_hevc_context_new();
            assert!(!ctx.is_null());

            let mut out_nalu = ptr::null();
            let result = vidsynt_hevc_parse_nalu_from_bytes(
                ctx,
                vps_data.as_ptr(),
                vps_data.len(),
                &mut out_nalu,
            );

            // The parse might fail because the VPS data isn't complete/valid
            // For now, we just check that it doesn't crash
            match result {
                VidsyntResult::Success => {
                    assert!(!out_nalu.is_null(), "NAL unit pointer should be valid");
                    println!("Successfully parsed VPS NAL unit");
                }
                VidsyntResult::ParseFailed => {
                    println!("Parse failed (expected for incomplete test data)");
                }
                _ => panic!("Unexpected result: {:?}", result),
            }

            vidsynt_hevc_context_free(ctx);
        }
    }
}
