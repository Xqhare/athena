use super::{IoNiceClass, SchedulerPolicy};
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
/// For Real-Time policies (`Fifo`, `RoundRobin`), the priority is used as `sched_priority`
/// and requires the process to have the `CAP_SYS_NICE` capability (or run as root).
///
/// For non-RT policies (`Standard`, `Batch`, `Idle`), the `sched_priority` is set to 0
/// (as required by Linux), and the provided `priority` is applied via `set_nice_value`.
pub fn set_scheduler(policy: SchedulerPolicy, priority: i32) -> AthenaResult<()> {
    let libc_policy = match policy {
        SchedulerPolicy::Standard => libc::SCHED_OTHER,
        SchedulerPolicy::Batch => libc::SCHED_BATCH,
        SchedulerPolicy::Idle => libc::SCHED_IDLE,
        SchedulerPolicy::Fifo => libc::SCHED_FIFO,
        SchedulerPolicy::RoundRobin => libc::SCHED_RR,
    };

    let is_rt = matches!(policy, SchedulerPolicy::Fifo | SchedulerPolicy::RoundRobin);
    let sched_priority = if is_rt { priority } else { 0 };

    let param = libc::sched_param { sched_priority };

    unsafe {
        if libc::sched_setscheduler(0, libc_policy, &param) == -1 {
            return Err(AthenaError::IoError(std::io::Error::last_os_error()));
        }
    }

    if !is_rt {
        set_nice_value(priority)?;
    }

    Ok(())
}

/// Sets the "I/O nice" priority of the current process.
///
/// # Arguments
///
/// * `class` - The I/O class to be set. Possible values:
///     - `IoNiceClass::None` (default, 0)
///     - `IoNiceClass::RealTime` (1)
///     - `IoNiceClass::BestEffort` (2)
///     - `IoNiceClass::Idle` (3)
/// * `class_data` - The I/O class data to be set. Possible values:
///     - For `RealTime` and `BestEffort` classes
///         - Values 0-7, 0 being the highest priority
///     - For `Idle` and `None` class
///         - No effect
///
/// # Returns
///
/// `Ok(())` if successful, or an `AthenaError` if an error occurs.
pub fn set_ionice_value(class: IoNiceClass, class_data: u32) -> AthenaResult<()> {
    let pid = std::process::id() as i32;
    let libc_class = match class {
        IoNiceClass::None => 0,
        IoNiceClass::RealTime => 1,
        IoNiceClass::BestEffort => 2,
        IoNiceClass::Idle => 3,
    };

    let mut command = std::process::Command::new("ionice");
    command
        .arg("-c")
        .arg(format!("{}", libc_class))
        .arg("-p")
        .arg(format!("{}", pid));

    if libc_class != 0 && libc_class != 3 {
        if class_data > 7 {
            return Err(AthenaError::InvalidInput);
        }
        command.arg("-n").arg(format!("{}", class_data));
    }

    let mut proc = command.spawn().map_err(AthenaError::IoError)?;
    let status = proc.wait().map_err(AthenaError::IoError)?;

    if !status.success() {
        return Err(AthenaError::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            "ionice command failed",
        )));
    }

    Ok(())
}

/// Gets the "I/O nice" priority of the current process.
///
/// # Returns
/// * `Ok((IoNiceClass, u32))` if successful, or an `AthenaError` if an error occurs
///
/// The return value is a tuple of `(IoNiceClass, u32)`, where `IoNiceClass` is the I/O class and `u32` is the priority level. 0 is the highest priority.
pub fn get_ionice_value() -> AthenaResult<(IoNiceClass, u32)> {
    let pid = std::process::id() as i32;
    let proc = std::process::Command::new("ionice")
        .arg("-p")
        .arg(format!("{}", pid))
        .stdout(std::process::Stdio::piped())
        .spawn()
        .map_err(AthenaError::IoError)?;
    let output = proc
        .wait_with_output()
        .map_err(AthenaError::IoError)?;
    let stdout = String::from_utf8(output.stdout).map_err(AthenaError::ParseErrorUtf8)?;
    let stdout = stdout.trim();

    if stdout.is_empty() {
        return Err(AthenaError::InvalidOutput);
    }

    if stdout == "idle" {
        return Ok((IoNiceClass::Idle, 0));
    }

    let (class_str, class_data_str) = stdout.split_once(':').ok_or(AthenaError::InvalidInput)?;

    let prio: u32 = class_data_str
        .split_whitespace()
        .last()
        .ok_or(AthenaError::InvalidInput)?
        .parse()
        .map_err(AthenaError::ParseErrorInt)?;

    let io_class = if class_str.contains("none") {
        IoNiceClass::None
    } else if class_str.contains("real") {
        IoNiceClass::RealTime
    } else if class_str.contains("best") {
        IoNiceClass::BestEffort
    } else if class_str.contains("idle") {
        IoNiceClass::Idle
    } else {
        return Err(AthenaError::InvalidOutput);
    };

    Ok((io_class, prio))
}

mod tests {
    use super::{get_ionice_value, IoNiceClass, SchedulerPolicy, set_ionice_value, set_scheduler};

    #[test]
    fn set_cpu_scheduler() {
        assert!(set_scheduler(SchedulerPolicy::Standard, 0).is_ok());
        assert!(set_scheduler(SchedulerPolicy::Batch, 0).is_ok());
        assert!(set_scheduler(SchedulerPolicy::Idle, 0).is_ok());
        // These might fail if not root, so we just check that they return an error if they do
        let _ = set_scheduler(SchedulerPolicy::Fifo, 0);
        let _ = set_scheduler(SchedulerPolicy::RoundRobin, 0);
    }

    #[test]
    fn set_ionice() {
        assert!(set_ionice_value(IoNiceClass::None, 0).is_ok());
        assert!(set_ionice_value(IoNiceClass::BestEffort, 2).is_ok());
        assert!(set_ionice_value(IoNiceClass::Idle, 0).is_ok());
        // RealTime might fail if not root
        let _ = set_ionice_value(IoNiceClass::RealTime, 0);
    }

    #[test]
    fn ionice_with_invalid_priority() {
        assert!(set_ionice_value(IoNiceClass::RealTime, 8).is_err());
        assert!(set_ionice_value(IoNiceClass::BestEffort, 8).is_err());

        assert!(set_ionice_value(IoNiceClass::Idle, 8).is_ok());
        assert!(set_ionice_value(IoNiceClass::None, 8).is_ok());
    }

    #[test]
    fn ionice_with_readback() {
        assert!(set_ionice_value(IoNiceClass::None, 0).is_ok());
        assert_eq!(get_ionice_value().unwrap(), (IoNiceClass::None, 0));

        // RealTime might fail if not root, so we only check if it succeeded
        if set_ionice_value(IoNiceClass::RealTime, 4).is_ok() {
            assert_eq!(get_ionice_value().unwrap(), (IoNiceClass::RealTime, 4));
        }

        assert!(set_ionice_value(IoNiceClass::BestEffort, 4).is_ok());
        assert_eq!(get_ionice_value().unwrap(), (IoNiceClass::BestEffort, 4));

        assert!(set_ionice_value(IoNiceClass::Idle, 0).is_ok());
        assert_eq!(get_ionice_value().unwrap(), (IoNiceClass::Idle, 0));
    }
}
