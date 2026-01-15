# H.265/HEVC Parser Example

This example demonstrates how to use the vidsynt C FFI from C++ to parse H.265/HEVC bitstreams.

## Prerequisites

- CMake 3.15 or later
- C++20 compatible compiler
- Rust toolchain (to build vidsynt)

## Building

### 1. Build vidsynt with FFI support

First, build the vidsynt library with the FFI feature enabled:

```bash
# From the vidsynt root directory
cd ../..
cargo build --release --features ffi
```

This creates the static library at:
- Windows: `target/release/vidsynt.lib`
- Linux: `target/release/libvidsynt.a`
- macOS: `target/release/libvidsynt.a`

### 2. Generate the C header

Build the vidsynt-sys crate to generate the C header file:

```bash
cd vidsynt-sys
cargo build
```

This creates `vidsynt-sys/include/vidsynt.h`.

### 3. Build the example

Now build the C++ example:

```bash
cd ../examples/parse_hevc
mkdir build
cd build
cmake ..
cmake --build . --config Release
```

On Windows with Visual Studio, you can also open the generated solution:
```powershell
cmake ..
start vidsynt_hevc_example.sln
```

## Running

### Basic usage

Without arguments, the example demonstrates basic API usage:

```bash
# Unix
./parse_hevc

# Windows
.\Release\parse_hevc.exe
```

### Parse a file

To parse an H.265 bitstream file:

```bash
# Unix
./parse_hevc /path/to/video.h265
./parse_hevc /path/to/video.mp4

# Windows
.\Release\parse_hevc.exe C:\path\to\video.h265
.\Release\parse_hevc.exe C:\path\to\video.mp4
```

The example supports:
- **MP4/M4V/MOV files**: Automatically demuxes HEVC stream, extracts parameter sets from hvcC box, and parses video samples
- **Parameter sets**: VPS, SPS, PPS with full syntax element extraction
- **VCL NAL units**: Coded slice segments (including IDR, IRAP, and inter-coded slices)
- **Explicit context management**: Demonstrates explicit activation of SPS/PPS for slice parsing
- Annex B format (start code separated NAL units)
- Raw length-prefixed format

## Example Output

### Parsing MP4 file

```
vidsynt H.265/HEVC parser example
==================================

Reading file: video.mp4

Demuxing MP4 file...

MP4 Information:
  Resolution: 1920x1080
  Timescale: 30000
  Duration: 300000 (timescale units)
  Duration: 10.0 seconds

Decoder Configuration (hvcC):
  Profile: 1
  Level: 120 (Level 4.0)
  Chroma format: 1
  Bit depth (luma): 8
  Bit depth (chroma): 8
  Length size: 4 bytes

=== Parameter Sets from hvcC ===

[VPS #1]

Successfully parsed NAL unit
  NAL unit type: 32 (VPS)
  Is reference picture

Video Parameter Set (VPS):
  vps_video_parameter_set_id: 0
  vps_max_layers_minus1: 0
  vps_max_sub_layers_minus1: 0
  ...

[SPS #1]

Successfully parsed NAL unit
  NAL unit type: 33 (SPS)
  Is reference picture

Sequence Parameter Set (SPS):
  sps_video_parameter_set_id: 0
  sps_seq_parameter_set_id: 0
  Resolution: 1920x1080
  Bit depth (luma): 8 bit
  Bit depth (chroma): 8 bit
  Chroma format: 1 (4:2:0)
  Profile/Tier/Level:
    general_profile_idc: 1 (Main)
    general_level_idc: 120 (Level 4.0)

[PPS #1]

Successfully parsed NAL unit
  NAL unit type: 34 (PPS)
  Is reference picture

Picture Parameter Set (PPS):
  pps_pic_parameter_set_id: 0
  pps_seq_parameter_set_id: 0
  ...

=== NAL Units from Video Samples ===
Total NAL units extracted: 25

NAL unit type distribution:
  Type 1: 20 units (VCL - coded slice)
  Type 19: 5 units (VCL - IRAP)

Parsing first few NAL units:

[NAL unit - Type 19 - IRAP, size 4600 bytes]

Successfully parsed NAL unit
  NAL unit type: 19 (IDR_W_RADL)
  Is IDR picture
  Is IRAP picture
  Is reference picture

Slice Segment Header:
  first_slice_segment_in_pic_flag: 1
  slice_pic_parameter_set_id: 0
  dependent_slice_segment_flag: 255
  slice_segment_address: 0

[NAL unit - Type 19 - IRAP, size 9889 bytes]

Successfully parsed NAL unit
  NAL unit type: 19 (IDR_W_RADL)
  Is IDR picture
  Is IRAP picture
  Is reference picture

Slice Segment Header:
  first_slice_segment_in_pic_flag: 0
  slice_pic_parameter_set_id: 0
  dependent_slice_segment_flag: 0
  slice_segment_address: 60

Done.
```

