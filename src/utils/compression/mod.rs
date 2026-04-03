//! High-performance data compression algorithms.
//!
//! This module contains implementations of standard and custom compression techniques
//! optimized for the XFF ecosystem, including:
//! - Lempel-Ziv-Welch (LZW)
//! - Combined Delta and Run-Length Encoding

pub mod delta_with_run_length;
pub mod lzw;
