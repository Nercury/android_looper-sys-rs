// Copyright 2016 The android_looper_sys Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! <a href="https://github.com/Nercury/android_looper-sys-rs">
//! <img style="position: absolute; top: 0; left: 0; border: 0;" src="https://s3.amazonaws.com/github/ribbons/forkme_left_orange_ff7600.png" alt="Fork me on GitHub">
//! </a>
//! <style>.sidebar { margin-top: 53px }</style>
//!

#![allow(non_camel_case_types)]

extern crate libc;
#[macro_use]
extern crate bitflags;

use libc::{c_int, c_void};

/**
 * ALooper
 *
 * A looper is the state tracking an event loop for a thread.
 * Loopers do not define event structures or other such things; rather
 * they are a lower-level facility to attach one or more discrete objects
 * listening for an event.  An "event" here is simply data available on
 * a file descriptor: each attached object has an associated file descriptor,
 * and waiting for "events" means (internally) polling on all of these file
 * descriptors until one or more of them have data available.
 *
 * A thread can have only one ALooper associated with it.
 */
pub enum ALooper { }

/// Option for for `ALooper_prepare()`.
#[derive(Clone, Copy)]
#[repr(isize)]
pub enum LooperPrepareOpts {
    /// `ALLOW_NON_CALLBACKS`
    ///
    /// Option for ALooper_prepare: this looper will accept calls to
    /// ALooper_addFd() that do not have a callback (that is provide NULL
    /// for the callback).  In this case the caller of ALooper_pollOnce()
    /// or ALooper_pollAll() MUST check the return from these functions to
    /// discover when data is available on such fds and process it.
    AllowNonCallbacks = 1,
    /// `0` value, allow only callbacks
    None = 0,
}

