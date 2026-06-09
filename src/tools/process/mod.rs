//! UNIX-specific process and system monitoring utilities.
//!
//! This module provides tools for managing process priorities, CPU scheduling, 
//! and I/O scheduling on UNIX-like systems.

#[cfg(not(unix))]
use crate::error::{AthenaError, AthenaResult};

#[cfg(unix)]
mod unix;

#[cfg(unix)]
pub use unix::*;

/// Represents the available Linux/Unix scheduling policies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulerPolicy {
    /// The default interactive policy (`SCHED_OTHER`).
    Standard,
    /// For CPU-intensive background tasks (`SCHED_BATCH`).
    Batch,
    /// Very low priority, only runs when system is idle (`SCHED_IDLE`).
    Idle,
    /// Real-time First-In-First-Out (`SCHED_FIFO`). Requires special privileges.
    Fifo,
    /// Real-time Round-Robin (`SCHED_RR`). Requires special privileges.
    RoundRobin,
}

/// Represents the available Linux/Unix "I/O nice" classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoNiceClass {
    /// Real-time class. For I/O tasks that need to be executed as soon as possible.
    RealTime,
    /// Best-effort class. For I/O tasks that don't need to be executed as soon as possible.
    BestEffort,
    /// Idle class. For I/O tasks that don't need to be executed if any other I/O task is running.
    Idle,
    /// The default class (none).
    None,
}

#[cfg(not(unix))]
/// Set the "nice" value of the current process (unsupported on this platform).
pub fn set_nice_value(_priority: i32) -> AthenaResult<()> {
    Err(AthenaError::UnsupportedPlatform)
}

#[cfg(not(unix))]
/// Set the scheduler policy and priority (unsupported on this platform).
pub fn set_scheduler(_policy: SchedulerPolicy, _priority: i32) -> AthenaResult<()> {
    Err(AthenaError::UnsupportedPlatform)
}

#[cfg(not(unix))]
/// Set the I/O priority class and data (unsupported on this platform).
pub fn set_ionice_value(_class: IoNiceClass, _class_data: u32) -> AthenaResult<()> {
    Err(AthenaError::UnsupportedPlatform)
}
