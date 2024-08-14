//! Asynchronous APIs

use core::{
    future::Future,
    pin::Pin,
    ptr,
    task::{Context, RawWaker, RawWakerVTable, Waker},
};

use crate::{cmd, portal, tls::Local};

const VTABLE: RawWakerVTable =
    RawWakerVTable::new(clone, wake_any, wake_any, drop);

static ASLEEP: Local<bool> = Local::new(false);

/// Sleep until wake
#[inline(always)]
pub fn sleep() {
    unsafe {
        ASLEEP.with(|asleep| {
            if *asleep {
                cmd::flush();
            } else {
                *asleep = true;
            }
        })
    }
}

/// Wake
#[inline(always)]
pub fn wake() {
    unsafe { ASLEEP.with(|asleep| *asleep = false) }
}

unsafe fn clone(ptr: *const ()) -> RawWaker {
    RawWaker::new(ptr, &VTABLE)
}

unsafe fn wake_any(_: *const ()) {
    wake()
}

unsafe fn drop(_: *const ()) {}

#[inline(always)]
pub(crate) fn new_waker() -> Waker {
    unsafe { Waker::from_raw(clone(ptr::null())) }
}

/// Start execution of a the "main" future
///
/// # Safety
/// Must only be called once at the start of the program.
#[inline(always)]
pub unsafe fn start<F: Future<Output = ()>>(future: F) {
    portal::init();

    let waker = new_waker();
    let mut future = future;
    let mut future = Pin::new_unchecked(&mut future);
    let mut cx = Context::from_waker(&waker);

    while future.as_mut().poll(&mut cx).is_pending() {
        sleep();
    }

    cmd::flush();
}
