use std::{mem::zeroed, os::fd::RawFd};

use libc::{TIOCGWINSZ, ioctl, winsize};

use crate::error::AthenaResult;

/// Get the terminal size in cells from the linux kernel
///
/// Returns a tuple of (rows, columns)
///
/// # Errors
///
/// Any error will be a system level error
///
/// # Unsafe
///
/// Using `libc::ioctl` to get the terminal size from the linux kernel is inherently unsafe
/// OS errors are handled using `std::io::Error::last_os_error`
///
/// # Note
///
/// Code taken from `Talos` https://github.com/Xqhare/talos/blob/728c983f622b16945e0f0316bf386b523ac6383d/src/backend/sys/unix/mod.rs
/// If any bug is found / fixed also fix it in `Talos`
pub fn terminal_size(fd_stdout: RawFd) -> AthenaResult<(u16, u16)> {
    unsafe {
        let mut winsize: winsize = zeroed();
        if ioctl(fd_stdout, TIOCGWINSZ, &mut winsize) != 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        Ok((winsize.ws_row, winsize.ws_col))
    }
}