### Parsing raw NAL unit

```
vidsynt H.265/HEVC parser example
==================================

Reading file: sample.sps
File size: 42 bytes

Parsing as raw HEVC bitstream...

Successfully parsed NAL unit
  NAL unit type: 33 (SPS)
  Is reference picture

Sequence Parameter Set (SPS):
  sps_video_parameter_set_id: 0
  sps_seq_parameter_set_id: 0
  Resolution: 1920x1080
  Bit depth (luma): 8 bit
  Bit depth (chroma): 8 bit
  Chroma format: 1 (4:2:0)
  Profile/Tier/Level:
    general_profile_idc: 1 (Main)
    general_level_idc: 120 (Level 4.0)

Done.
```

## Code Structure

The example demonstrates:

1. **MP4 Demuxing** (`mp4_demux.cpp`): Uses Bento4 library to extract HEVC streams
   - Parses MP4 container structure
   - Extracts decoder configuration (VPS, SPS, PPS) from hvcC box
   - Reads video samples and extracts length-prefixed NAL units
2. **Context management**: Using RAII wrapper `VidsyntHevcContextGuard` for automatic cleanup
3. **Explicit parameter set activation**: Shows how to explicitly set active SPS/PPS for slice parsing
4. **NAL unit parsing**: Parsing all NAL unit types including:
   - Parameter sets (VPS, SPS, PPS)
   - Coded slice segments (IDR, IRAP, inter-coded slices)
   - The parser automatically uses active SPS/PPS for slice parsing
5. **Type queries**: Checking NAL unit types (IDR, IRAP, reference)
6. **Slice segment header access**: Reading slice information from VCL NAL units
7. **Parameter set access**: Reading VPS, SPS, and PPS structures
8. **Format conversion**: Converting between Annex B and length-prefixed formats

## Obtaining H.265 Test Files

You can obtain H.265 test files from:

1. **Use MP4 files directly** (Recommended): The example now supports MP4 files natively!
   - Any H.265/HEVC encoded MP4 video will work
   - Create one with FFmpeg:
     ```bash
     ffmpeg -i input.mp4 -c:v libx265 -crf 28 -preset medium output_hevc.mp4
     ```

2. **FFmpeg**: Extract raw NAL units from any H.265 video:
   ```bash
   ffmpeg -i input.mp4 -vcodec copy -bsf:v hevc_mp4toannexb -f hevc output.h265
   ```

3. **JCT-VC Test Sequences**: Download conformance bitstreams from ITU-T/ISO/IEC

4. **Create minimal test files**: Use a hex editor to create individual NAL units (VPS, SPS, PPS)

## API Reference

See the full API documentation in `vidsynt-sys/include/vidsynt.h`.

### Usage Pattern

To parse VCL NAL units (coded slices), you must explicitly set active SPS and PPS:

