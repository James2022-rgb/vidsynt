# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`vidsynt` is a Rust crate for parsing H.265/HEVC (High Efficiency Video Coding) bitstreams. The crate is designed to extract information from Main/Main10 profile H.265/HEVC bitstreams necessary for decoding using Vulkan Video (`VK_KHR_video_decode_h265`). This is a work-in-progress library focused on video codec syntax parsing.

## Build and Development Commands

```bash
# Build the project
cargo build

# Build with release optimizations
cargo build --release

# Check the project (faster than build, doesn't produce binaries)
cargo check

# Run tests
cargo test

# Run tests with output displayed
cargo test -- --nocapture

# Build documentation
cargo doc --no-deps --open

# Format code (uses rustfmt.toml configuration)
cargo fmt

# Run clippy linter
cargo clippy
```

## Architecture Overview

### Module Structure

The crate is organized into two main modules:

1. **`base` module** (`src/base.rs`): Core bitstream parsing utilities
   - Exponential-Golomb coding functions (`read_exp_golomb_ue`, `read_exp_golomb_se`)
   - EBSP (Encapsulated Byte Sequence Payload) to RBSP (Raw Byte Sequence Payload) conversion
   - These are foundational utilities used throughout the H.265 parsing code

2. **`h265` module** (`src/h265/`): H.265/HEVC-specific parsing implementation
   - Organized by H.265 syntax elements and parameter sets
   - Each submodule corresponds to specific H.265 bitstream structures

### H.265 Module Components

The H.265 module is subdivided into specialized parsing modules:

- **`nalu.rs`**: NAL (Network Abstraction Layer) unit parsing
  - Defines `Nalu`, `NaluHeader`, `NaluType`, and `NaluValue` structures
  - Handles different NAL unit types (VPS, SPS, PPS, coded slices, etc.)
  - Implements EBSP to RBSP conversion within NAL unit parsing
  - Key enum `NaluType` defines all supported NAL unit types with helper methods (`is_irap()`, `is_idr()`, `is_radl()`, etc.)

- **`bytestream.rs`**: Length-prefixed bytestream parsing and Annex B conversion
  - `LengthPrefixedByteStreamContentReader` for parsing NALUs from length-prefixed format
  - Conversion utilities between length-prefixed and Annex B byte stream formats
  - Functions like `parse_nalus_length_prefixed()` and `convert_length_prefixed_to_annex_b()`

- **`vps.rs`**: Video Parameter Set parsing
  - Contains `VideoParameterSet` structure with profile, tier, level, timing info
  - Handles temporal sub-layers and layer sets

- **`sps.rs`**: Sequence Parameter Set parsing
  - `SequenceParameterSet` with picture dimensions, chroma format, bit depth
  - VUI (Video Usability Information) parameters
  - Short-term reference picture sets
  - Profile, tier, and level information

- **`pps.rs`**: Picture Parameter Set parsing
  - `PictureParameterSet` with QP offsets, tiles, deblocking filter control
  - Slice segment header parameters

- **`slice.rs`**: Slice segment header parsing
  - `SliceSegmentLayer` and `SliceSegmentHeader` structures
  - Handles first slice in picture flag, POC (Picture Order Count) values
  - Slice types (I, P, B)
  - Requires `SliceSegmentContext` for parsing (contains references to SPS/PPS parameters)

- **`ptl.rs`**: Profile, Tier, and Level parsing
  - Shared by VPS and SPS
  - `ProfileTierLevel` structure

- **`rps.rs`**: Short-Term Reference Picture Set parsing
  - `ShortTermReferencePictureSet` for managing reference pictures
  - Used in both SPS and slice headers

- **`poc.rs`**: Picture Order Count computation
  - `PocComputer` maintains state across pictures to compute `PicOrderCntVal`
  - Handles IDR pictures, IRAP pictures, and temporal IDs
  - Critical for decoding order and presentation order mapping

- **`nalu_ref.rs`**: NAL unit reference structures
  - Lightweight references to NAL units without full parsing

### Key Dependencies

- **`bitstream-io`**: Bitstream reading/writing at bit-level granularity
  - Used extensively with `BitReader` and `BigEndian` for parsing H.265 syntax elements
- **`tracing`**: Logging framework (with `log` feature enabled)
- **`serde`**: Optional serialization support (enabled by default)

### Parsing Flow

1. **Input**: Raw H.265 bitstream bytes (length-prefixed or Annex B format)
2. **NAL Unit Extraction**: Parse NAL unit headers and identify unit types
3. **EBSP → RBSP**: Remove emulation prevention bytes (0x000003 sequences)
4. **Syntax Element Parsing**: Use bit-level readers to parse structures according to H.265 spec
5. **Parameter Sets**: Extract VPS, SPS, PPS which are referenced by coded slices
6. **Slice Parsing**: Parse slice segment headers which require context from parameter sets
7. **POC Computation**: Use `PocComputer` to determine picture order counts

### Context Requirements

Several parsing operations require context from previously parsed parameter sets:

- **`NaluValueContext`**: Used when parsing NAL units, contains `SliceSegmentContext`
- **`SliceSegmentContext`**: Required for parsing slice segments, contains:
  - Parameters from PPS (`dependent_slice_segments_enabled_flag`, `output_flag_present_flag`, etc.)
  - Parameters from SPS (picture dimensions, coding block sizes, POC parameters)

This means you typically need to parse VPS → SPS → PPS before parsing coded slice segments.

## Code Style Notes

- Uses `rustfmt` with 4-space indentation (not tabs), as configured in `rustfmt.toml`
- Module structure follows H.265 specification sections closely
- Extensive use of `Option<T>` for conditional syntax elements (e.g., fields present based on flags)
- Pattern: Many structures have `from_reader()` or `from_rbsp_reader()` methods for parsing
- Error handling: Uses `io::Error` for parsing errors
- Comments often reference specific sections from the H.265/HEVC specification (e.g., "_7.3.2.1 Video parameter set RBSP syntax_")

## Feature Flags

- **`serde`**: Enabled by default, adds Serialize/Deserialize derives to key structures
- To build without serde: `cargo build --no-default-features`

## Current Limitations and TODOs

Several H.265 features have `todo!()` macros indicating unimplemented functionality:

- VPS: `vps_num_hrd_parameters > 0` not supported
- SPS: `scaling_list_enabled_flag`, `pcm_enabled_flag`, `long_term_ref_pics_present_flag`
- VUI: `overscan_info_present_flag`, `default_display_window_flag`, `vui_hrd_parameters_present_flag`, `bitstream_restriction_flag`

When working on these areas, you'll need to implement the corresponding parsing logic according to the H.265 specification.
