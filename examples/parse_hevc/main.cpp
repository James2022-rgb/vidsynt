#include "vidsynt.h"
#include "mp4_demux.h"
#include <iostream>
#include <fstream>
#include <vector>
#include <map>
#include <iomanip>
#include <cstring>
#include <algorithm>

// Helper function to print error messages
void print_error(const char* operation, VidsyntResult result) {
    std::cerr << "Error during " << operation << ": ";
    switch (result) {
        case VidsyntResult::Success:
            std::cerr << "Success";
            break;
        case VidsyntResult::InvalidParameter:
            std::cerr << "Invalid parameter";
            break;
        case VidsyntResult::ParseFailed:
            std::cerr << "Parse failed";
            break;
        case VidsyntResult::OutOfMemory:
            std::cerr << "Out of memory";
            break;
        case VidsyntResult::UnsupportedFeature:
            std::cerr << "Unsupported feature";
            break;
        case VidsyntResult::InvalidNaluType:
            std::cerr << "Invalid NAL unit type";
            break;
        case VidsyntResult::DataNotAvailable:
            std::cerr << "Data not available";
            break;
        default:
            std::cerr << "Unknown error (" << static_cast<int>(result) << ")";
            break;
    }
    std::cerr << "\n";
}

// Helper function to read entire file into memory
std::vector<uint8_t> read_file(const char* filename) {
    std::ifstream file(filename, std::ios::binary | std::ios::ate);
    if (!file) {
        std::cerr << "Failed to open file: " << filename << "\n";
        return {};
    }

    std::streamsize size = file.tellg();
    file.seekg(0, std::ios::beg);

    std::vector<uint8_t> buffer(size);
    if (!file.read(reinterpret_cast<char*>(buffer.data()), size)) {
        std::cerr << "Failed to read file: " << filename << "\n";
        return {};
    }

    return buffer;
}

// RAII wrapper for VidsyntHevcContext
class VidsyntHevcContextGuard {
public:
    VidsyntHevcContextGuard() : ctx_(vidsynt_hevc_context_new()) {
        if (!ctx_) {
            throw std::runtime_error("Failed to create vidsynt context");
        }
    }

    ~VidsyntHevcContextGuard() {
        if (ctx_) {
            vidsynt_hevc_context_free(ctx_);
        }
    }

    // Disable copy
    VidsyntHevcContextGuard(const VidsyntHevcContextGuard&) = delete;
    VidsyntHevcContextGuard& operator=(const VidsyntHevcContextGuard&) = delete;

    // Allow move
    VidsyntHevcContextGuard(VidsyntHevcContextGuard&& other) noexcept : ctx_(other.ctx_) {
        other.ctx_ = nullptr;
    }

    VidsyntHevcContextGuard& operator=(VidsyntHevcContextGuard&& other) noexcept {
        if (this != &other) {
            if (ctx_) {
                vidsynt_hevc_context_free(ctx_);
            }
            ctx_ = other.ctx_;
            other.ctx_ = nullptr;
        }
        return *this;
    }

    VidsyntHevcContext* get() { return ctx_; }
    operator VidsyntHevcContext*() { return ctx_; }

private:
    VidsyntHevcContext* ctx_;
};

void print_nalu_type_info(uint8_t nal_type) {
    std::cout << "  NAL unit type: " << static_cast<int>(nal_type);

    // Print type name
    if (nal_type == VIDSYNT_HEVC_NALU_VPS_NUT) {  // VPS
        std::cout << " (VPS)";
    } else if (nal_type == VIDSYNT_HEVC_NALU_SPS_NUT) {  // SPS
        std::cout << " (SPS)";
    } else if (nal_type == VIDSYNT_HEVC_NALU_PPS_NUT) {  // PPS
        std::cout << " (PPS)";
    } else if (nal_type == VIDSYNT_HEVC_NALU_IDR_W_RADL) {  // IDR_W_RADL
        std::cout << " (IDR_W_RADL)";
    } else if (nal_type == VIDSYNT_HEVC_NALU_IDR_N_LP) {  // IDR_N_LP
        std::cout << " (IDR_N_LP)";
    } else if (nal_type == VIDSYNT_HEVC_NALU_CRA_NUT) {  // CRA
        std::cout << " (CRA)";
    }

    std::cout << "\n";

    // Check properties
    if (vidsynt_hevc_nalu_type_is_idr(nal_type)) {
        std::cout << "  Is IDR picture\n";
    }
    if (vidsynt_hevc_nalu_type_is_irap(nal_type)) {
        std::cout << "  Is IRAP picture\n";
    }
    if (vidsynt_hevc_nalu_type_is_reference(nal_type)) {
        std::cout << "  Is reference picture\n";
    }
}

