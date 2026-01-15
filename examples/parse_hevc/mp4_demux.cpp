#include "mp4_demux.h"
#include <Ap4.h>
#include <iostream>

// Helper to read big-endian 16-bit integer
static uint16_t read_be16(const uint8_t* data) {
    return (static_cast<uint16_t>(data[0]) << 8) | data[1];
}

// Parse hvcC box data to extract decoder configuration
static bool parse_hvcc(const std::vector<uint8_t>& data, HEVCDecoderConfig& config) {
    if (data.size() < 23) {
        return false;
    }

    size_t pos = 0;

    // Parse fixed fields
    config.configuration_version = data[pos++];

    uint8_t byte = data[pos++];
    config.general_profile_space = (byte >> 6) & 0x03;
    config.general_tier_flag = (byte >> 5) & 0x01;
    config.general_profile_idc = byte & 0x1F;

    config.general_profile_compatibility_flags =
        (static_cast<uint32_t>(data[pos]) << 24) |
        (static_cast<uint32_t>(data[pos + 1]) << 16) |
        (static_cast<uint32_t>(data[pos + 2]) << 8) |
        data[pos + 3];
    pos += 4;

    config.general_constraint_indicator_flags = 0;
    for (int i = 0; i < 6; i++) {
        config.general_constraint_indicator_flags =
            (config.general_constraint_indicator_flags << 8) | data[pos++];
    }

    config.general_level_idc = data[pos++];

    config.min_spatial_segmentation_idc = read_be16(&data[pos]) & 0x0FFF;
    pos += 2;

    config.parallelism_type = data[pos++] & 0x03;
    config.chroma_format_idc = data[pos++] & 0x03;
    config.bit_depth_luma_minus8 = data[pos++] & 0x07;
    config.bit_depth_chroma_minus8 = data[pos++] & 0x07;

    config.avg_frame_rate = read_be16(&data[pos]);
    pos += 2;

    byte = data[pos++];
    config.constant_frame_rate = (byte >> 6) & 0x03;
    config.num_temporal_layers = (byte >> 3) & 0x07;
    config.temporal_id_nested = (byte >> 2) & 0x01;
    config.length_size_minus_one = byte & 0x03;

    // Parse NAL unit arrays
    if (pos >= data.size()) {
        return false;
    }

    uint8_t num_of_arrays = data[pos++];

    for (uint8_t i = 0; i < num_of_arrays; i++) {
        if (pos + 3 > data.size()) {
            break;
        }

        uint8_t array_byte = data[pos++];
        uint8_t nal_unit_type = array_byte & 0x3F;
        uint16_t num_nalus = read_be16(&data[pos]);
        pos += 2;

        // Determine which list to add to
        std::vector<MP4NalUnit>* target_list = nullptr;
        if (nal_unit_type == 32) {  // VPS
            target_list = &config.vps_list;
        } else if (nal_unit_type == 33) {  // SPS
            target_list = &config.sps_list;
        } else if (nal_unit_type == 34) {  // PPS
            target_list = &config.pps_list;
        } else if (nal_unit_type == 39 || nal_unit_type == 40) {  // SEI
            target_list = &config.sei_list;
        }

        for (uint16_t j = 0; j < num_nalus; j++) {
            if (pos + 2 > data.size()) {
                break;
            }

            uint16_t nal_size = read_be16(&data[pos]);
            pos += 2;

            if (pos + nal_size > data.size()) {
                break;
            }

            if (target_list) {
                MP4NalUnit nalu;
                nalu.data.assign(data.begin() + pos, data.begin() + pos + nal_size);
                nalu.nal_unit_type = nal_unit_type;
                target_list->push_back(nalu);
            }

            pos += nal_size;
        }
    }

    return true;
}

// Extract NAL units from length-prefixed sample data
static std::vector<MP4NalUnit> extract_nal_units(const AP4_DataBuffer& sample_data,
                                                   uint8_t length_size_minus_one) {
    std::vector<MP4NalUnit> nalus;
    size_t length_size = length_size_minus_one + 1;
    size_t pos = 0;
    const uint8_t* data = sample_data.GetData();
    size_t data_size = sample_data.GetDataSize();

    while (pos + length_size <= data_size) {
        // Read NAL unit length
        size_t nal_length = 0;
        for (size_t i = 0; i < length_size; i++) {
            nal_length = (nal_length << 8) | data[pos + i];
        }
        pos += length_size;

        if (pos + nal_length > data_size) {
            break;
        }

        MP4NalUnit nalu;
        nalu.data.assign(data + pos, data + pos + nal_length);
        if (!nalu.data.empty()) {
            nalu.nal_unit_type = (nalu.data[0] >> 1) & 0x3F;
        }
        nalus.push_back(nalu);

        pos += nal_length;
    }

    return nalus;
}

