//! High-level computational utilities and algorithms.
//!
//! This module provides a set of tools for data processing and system interaction:
//! - Data compression algorithms (`compression`)
//! - Integrity verification tools (`checksum`)
//! - System and process management utilities (`process`)
//! - Dependency resolution and sorting algorithms (`sorting`)
//! - Generic numeric traits (`traits`)

#[cfg(any(doc, feature = "checksum"))]
pub mod checksum;
#[cfg(any(doc, feature = "compression"))]
pub mod compression;
#[cfg(any(doc, feature = "process"))]
pub mod process;
#[cfg(any(doc, feature = "sorting"))]
pub mod sorting;
#[cfg(any(doc, feature = "traits"))]
pub mod traits;