void parse_and_print_nalu(VidsyntHevcContext* ctx, const uint8_t* data, size_t len) {
    const VidsyntHevcNalu* nalu = nullptr;
    VidsyntResult res = vidsynt_hevc_parse_nalu_from_bytes(ctx, data, len, &nalu);

    if (res != VidsyntResult::Success) {
        print_error("parse NAL unit", res);
        return;
    }

    std::cout << "\nSuccessfully parsed NAL unit\n";

    // Get NAL unit type
    uint8_t nal_type = vidsynt_hevc_nalu_get_type(nalu);
    print_nalu_type_info(nal_type);

    // Try to extract parameter sets
    if (nal_type == VIDSYNT_HEVC_NALU_VPS_NUT) {  // VPS
        const VidsyntHevcVideoParameterSet* vps = nullptr;
        if (vidsynt_hevc_nalu_get_vps(ctx, nalu, &vps) == VidsyntResult::Success && vps) {
            std::cout << "\nVideo Parameter Set (VPS):\n";
            std::cout << "  vps_video_parameter_set_id: " << static_cast<int>(vps->vps_video_parameter_set_id) << "\n";
            std::cout << "  vps_max_layers_minus1: " << static_cast<int>(vps->vps_max_layers_minus1) << "\n";
            std::cout << "  vps_max_sub_layers_minus1: " << static_cast<int>(vps->vps_max_sub_layers_minus1) << "\n";
            std::cout << "  vps_temporal_id_nesting_flag: " << static_cast<int>(vps->vps_temporal_id_nesting_flag) << "\n";

            if (vps->profile_tier_level) {
                std::cout << "  Profile/Tier/Level:\n";
                std::cout << "    general_profile_idc: " << static_cast<int>(vps->profile_tier_level->general_profile_idc) << "\n";
                std::cout << "    general_level_idc: " << static_cast<int>(vps->profile_tier_level->general_level_idc) << "\n";
            }
        }
    } else if (nal_type == VIDSYNT_HEVC_NALU_SPS_NUT) {  // SPS
        const VidsyntHevcSequenceParameterSet* sps = nullptr;
        if (vidsynt_hevc_nalu_get_sps(ctx, nalu, &sps) == VidsyntResult::Success && sps) {
            std::cout << "\nSequence Parameter Set (SPS):\n";
            std::cout << "  sps_video_parameter_set_id: " << static_cast<int>(sps->sps_video_parameter_set_id) << "\n";
            std::cout << "  sps_seq_parameter_set_id: " << static_cast<int>(sps->sps_seq_parameter_set_id) << "\n";
            std::cout << "  Resolution: " << sps->pic_width_in_luma_samples << "x" << sps->pic_height_in_luma_samples << "\n";
            std::cout << "  Bit depth (luma): " << static_cast<int>(sps->bit_depth_luma_minus8 + 8) << " bit\n";
            std::cout << "  Bit depth (chroma): " << static_cast<int>(sps->bit_depth_chroma_minus8 + 8) << " bit\n";
            std::cout << "  Chroma format: " << static_cast<int>(sps->chroma_format_idc);
            if (sps->chroma_format_idc == 0) std::cout << " (monochrome)";
            else if (sps->chroma_format_idc == 1) std::cout << " (4:2:0)";
            else if (sps->chroma_format_idc == 2) std::cout << " (4:2:2)";
            else if (sps->chroma_format_idc == 3) std::cout << " (4:4:4)";
            std::cout << "\n";

            if (sps->profile_tier_level) {
                std::cout << "  Profile/Tier/Level:\n";
                std::cout << "    general_profile_idc: " << static_cast<int>(sps->profile_tier_level->general_profile_idc);
                if (sps->profile_tier_level->general_profile_idc == 1) std::cout << " (Main)";
                else if (sps->profile_tier_level->general_profile_idc == 2) std::cout << " (Main 10)";
                std::cout << "\n";
                std::cout << "    general_level_idc: " << static_cast<int>(sps->profile_tier_level->general_level_idc);
                std::cout << " (Level " << (sps->profile_tier_level->general_level_idc / 30.0) << ")\n";
            }

            if (sps->conformance_window) {
                std::cout << "  Conformance window (cropping):\n";
                std::cout << "    left: " << sps->conformance_window->conf_win_left_offset << "\n";
                std::cout << "    right: " << sps->conformance_window->conf_win_right_offset << "\n";
                std::cout << "    top: " << sps->conformance_window->conf_win_top_offset << "\n";
                std::cout << "    bottom: " << sps->conformance_window->conf_win_bottom_offset << "\n";
            }

            // Set as active SPS for slice parsing
            vidsynt_hevc_context_set_active_sps(ctx, sps);
        }
    } else if (nal_type == VIDSYNT_HEVC_NALU_PPS_NUT) {  // PPS
        const VidsyntHevcPictureParameterSet* pps = nullptr;
        if (vidsynt_hevc_nalu_get_pps(ctx, nalu, &pps) == VidsyntResult::Success && pps) {
            std::cout << "\nPicture Parameter Set (PPS):\n";
            std::cout << "  pps_pic_parameter_set_id: " << static_cast<int>(pps->pps_pic_parameter_set_id) << "\n";
            std::cout << "  pps_seq_parameter_set_id: " << static_cast<int>(pps->pps_seq_parameter_set_id) << "\n";
            std::cout << "  dependent_slice_segments_enabled_flag: " << static_cast<int>(pps->dependent_slice_segments_enabled_flag) << "\n";
            std::cout << "  output_flag_present_flag: " << static_cast<int>(pps->output_flag_present_flag) << "\n";
            std::cout << "  num_extra_slice_header_bits: " << static_cast<int>(pps->num_extra_slice_header_bits) << "\n";

            // Set as active PPS for slice parsing
            vidsynt_hevc_context_set_active_pps(ctx, pps);
        }
    } else if (nal_type >= VIDSYNT_HEVC_NALU_TRAIL_N && nal_type <= VIDSYNT_HEVC_NALU_CRA_NUT) {  // Coded slice segment (VCL)
        const VidsyntHevcSliceSegmentHeader* slice = nullptr;
        if (vidsynt_hevc_nalu_get_slice_header(ctx, nalu, &slice) == VidsyntResult::Success && slice) {
            std::cout << "\nSlice Segment Header:\n";
            std::cout << "  first_slice_segment_in_pic_flag: " << static_cast<int>(slice->first_slice_segment_in_pic_flag) << "\n";
            std::cout << "  slice_pic_parameter_set_id: " << static_cast<int>(slice->slice_pic_parameter_set_id) << "\n";
            std::cout << "  dependent_slice_segment_flag: " << static_cast<int>(slice->dependent_slice_segment_flag) << "\n";
            std::cout << "  slice_segment_address: " << slice->slice_segment_address << "\n";
        }
    }
}

