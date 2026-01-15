#pragma once

#include <vector>
#include <cstdint>
#include <string>

// Represents a single NAL unit extracted from MP4
struct MP4NalUnit {
    std::vector<uint8_t> data;
    uint8_t nal_unit_type;
};

// HEVC decoder configuration (from hvcC box)
struct HEVCDecoderConfig {
    uint8_t configuration_version;
    uint8_t general_profile_space;
    uint8_t general_tier_flag;
    uint8_t general_profile_idc;
    uint32_t general_profile_compatibility_flags;
    uint64_t general_constraint_indicator_flags;
    uint8_t general_level_idc;
    uint16_t min_spatial_segmentation_idc;
    uint8_t parallelism_type;
    uint8_t chroma_format_idc;
    uint8_t bit_depth_luma_minus8;
    uint8_t bit_depth_chroma_minus8;
    uint16_t avg_frame_rate;
    uint8_t constant_frame_rate;
    uint8_t num_temporal_layers;
    uint8_t temporal_id_nested;
    uint8_t length_size_minus_one;

    // NAL unit arrays (VPS, SPS, PPS, etc.)
    std::vector<MP4NalUnit> vps_list;
    std::vector<MP4NalUnit> sps_list;
    std::vector<MP4NalUnit> pps_list;
    std::vector<MP4NalUnit> sei_list;
};

// Result of MP4 demuxing
struct MP4DemuxResult {
    bool success;
    std::string error_message;

    HEVCDecoderConfig decoder_config;
    std::vector<MP4NalUnit> sample_nal_units;  // NAL units from first few samples

    uint32_t width;
    uint32_t height;
    uint32_t timescale;
    uint64_t duration;
};

// Demux HEVC stream from MP4 file
// Extracts decoder configuration (VPS, SPS, PPS) and first 'max_samples' video samples
MP4DemuxResult demux_hevc_from_mp4(const char* filename, size_t max_samples = 5);
