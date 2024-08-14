use alloc::{boxed::Box, vec::Vec};
use core::{any::Any, ptr};

use crate::{sys::ReadyList, tls::Local};

#[export_name = "Rl4"]
static RL4: ReadyListAddr = ReadyListAddr(READY_LIST.as_ptr());

/// Default to 16 commands in the ready list (`Rl4` => 2⁴ = 16)
const READY_LIST_SIZE: usize = 16;

/// How this crate tells which commands are ready
static READY_LIST: Local<ReadyList<READY_LIST_SIZE>> =
    Local::new(ReadyList([ptr::null_mut(); READY_LIST_SIZE]));

/// Active commands as fat pointers
static COMMANDS: Local<Vec<*mut dyn Any>> = Local::new(Vec::new());

struct ReadyListAddr(*mut ReadyList<READY_LIST_SIZE>);

unsafe impl Send for ReadyListAddr {}
unsafe impl Sync for ReadyListAddr {}

/// Take command with matching vtable.
unsafe fn take_command(ptr: *mut ()) -> Option<Box<dyn Any>> {
    COMMANDS.with(|commands| {
        for (i, command) in commands.iter().cloned().enumerate() {
            // Compare without vtable
            let other: *mut () = command.cast();

            if other == ptr {
                commands.swap_remove(i);

                return Some(Box::from_raw(command));
            }
        }

        None
    })
}

/// Process the ready list.
pub(crate) unsafe fn ready_list(count: usize) {
    READY_LIST.with(|ready_list| {
        for cmd in ready_list.0[0..count].iter().cloned() {
            drop(take_command(cmd));
        }
    })
}

/// Append a new active command to be dropped when ready.
///
/// Not all commands need to be dropped when ready, they can be reüsed to reduce
/// allocations.
pub(crate) fn push_command(command: Box<dyn Any>) {
    let command = Box::into_raw(command);

    unsafe { COMMANDS.with(|commands| commands.push(command)) }
}
