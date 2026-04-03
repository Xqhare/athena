//! Generic numeric traits for type-safe arithmetic operations.
//!
//! These traits facilitate writing generic code that can operate on various 
//! integer widths while maintaining type safety and clarity.

/// Traits for signed integer types.
pub mod signed;
/// Traits for unsigned integer types.
pub mod unsigned;

#[cfg(test)]
mod tests;
