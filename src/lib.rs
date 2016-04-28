// Copyright 2016 The android_looper_sys Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![allow(non_camel_case_types)]

extern crate libc;
#[macro_use]
extern crate bitflags;

use libc::{c_int, c_void};

pub enum ALooper { }

/// Option for for `ALooper_prepare()`.
#[derive(Clone, Copy)]
#[repr(isize)]
pub enum LooperPrepareOpts {
    /// `ALLOW_NON_CALLBACKS`
    AllowNonCallbacks = 1,
    /// `0` value
    None = 0,
}

#[derive(Clone, Copy)]
#[repr(isize)]
pub enum ALooperPoll {
    Wake = -1,
    Callback = -2,
    Timeout = -3,
    Error = -4,
}

/// Flags for file descriptor events that a looper can monitor.
///
/// These flag bits can be combined to monitor multiple events at once.
pub mod event {
    bitflags! {
        pub flags Type: isize {
            /// `ALOOPER_EVENT_INPUT`
            ///
            /// The file descriptor is available for read operations.
            const INPUT       = 1 << 0,
            /// `ALOOPER_EVENT_OUTPUT`
            ///
            /// The file descriptor is available for write operations.
            const OUTPUT    = 1 << 1,
            /// `ALOOPER_EVENT_ERROR`
            ///
            /// The file descriptor has encountered an error condition.
            ///
            /// The looper always sends notifications about errors; it is not necessary
            /// to specify this event flag in the requested event set.
            const ERROR   = 1 << 2,
            /// `ALOOPER_EVENT_HANGUP`
            ///
            /// The file descriptor was hung up.
            /// For example, indicates that the remote end of a pipe or socket was closed.
            ///
            /// The looper always sends notifications about hangups; it is not necessary
            /// to specify this event flag in the requested event set.
            const HANGUP   = 1 << 3,
            /// `ALOOPER_EVENT_INVALID`
            ///
            /// The file descriptor is invalid.
            /// For example, the file descriptor was closed prematurely.
            ///
            /// The looper always sends notifications about invalid file descriptors; it is not necessary
            /// to specify this event flag in the requested event set.
            const INVALID       = 1 << 4,
        }
    }
}

pub type ALooper_callbackFunc = ::std::option::Option<unsafe extern "C" fn(fd: c_int,
                                                                             events: c_int,
                                                                             data: *mut c_void)
                                                                             -> c_int>;
extern "C" {
    pub fn ALooper_forThread() -> *mut ALooper;
    pub fn ALooper_prepare(opts: c_int) -> *mut ALooper;
    pub fn ALooper_acquire(looper: *mut ALooper);
    pub fn ALooper_release(looper: *mut ALooper);
    pub fn ALooper_pollOnce(timeoutMillis: c_int,
                            outFd: *mut c_int,
                            outEvents: *mut c_int,
                            outData: *mut *mut c_void)
                            -> c_int;
    pub fn ALooper_pollAll(timeoutMillis: c_int,
                           outFd: *mut c_int,
                           outEvents: *mut c_int,
                           outData: *mut *mut c_void)
                           -> c_int;
    pub fn ALooper_wake(looper: *mut ALooper);
    pub fn ALooper_addFd(looper: *mut ALooper,
                         fd: c_int,
                         ident: c_int,
                         events: c_int,
                         callback: ALooper_callbackFunc,
                         data: *mut c_void)
                         -> c_int;
    pub fn ALooper_removeFd(looper: *mut ALooper, fd: c_int) -> c_int;
}
