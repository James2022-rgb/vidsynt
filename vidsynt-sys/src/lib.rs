//! C bindings for vidsynt
//!
//! This crate re-exports the FFI types and functions from the vidsynt library
//! and generates a C header file using cbindgen.
//!
//! The generated header file can be found at: `include/vidsynt.h`

// Re-export all FFI types and functions
pub use vidsynt::ffi::*;
