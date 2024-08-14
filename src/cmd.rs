//! Task local command queue

use alloc::{boxed::Box, vec::Vec};
use core::{
    any::Any,
    future::Future,
    pin::Pin,
    task::{
        Context,
        Poll::{self, Pending, Ready},
        Waker,
    },
};

use crate::{
    portal, run,
    sys::{self, Command},
    tls::Local,
};

struct State {
    // Task local command queue
    queue: Vec<Command>,
    // Pending wakers
    pending: Vec<Option<Waker>>,
    // Pending drops
    drops: Vec<Box<dyn Any>>,
}

static STATE: Local<State> = Local::new(State {
    queue: Vec::new(),
    pending: Vec::new(),
    drops: Vec::new(),
});

/// Add a mock waker to be replaced
#[inline(always)]
fn add_waker() -> usize {
    unsafe {
        STATE.with(|state| {
            let waker = run::new_waker();

            if let Some(index) = state.pending.iter().position(|w| w.is_none())
            {
                state.pending[index] = Some(waker);
                index
            } else {
                let index = state.pending.len();
                state.pending.push(Some(waker));
                index
            }
        })
    }
}

/// Defer drop until next flush
#[inline(never)]
pub fn defer(mut item: Box<dyn Any>) -> *mut () {
    let ptr: *mut _ = &mut *item;
    unsafe { STATE.with(|state| state.drops.push(item)) };
    ptr.cast()
}

/// Queue a command
///
/// # Safety
/// Command must be valid according to the Daku spec.  Failure to pass in valid
/// `Command` struct may cause undefined behavior.
#[inline(never)]
pub unsafe fn queue(command: Command) {
    STATE.with(|state| state.queue.push(command));
}

/// Flush commands
#[inline(never)]
pub fn flush() {
    unsafe {
        STATE.with(|state| {
            portal::ready_list(
                sys::ar(state.queue.len(), state.queue.as_ptr()),
                |ready_list| {
                    for ready in ready_list {
                        if let Some(waker) = state.pending[*ready].take() {
                            waker.wake();
                        }
                    }
                    state.queue.clear();
                    state.drops.clear();
                },
            )
        });
    }
}

/// Queue and flush
///
/// # Safety
/// Command must be valid according to the Daku spec.  Failure to pass in valid
/// `Command` struct may cause undefined behavior.
pub unsafe fn until(command: Command) {
    queue(command);
    flush();
}

/// Send a command on a channel
///
/// # Safety
/// `data` must be valid according to the Daku spec.  Failure to pass in valid
/// `data` may cause undefined behavior.
#[inline(always)]
pub unsafe fn execute<T>(channel: u32, data: &T) -> impl Future<Output = ()> {
    // Data can't move since it's borrowed
    let data: *const T = data;
    let data = data.cast();
    let size = core::mem::size_of::<T>();

    execute_erased(channel, size, data)
}

#[inline(never)]
unsafe fn execute_erased(
    channel: u32,
    size: usize,
    data: *const (),
) -> impl Future<Output = ()> {
    let ready = add_waker();
    // Queue command and flush
    until(Command {
        ready,
        channel,
        size,
        data,
    });

    // Wait until ready
    Request(ready)
}

// An asynchronous request
struct Request(usize);

impl Future for Request {
    type Output = ();

    #[inline(never)]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe {
            STATE.with(|state| {
                if let Some(ref mut waker) = state.pending[self.0] {
                    *waker = cx.waker().clone();
                    Pending
                } else {
                    Ready(())
                }
            })
        }
    }
}