// Helper to check if file is MP4
bool is_mp4_file(const char* filename) {
    std::string name(filename);
    std::transform(name.begin(), name.end(), name.begin(), ::tolower);
    return name.ends_with(".mp4") || name.ends_with(".m4v") || name.ends_with(".mov");
}

// Process MP4 file
void process_mp4_file(VidsyntHevcContext* ctx, const char* filename) {
    std::cout << "\nDemuxing MP4 file...\n";

    MP4DemuxResult result = demux_hevc_from_mp4(filename, 5);

    if (!result.success) {
        std::cerr << "Failed to demux MP4: " << result.error_message << "\n";
        return;
    }

    std::cout << "\nMP4 Information:\n";
    std::cout << "  Resolution: " << result.width << "x" << result.height << "\n";
    std::cout << "  Timescale: " << result.timescale << "\n";
    std::cout << "  Duration: " << result.duration << " (timescale units)\n";
    if (result.timescale > 0) {
        double duration_sec = static_cast<double>(result.duration) / result.timescale;
        std::cout << "  Duration: " << duration_sec << " seconds\n";
    }

    std::cout << "\nDecoder Configuration (hvcC):\n";
    std::cout << "  Profile: " << static_cast<int>(result.decoder_config.general_profile_idc) << "\n";
    std::cout << "  Level: " << static_cast<int>(result.decoder_config.general_level_idc)
              << " (Level " << (result.decoder_config.general_level_idc / 30.0) << ")\n";
    std::cout << "  Chroma format: " << static_cast<int>(result.decoder_config.chroma_format_idc) << "\n";
    std::cout << "  Bit depth (luma): " << static_cast<int>(result.decoder_config.bit_depth_luma_minus8 + 8) << "\n";
    std::cout << "  Bit depth (chroma): " << static_cast<int>(result.decoder_config.bit_depth_chroma_minus8 + 8) << "\n";
    std::cout << "  Length size: " << static_cast<int>(result.decoder_config.length_size_minus_one + 1) << " bytes\n";

    // Parse parameter sets from hvcC
    std::cout << "\n=== Parameter Sets from hvcC ===\n";

    for (size_t i = 0; i < result.decoder_config.vps_list.size(); i++) {
        std::cout << "\n[VPS #" << (i + 1) << "]\n";
        parse_and_print_nalu(ctx, result.decoder_config.vps_list[i].data.data(),
                            result.decoder_config.vps_list[i].data.size());
    }

    for (size_t i = 0; i < result.decoder_config.sps_list.size(); i++) {
        std::cout << "\n[SPS #" << (i + 1) << "]\n";
        parse_and_print_nalu(ctx, result.decoder_config.sps_list[i].data.data(),
                            result.decoder_config.sps_list[i].data.size());
    }

    for (size_t i = 0; i < result.decoder_config.pps_list.size(); i++) {
        std::cout << "\n[PPS #" << (i + 1) << "]\n";
        parse_and_print_nalu(ctx, result.decoder_config.pps_list[i].data.data(),
                            result.decoder_config.pps_list[i].data.size());
    }

    // Parse NAL units from samples
    if (!result.sample_nal_units.empty()) {
        std::cout << "\n=== NAL Units from Video Samples ===\n";
        std::cout << "Total NAL units extracted: " << result.sample_nal_units.size() << "\n";

        // Group by type
        std::map<uint8_t, int> nal_type_counts;
        for (const auto& nalu : result.sample_nal_units) {
            nal_type_counts[nalu.nal_unit_type]++;
        }

        std::cout << "\nNAL unit type distribution:\n";
        for (const auto& [type, count] : nal_type_counts) {
            std::cout << "  Type " << static_cast<int>(type) << ": " << count << " units";
            if (type >= 0 && type <= 9) {
                std::cout << " (VCL - coded slice)";
            } else if (type >= VIDSYNT_HEVC_NALU_BLA_W_LP && type <= VIDSYNT_HEVC_NALU_CRA_NUT) {
                std::cout << " (VCL - IRAP)";
            } else if (type == VIDSYNT_HEVC_NALU_VPS_NUT) {
                std::cout << " (VPS)";
            } else if (type == VIDSYNT_HEVC_NALU_SPS_NUT) {
                std::cout << " (SPS)";
            } else if (type == VIDSYNT_HEVC_NALU_PPS_NUT) {
                std::cout << " (PPS)";
            }
            std::cout << "\n";
        }

        // Parse and display first few interesting NAL units
        std::cout << "\nParsing first few NAL units:\n";
        int displayed = 0;
        for (const auto& nalu : result.sample_nal_units) {
            if (displayed >= 10) break;  // Limit output

            // Parse parameter sets and VCL NAL units (slices)
            // Now that we have active SPS/PPS, slices should parse correctly
            if (nalu.nal_unit_type >= VIDSYNT_HEVC_NALU_VPS_NUT && nalu.nal_unit_type <= VIDSYNT_HEVC_NALU_PPS_NUT) {
                std::cout << "\n[NAL unit - Type " << static_cast<int>(nalu.nal_unit_type) << "]\n";
                parse_and_print_nalu(ctx, nalu.data.data(), nalu.data.size());
                displayed++;
            } else if (nalu.nal_unit_type >= VIDSYNT_HEVC_NALU_TRAIL_N && nalu.nal_unit_type <= VIDSYNT_HEVC_NALU_CRA_NUT) {
                // VCL NAL units (coded slices) - now parseable with active SPS/PPS
                std::cout << "\n[NAL unit - Type " << static_cast<int>(nalu.nal_unit_type);
                if (nalu.nal_unit_type >= VIDSYNT_HEVC_NALU_BLA_W_LP && nalu.nal_unit_type <= VIDSYNT_HEVC_NALU_CRA_NUT) {
                    std::cout << " - IRAP";
                }
                std::cout << ", size " << nalu.data.size() << " bytes]\n";
                parse_and_print_nalu(ctx, nalu.data.data(), nalu.data.size());
                displayed++;
            }
        }
    }
}

