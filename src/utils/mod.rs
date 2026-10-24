//! High-level computational utilities and algorithms.
//!
//! This module provides a set of tools for data processing and system interaction:
//! - Data compression algorithms (`compression`)
//! - Integrity verification tools (`checksum`)
//! - System and process management utilities (`process`)
//! - Dependency resolution and sorting algorithms (`sorting`)
//! - Unified numeric and RNG traits (`traits`)
//! - Generational arena / slot map (`slot_map`)

#[cfg(any(doc, feature = "checksum"))]
pub mod checksum;
#[cfg(any(doc, feature = "compression"))]
pub mod compression;
#[cfg(any(doc, feature = "slot_map"))]
pub mod slot_map;
#[cfg(any(doc, feature = "sorting"))]
pub mod sorting;
#[cfg(any(
    doc,
    feature = "traits",
    feature = "numeric_traits",
    feature = "rng_api"
))]
pub mod traits;
