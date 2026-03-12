use super::SchedulerPolicy;
use crate::error::{AthenaError, AthenaResult};

/// Sets the "nice" priority of the current process.
///
/// Common values:
/// - `19`: Lowest priority (most polite)
/// - `0`: Default priority
/// - `-20`: Highest priority (requires root/CAP_SYS_NICE)
pub fn set_nice_value(priority: i32) -> AthenaResult<()> {
    unsafe {
        if libc::setpriority(libc::PRIO_PROCESS, 0, priority) == -1 {
            return Err(AthenaError::IoError(std::io::Error::last_os_error()));
        }
    }
    Ok(())
}

/// Sets the scheduling policy and its priority for the current process.
///
/// Policies like `Fifo` and `RoundRobin` require the process to have
/// the `CAP_SYS_NICE` capability (or run as root).
/// For non-RT policies (`Standard`, `Batch`, `Idle`), the priority should be 0.
pub fn set_scheduler(policy: SchedulerPolicy, priority: i32) -> AthenaResult<()> {
    let libc_policy = match policy {
        SchedulerPolicy::Standard => libc::SCHED_OTHER,
        SchedulerPolicy::Batch => libc::SCHED_BATCH,
        SchedulerPolicy::Idle => libc::SCHED_IDLE,
        SchedulerPolicy::Fifo => libc::SCHED_FIFO,
        SchedulerPolicy::RoundRobin => libc::SCHED_RR,
    };

    let param = libc::sched_param {
        sched_priority: priority,
    };

    unsafe {
        if libc::sched_setscheduler(0, libc_policy, &param) == -1 {
            return Err(AthenaError::IoError(std::io::Error::last_os_error()));
        }
    }
    Ok(())
}
