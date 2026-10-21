//! Unified traits for numeric operations and random number generation.

/// Generic numeric traits (`Signed`, `Unsigned`) for type-safe arithmetic operations.
#[cfg(any(doc, feature = "numeric_traits"))]
pub mod numeric;

/// Unified API trait for Random Number Generators (`RngApi`).
#[cfg(any(doc, feature = "rng_api"))]
pub mod rng_api;