MP4DemuxResult demux_hevc_from_mp4(const char* filename, size_t max_samples) {
    MP4DemuxResult result;
    result.success = false;
    result.width = 0;
    result.height = 0;
    result.timescale = 0;
    result.duration = 0;

    // Open file
    AP4_ByteStream* input = nullptr;
    AP4_Result ap4_result = AP4_FileByteStream::Create(
        filename,
        AP4_FileByteStream::STREAM_MODE_READ,
        input);

    if (AP4_FAILED(ap4_result) || !input) {
        result.error_message = "Failed to open file";
        return result;
    }

    // Parse MP4 file
    AP4_File file(*input, true);
    AP4_Movie* movie = file.GetMovie();

    if (!movie) {
        input->Release();
        result.error_message = "No movie found in file";
        return result;
    }

    // Find HEVC video track
    AP4_Track* video_track = nullptr;
    AP4_List<AP4_Track>& tracks = movie->GetTracks();
    for (AP4_List<AP4_Track>::Item* track_item = tracks.FirstItem();
         track_item;
         track_item = track_item->GetNext()) {
        AP4_Track* track = track_item->GetData();

        if (track->GetType() == AP4_Track::TYPE_VIDEO) {
            // Check if it's HEVC
            AP4_SampleDescription* sample_desc = track->GetSampleDescription(0);
            if (sample_desc) {
                AP4_HevcSampleDescription* hevc_desc =
                    AP4_DYNAMIC_CAST(AP4_HevcSampleDescription, sample_desc);
                if (hevc_desc) {
                    video_track = track;
                    break;
                }
            }
        }
    }

    if (!video_track) {
        input->Release();
        result.error_message = "No HEVC video track found";
        return result;
    }

    // Get track info
    result.timescale = video_track->GetMediaTimeScale();
    result.duration = video_track->GetMediaDuration();

    // Get HEVC sample description
    AP4_SampleDescription* sample_desc = video_track->GetSampleDescription(0);
    AP4_HevcSampleDescription* hevc_desc =
        AP4_DYNAMIC_CAST(AP4_HevcSampleDescription, sample_desc);

    if (!hevc_desc) {
        input->Release();
        result.error_message = "Failed to get HEVC sample description";
        return result;
    }

    // Get width and height
    result.width = hevc_desc->GetWidth();
    result.height = hevc_desc->GetHeight();

    // Extract decoder configuration
    result.decoder_config.configuration_version = 1;
    result.decoder_config.general_profile_idc = hevc_desc->GetGeneralProfile();
    result.decoder_config.general_level_idc = hevc_desc->GetGeneralLevel();
    result.decoder_config.general_tier_flag = hevc_desc->GetGeneralTierFlag();
    result.decoder_config.general_profile_space = hevc_desc->GetGeneralProfileSpace();
    result.decoder_config.chroma_format_idc = hevc_desc->GetChromaFormat();
    result.decoder_config.bit_depth_luma_minus8 = hevc_desc->GetLumaBitDepth() - 8;
    result.decoder_config.bit_depth_chroma_minus8 = hevc_desc->GetChromaBitDepth() - 8;
    result.decoder_config.length_size_minus_one = hevc_desc->GetNaluLengthSize() - 1;

    // Extract parameter sets from sequences
    const AP4_Array<AP4_HvccAtom::Sequence>& sequences = hevc_desc->GetSequences();
    for (unsigned int i = 0; i < sequences.ItemCount(); i++) {
        const AP4_HvccAtom::Sequence& seq = sequences[i];
        uint8_t nal_unit_type = seq.m_NaluType;

        const AP4_Array<AP4_DataBuffer>& nalus = seq.m_Nalus;
        for (unsigned int j = 0; j < nalus.ItemCount(); j++) {
            const AP4_DataBuffer& nalu_data = nalus[j];

            MP4NalUnit nalu;
            nalu.data.assign(nalu_data.GetData(),
                           nalu_data.GetData() + nalu_data.GetDataSize());
            nalu.nal_unit_type = nal_unit_type;

            if (nal_unit_type == 32) {  // VPS
                result.decoder_config.vps_list.push_back(nalu);
            } else if (nal_unit_type == 33) {  // SPS
                result.decoder_config.sps_list.push_back(nalu);
            } else if (nal_unit_type == 34) {  // PPS
                result.decoder_config.pps_list.push_back(nalu);
            } else if (nal_unit_type == 39 || nal_unit_type == 40) {  // SEI
                result.decoder_config.sei_list.push_back(nalu);
            }
        }
    }

    if (result.decoder_config.vps_list.empty() &&
        result.decoder_config.sps_list.empty() &&
        result.decoder_config.pps_list.empty()) {
        input->Release();
        result.error_message = "No parameter sets found";
        return result;
    }

    // Read video samples
    AP4_Sample sample;
    AP4_DataBuffer sample_data;
    AP4_Ordinal sample_index = 0;
    size_t samples_read = 0;

    while (samples_read < max_samples &&
           AP4_SUCCEEDED(video_track->ReadSample(sample_index, sample, sample_data))) {
        // Extract NAL units from this sample
        auto sample_nalus = extract_nal_units(sample_data, result.decoder_config.length_size_minus_one);
        result.sample_nal_units.insert(result.sample_nal_units.end(),
                                      sample_nalus.begin(),
                                      sample_nalus.end());

        sample_index++;
        samples_read++;
    }

    result.success = true;
    input->Release();

    return result;
}