int main(int argc, char* argv[]) {
    std::cout << "vidsynt H.265/HEVC parser example\n";
    std::cout << "==================================\n\n";

    // Create context
    VidsyntHevcContextGuard ctx;

    if (argc > 1) {
        // Parse file provided as command-line argument
        const char* filename = argv[1];
        std::cout << "Reading file: " << filename << "\n";

        // Check if it's an MP4 file
        if (is_mp4_file(filename)) {
            process_mp4_file(ctx, filename);
        } else {
            // Treat as raw HEVC bitstream
            std::vector<uint8_t> data = read_file(filename);
            if (data.empty()) {
                return 1;
            }

            std::cout << "File size: " << data.size() << " bytes\n";
            std::cout << "\nParsing as raw HEVC bitstream...\n";

            // Try to parse as a single NAL unit
            parse_and_print_nalu(ctx, data.data(), data.size());
        }

    } else {
        std::cout << "Usage: " << argv[0] << " <hevc_bitstream_file>\n";
        std::cout << "\nNo file provided, demonstrating basic API usage:\n\n";

        // Demonstrate format conversion
        std::cout << "--- Format Conversion Example ---\n";

        // Sample length-prefixed NAL units (4-byte length prefix)
        // Format: [length (4 bytes)][NAL data][length][NAL data]...
        // This is a minimal example - replace with real data
        const uint8_t length_prefixed_data[] = {
            // Length: 5 bytes
            0x00, 0x00, 0x00, 0x05,
            // NAL unit header + minimal payload
            0x40, 0x01, 0x0C, 0x01, 0xFF,
            // Length: 4 bytes
            0x00, 0x00, 0x00, 0x04,
            // Another NAL unit
            0x42, 0x01, 0x01, 0x50
        };

        const uint8_t* annex_b_data = nullptr;
        size_t annex_b_len = 0;

        VidsyntResult res = vidsynt_hevc_convert_length_prefixed_to_annex_b(
            ctx,
            length_prefixed_data,
            sizeof(length_prefixed_data),
            3,  // 4-byte length fields (3 = 4-1)
            &annex_b_data,
            &annex_b_len
        );

        if (res == VidsyntResult::Success) {
            std::cout << "Successfully converted to Annex B format\n";
            std::cout << "Output size: " << annex_b_len << " bytes\n";
            std::cout << "First 16 bytes (hex): ";
            for (size_t i = 0; i < std::min<size_t>(16, annex_b_len); i++) {
                std::cout << std::hex << std::setw(2) << std::setfill('0')
                         << static_cast<int>(annex_b_data[i]) << " ";
            }
            std::cout << std::dec << "\n";
        } else {
            print_error("format conversion", res);
        }
    }

    std::cout << "\nDone.\n";
    return 0;
}
