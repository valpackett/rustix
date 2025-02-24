//! Functions returning the stdio file descriptors.
//!
//! # Safety
//!
//! These access the file descriptors by absolute index value, and nothing
//! prevents them from being closed and reused. They should only be used in
//! `main` or other situations where one is in control of the process'
//! stdio streams.
#![allow(unsafe_code)]

use crate::backend;
use crate::fd::OwnedFd;
use backend::fd::{BorrowedFd, FromRawFd, RawFd};

/// `STDIN_FILENO`—Standard input, borrowed.
///
/// In `std`-using configurations, this is a safe function, because the
/// standard library already assumes that the stdin file descriptor is always
/// valid. In `no_std` configurations, it is `unsafe`.
///
/// # Warning
///
/// This function allows reading directly from stdin without coordinating
/// with the buffering performed by [`std::io::Stdin`], so it could cause
/// corrupted input.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stdin.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stdin.3.html
#[cfg(feature = "std")]
#[doc(alias = "STDIN_FILENO")]
#[inline]
pub const fn stdin() -> BorrowedFd<'static> {
    // Safety: When "std" is enabled, the standard library assumes that the stdio
    // file descriptors are all valid.
    unsafe { BorrowedFd::borrow_raw(backend::io::types::STDIN_FILENO as RawFd) }
}

/// `STDIN_FILENO`—Standard input, borrowed.
///
/// In `std`-using configurations, this is a safe function, because the
/// standard library already assumes that the stdin file descriptor is always
/// valid. In `no_std` configurations, it is `unsafe`.
///
/// # Safety
///
/// In `no_std` configurations, the stdin file descriptor can be closed,
/// potentially on other threads, in which case the file descriptor index
/// value could be dynamically reused for other purposes, potentially on
/// different threads.
///
/// # Warning
///
/// This function allows reading directly from stdin without coordinating
/// with the buffering performed by [`std::io::Stdin`], so it could cause
/// corrupted input.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stdin.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stdin.3.html
#[cfg(not(feature = "std"))]
#[doc(alias = "STDIN_FILENO")]
#[inline]
pub const unsafe fn stdin() -> BorrowedFd<'static> {
    BorrowedFd::borrow_raw(backend::io::types::STDIN_FILENO as RawFd)
}

/// `STDIN_FILENO`—Standard input, owned.
///
/// This is similar to [`stdin`], however it returns an `OwnedFd` which closes
/// standard input when it is dropped.
///
/// # Safety
///
/// Safe `std`-using Rust code is permitted to assume that the stdin file
/// descriptor is always valid. This function returns an `OwnedFd` which will
/// close the stdin file descriptor when dropped.
///
/// # Warning
///
/// This has the same hazards as [`stdin`].
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stdin.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stdin.3.html
#[doc(alias = "STDIN_FILENO")]
#[inline]
pub unsafe fn take_stdin() -> OwnedFd {
    backend::fd::OwnedFd::from_raw_fd(backend::io::types::STDIN_FILENO as RawFd)
}

/// `STDOUT_FILENO`—Standard output, borrowed.
///
/// In `std`-using configurations, this is a safe function, because the
/// standard library already assumes that the stdout file descriptor is always
/// valid. In `no_std` configurations, it is `unsafe`.
///
/// # Warning
///
/// This function allows reading directly from stdout without coordinating
/// with the buffering performed by [`std::io::Stdout`], so it could cause
/// corrupted input.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stdout.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stdout.3.html
#[cfg(feature = "std")]
#[doc(alias = "STDOUT_FILENO")]
#[inline]
pub const fn stdout() -> BorrowedFd<'static> {
    // Safety: When "std" is enabled, the standard library assumes that the stdio
    // file descriptors are all valid.
    unsafe { BorrowedFd::borrow_raw(backend::io::types::STDOUT_FILENO as RawFd) }
}

/// `STDOUT_FILENO`—Standard output, borrowed.
///
/// In `std`-using configurations, this is a safe function, because the
/// standard library already assumes that the stdin file descriptor is always
/// valid. In `no_std` configurations, it is `unsafe`.
///
/// # Safety
///
/// In `no_std` configurations, the stdout file descriptor can be closed,
/// potentially on other threads, in which case the file descriptor index
/// value could be dynamically reused for other purposes, potentially on
/// different threads.
///
/// # Warning
///
/// This function allows reading directly from stdout without coordinating
/// with the buffering performed by [`std::io::Stdout`], so it could cause
/// corrupted input.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stdout.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stdout.3.html
#[cfg(not(feature = "std"))]
#[doc(alias = "STDOUT_FILENO")]
#[inline]
pub const unsafe fn stdout() -> BorrowedFd<'static> {
    BorrowedFd::borrow_raw(backend::io::types::STDOUT_FILENO as RawFd)
}

