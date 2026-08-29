//! Read-path helpers for CX memories.
//!
//! This crate owns memory injection, memory citation parsing, and telemetry
//! classification for read access to the memory folder. It intentionally does
//! not depend on the memory write pipeline.

pub mod citations;
mod metrics;
pub mod usage;

use cx_utils_absolute_path::AbsolutePathBuf;

pub fn memory_root(cx_home: &AbsolutePathBuf) -> AbsolutePathBuf {
    cx_home.join("memories")
}