/// Result from `ALooper_pollOnce()` and `ALooper_pollAll()`.
#[derive(Clone, Copy)]
#[repr(isize)]
pub enum LooperPoll {
    /// `ALOOPER_POLL_WAKE`
    ///
    /// The poll was awoken using wake() before the timeout expired
    /// and no callbacks were executed and no other file descriptors were ready.
    Wake = -1,
    /// `ALOOPER_POLL_CALLBACK`
    ///
    /// One or more callbacks were executed.
    Callback = -2,
    /// `ALOOPER_POLL_TIMEOUT`
    ///
    /// The timeout expired.
    Timeout = -3,
    /// `ALOOPER_POLL_ERROR`
    ///
    /// An error occurred.
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

/**
 * For callback-based event loops, this is the prototype of the function
 * that is called when a file descriptor event occurs.
 * It is given the file descriptor it is associated with,
 * a bitmask of the poll events that were triggered (typically ALOOPER_EVENT_INPUT),
 * and the data pointer that was originally supplied.
 *
 * Implementations should return 1 to continue receiving callbacks, or 0
 * to have this file descriptor and callback unregistered from the looper.
 */
pub type ALooper_callbackFunc = ::std::option::Option<unsafe extern "C" fn(fd: c_int,
                                                                             events: c_int,
                                                                             data: *mut c_void)
                                                                             -> c_int>;
extern "C" {

    /// Returns the looper associated with the calling thread, or NULL if
    /// there is not one.
    pub fn ALooper_forThread() -> *mut ALooper;

    /// Prepares a looper associated with the calling thread, and returns it.
    /// If the thread already has a looper, it is returned.  Otherwise, a new
    /// one is created, associated with the thread, and returned.
    ///
    /// The opts may be ALOOPER_PREPARE_ALLOW_NON_CALLBACKS or 0.
    pub fn ALooper_prepare(opts: c_int) -> *mut ALooper;

    /// Acquire a reference on the given ALooper object.  This prevents the object
    /// from being deleted until the reference is removed.  This is only needed
    /// to safely hand an ALooper from one thread to another.
    pub fn ALooper_acquire(looper: *mut ALooper);

    /// Remove a reference that was previously acquired with ALooper_acquire().
    pub fn ALooper_release(looper: *mut ALooper);
    /// Waits for events to be available, with optional timeout in milliseconds.
    /// Invokes callbacks for all file descriptors on which an event occurred.
    ///
    /// If the timeout is zero, returns immediately without blocking.
    /// If the timeout is negative, waits indefinitely until an event appears.
    ///
    /// Returns ALOOPER_POLL_WAKE if the poll was awoken using wake() before
    /// the timeout expired and no callbacks were invoked and no other file
    /// descriptors were ready.
    ///
    /// Returns ALOOPER_POLL_CALLBACK if one or more callbacks were invoked.
    ///
    /// Returns ALOOPER_POLL_TIMEOUT if there was no data before the given
    /// timeout expired.
    ///
    /// Returns ALOOPER_POLL_ERROR if an error occurred.
    ///
    /// Returns a value >= 0 containing an identifier if its file descriptor has data
    /// and it has no callback function (requiring the caller here to handle it).
    /// In this (and only this) case outFd, outEvents and outData will contain the poll
    /// events and data associated with the fd, otherwise they will be set to NULL.
    ///
    /// This method does not return until it has finished invoking the appropriate callbacks
    /// for all file descriptors that were signalled.
    pub fn ALooper_pollOnce(timeoutMillis: c_int,
                            outFd: *mut c_int,
                            outEvents: *mut c_int,
                            outData: *mut *mut c_void)
                            -> c_int;

    /// Like ALooper_pollOnce(), but performs all pending callbacks until all
    /// data has been consumed or a file descriptor is available with no callback.
    /// This function will never return ALOOPER_POLL_CALLBACK.
    pub fn ALooper_pollAll(timeoutMillis: c_int,
                           outFd: *mut c_int,
                           outEvents: *mut c_int,
                           outData: *mut *mut c_void)
                           -> c_int;

    /// Wakes the poll asynchronously.
    ///
    /// This method can be called on any thread.
    /// This method returns immediately.
    pub fn ALooper_wake(looper: *mut ALooper);

    /// Adds a new file descriptor to be polled by the looper.
    /// If the same file descriptor was previously added, it is replaced.
    ///
    /// - "fd" is the file descriptor to be added.
    /// - "ident" is an identifier for this event, which is returned from ALooper_pollOnce().
    /// The identifier must be >= 0, or ALOOPER_POLL_CALLBACK if providing a non-NULL callback.
    /// - "events" are the poll events to wake up on.  Typically this is ALOOPER_EVENT_INPUT.
    /// - "callback" is the function to call when there is an event on the file descriptor.
    /// - "data" is a private data pointer to supply to the callback.
    ///
    /// There are two main uses of this function:
    ///
    /// (1) If "callback" is non-NULL, then this function will be called when there is
    /// data on the file descriptor.  It should execute any events it has pending,
    /// appropriately reading from the file descriptor.  The 'ident' is ignored in this case.
    ///
    /// (2) If "callback" is NULL, the 'ident' will be returned by ALooper_pollOnce
    /// when its file descriptor has data available, requiring the caller to take
    /// care of processing it.
    ///
    /// Returns 1 if the file descriptor was added or -1 if an error occurred.
    ///
    /// This method can be called on any thread.
    /// This method may block briefly if it needs to wake the poll.
    pub fn ALooper_addFd(looper: *mut ALooper,
                         fd: c_int,
                         ident: c_int,
                         events: c_int,
                         callback: ALooper_callbackFunc,
                         data: *mut c_void)
                         -> c_int;

    /// Removes a previously added file descriptor from the looper.
    ///
    /// When this method returns, it is safe to close the file descriptor since the looper
    /// will no longer have a reference to it.  However, it is possible for the callback to
    /// already be running or for it to run one last time if the file descriptor was already
    /// signalled.  Calling code is responsible for ensuring that this case is safely handled.
    /// For example, if the callback takes care of removing itself during its own execution either
    /// by returning 0 or by calling this method, then it can be guaranteed to not be invoked
    /// again at any later time unless registered anew.
    ///
    /// Returns 1 if the file descriptor was removed, 0 if none was previously registered
    /// or -1 if an error occurred.
    ///
    /// This method can be called on any thread.
    ///
    /// This method may block briefly if it needs to wake the poll.
    pub fn ALooper_removeFd(looper: *mut ALooper, fd: c_int) -> c_int;
}