/// `STDOUT_FILENO`—Standard output, owned.
///
/// This is similar to [`stdout`], however it returns an `OwnedFd` which closes
/// standard output when it is dropped.
///
/// # Safety
///
/// Safe `std`-using Rust code is permitted to assume that the stdout file
/// descriptor is always valid. This function returns an `OwnedFd` which will
/// close the stdout file descriptor when dropped.
///
/// # Warning
///
/// This has the same hazards as [`stdout`].
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stdout.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stdout.3.html
#[doc(alias = "STDOUT_FILENO")]
#[inline]
pub unsafe fn take_stdout() -> OwnedFd {
    backend::fd::OwnedFd::from_raw_fd(backend::io::types::STDOUT_FILENO as RawFd)
}

/// `STDERR_FILENO`—Standard error, borrowed.
///
/// In `std`-using configurations, this is a safe function, because the
/// standard library already assumes that the stderr file descriptor is always
/// valid. In `no_std` configurations, it is `unsafe`.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stderr.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stderr.3.html
#[cfg(feature = "std")]
#[doc(alias = "STDERR_FILENO")]
#[inline]
pub const fn stderr() -> BorrowedFd<'static> {
    // Safety: When "std" is enabled, the standard library assumes that the stdio
    // file descriptors are all valid.
    unsafe { BorrowedFd::borrow_raw(backend::io::types::STDERR_FILENO as RawFd) }
}

/// `STDERR_FILENO`—Standard error, borrowed.
///
/// In `std`-using configurations, this is a safe function, because the
/// standard library already assumes that the stderr file descriptor is always
/// valid. In `no_std` configurations, it is `unsafe`.
///
/// # Safety
///
/// In `no_std` configurations, the stderr file descriptor can be closed,
/// potentially on other threads, in which case the file descriptor index
/// value could be dynamically reused for other purposes, potentially on
/// different threads.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stderr.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stderr.3.html
#[cfg(not(feature = "std"))]
#[doc(alias = "STDERR_FILENO")]
#[inline]
pub const unsafe fn stderr() -> BorrowedFd<'static> {
    BorrowedFd::borrow_raw(backend::io::types::STDERR_FILENO as RawFd)
}

/// `STDERR_FILENO`—Standard error, owned.
///
/// This is similar to [`stdout`], however it returns an `OwnedFd` which closes
/// standard output when it is dropped.
///
/// # Safety
///
/// Safe std-using Rust code is permitted to assume that the stderr file
/// descriptor is always valid. This function returns an `OwnedFd` which will
/// close the stderr file descriptor when dropped.
///
/// # Other hazards
///
/// This has the same hazards as [`stderr`].
///
/// And, when the `OwnedFd` is dropped, subsequent newly created file
/// descriptors may unknowingly reuse the stderr file descriptor number, which
/// may break common assumptions, so it should typically only be dropped at the
/// end of a program when no more file descriptors will be created.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stderr.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stderr.3.html
#[doc(alias = "STDERR_FILENO")]
#[inline]
pub unsafe fn take_stderr() -> OwnedFd {
    backend::fd::OwnedFd::from_raw_fd(backend::io::types::STDERR_FILENO as RawFd)
}

/// `STDIN_FILENO`—Standard input, raw.
///
/// This is similar to [`stdin`], however it returns a `RawFd`.
///
/// # Other hazards
///
/// This has the same hazards as [`stdin`].
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stdin.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stdin.3.html
#[doc(alias = "STDIN_FILENO")]
#[inline]
pub const fn raw_stdin() -> RawFd {
    backend::io::types::STDIN_FILENO as RawFd
}

/// `STDOUT_FILENO`—Standard output, raw.
///
/// This is similar to [`stdout`], however it returns a `RawFd`.
///
/// # Other hazards
///
/// This has the same hazards as [`stdout`].
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stdout.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stdout.3.html
#[doc(alias = "STDOUT_FILENO")]
#[inline]
pub const fn raw_stdout() -> RawFd {
    backend::io::types::STDOUT_FILENO as RawFd
}

/// `STDERR_FILENO`—Standard error, raw.
///
/// This is similar to [`stderr`], however it returns a `RawFd`.
///
/// # Other hazards
///
/// This has the same hazards as [`stderr`].
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stderr.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stderr.3.html
#[doc(alias = "STDERR_FILENO")]
#[inline]
pub const fn raw_stderr() -> RawFd {
    backend::io::types::STDERR_FILENO as RawFd
}
