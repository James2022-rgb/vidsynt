//! C FFI (Foreign Function Interface) for vidsynt
//!
//! This module provides a C-compatible API for parsing H.265/HEVC bitstreams.
//! All symbols are prefixed with `vidsynt_hevc_` to allow for future H.264 support.
//!
//! # Memory Management
//!
//! All memory is managed through a context object (`VidsyntHevcContext`). When you create
//! a context with `vidsynt_hevc_context_new()`, all subsequent parsing operations
//! allocate memory that is owned by that context. Free all memory at once by calling
//! `vidsynt_hevc_context_free()`.
//!
//! # Error Handling
//!
//! All functions return a `VidsyntResult` enum. Check for `VIDSYNT_SUCCESS` before
//! using output parameters.

pub mod context;
pub mod types;
pub mod nalu;
pub mod vps;
pub mod sps;
pub mod pps;
pub mod slice;
pub mod poc;
pub mod convert;

// Re-export key types for convenience
pub use context::{vidsynt_hevc_context_new, vidsynt_hevc_context_free, VidsyntHevcContext};
pub use types::*;
pub use nalu::*;
pub use vps::*;
pub use sps::*;
pub use pps::*;
pub use slice::*;
pub use poc::*;
pub use convert::*;