```cpp
// 1. Parse parameter sets
const VidsyntHevcSequenceParameterSet* sps = nullptr;
vidsynt_hevc_nalu_get_sps(ctx, sps_nalu, &sps);

const VidsyntHevcPictureParameterSet* pps = nullptr;
vidsynt_hevc_nalu_get_pps(ctx, pps_nalu, &pps);

// 2. Explicitly set them as active for slice parsing
vidsynt_hevc_context_set_active_sps(ctx, sps);
vidsynt_hevc_context_set_active_pps(ctx, pps);

// 3. Now you can parse slice NAL units
const VidsyntHevcNalu* slice_nalu = nullptr;
vidsynt_hevc_parse_nalu_from_bytes(ctx, slice_data, len, &slice_nalu);

const VidsyntHevcSliceSegmentHeader* slice_header = nullptr;
vidsynt_hevc_nalu_get_slice_header(ctx, slice_nalu, &slice_header);
```

### Key Functions

**Context Management:**
- `vidsynt_hevc_context_new()` - Create a new parsing context
- `vidsynt_hevc_context_free()` - Free context and all associated memory
- `vidsynt_hevc_context_set_active_sps()` - Set active SPS for slice parsing
- `vidsynt_hevc_context_set_active_pps()` - Set active PPS for slice parsing

**NAL Unit Parsing:**
- `vidsynt_hevc_parse_nalu_from_bytes()` - Parse a single NAL unit (requires active SPS/PPS for slices)
- `vidsynt_hevc_parse_nalus_annex_b()` - Parse Annex B format (multiple NAL units)

**Parameter Set Access:**
- `vidsynt_hevc_nalu_get_vps()` - Extract VPS from a NAL unit
- `vidsynt_hevc_nalu_get_sps()` - Extract SPS from a NAL unit
- `vidsynt_hevc_nalu_get_pps()` - Extract PPS from a NAL unit

**Slice Access:**
- `vidsynt_hevc_nalu_get_slice_header()` - Extract slice segment header from a VCL NAL unit

**Type Queries:**
- `vidsynt_hevc_nalu_type_is_idr()` - Check if NAL unit is IDR
- `vidsynt_hevc_nalu_type_is_irap()` - Check if NAL unit is IRAP
- `vidsynt_hevc_nalu_type_is_reference()` - Check if NAL unit is a reference picture

**Format Conversion:**
- `vidsynt_hevc_convert_length_prefixed_to_annex_b()` - Convert MP4 to Annex B format

## Troubleshooting

### Library not found

**Error**: `vidsynt library not found`

**Solution**: Make sure you built vidsynt with the ffi feature:
```bash
cargo build --release --features ffi
```

### Header not found

**Error**: `vidsynt.h not found`

**Solution**: Build the vidsynt-sys crate to generate the header:
```bash
cd vidsynt-sys
cargo build
```

### Link errors

**Error**: Undefined references to Rust standard library functions

**Solution**: On Windows, make sure the correct system libraries are linked. The CMakeLists.txt includes:
- ws2_32 (Windows sockets)
- userenv (User environment)
- bcrypt (Cryptography)
- ntdll (NT kernel)

## Advanced Usage

### RAII Wrapper

The example includes a `VidsyntHevcContextGuard` class that provides automatic cleanup:

```cpp
{
    VidsyntHevcContextGuard ctx;
    // Use ctx.get() or implicit conversion to VidsyntHevcContext*
    vidsynt_hevc_parse_nalu_from_bytes(ctx, data, len, &nalu);
    // Automatically freed when ctx goes out of scope
}
```

### Error Handling

All FFI functions return `VidsyntResult`. Check for success:

```cpp
VidsyntResult res = vidsynt_hevc_parse_nalu_from_bytes(ctx, data, len, &nalu);
if (res != VidsyntResult::Success) {
    // Handle error
    return;
}
```

### Memory Management

- All memory is owned by the `VidsyntHevcContext`
- Pointers remain valid until the context is freed
- Do not free individual structures - free the context to clean up everything

## License

Same as the vidsynt library (MIT License).
