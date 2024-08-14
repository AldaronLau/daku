use alloc::vec::Vec;
use core::mem;

use crate::{
    sys::{self, Command, Connect},
    tls::Local,
};

// 1kB list of up to 256 ready channels
#[allow(unused)] // For when no portals are enabled via feature flags
const READY_LIST_CAPACITY: usize = 256;

const PORTAL_LOG: usize = 0;
const PORTAL_PROMPT: usize = PORTAL_LOG + cfg!(feature = "log") as usize;
const PORTAL_COUNT: usize = PORTAL_PROMPT + cfg!(feature = "prompt") as usize;

static PORTALS: Local<[u32; PORTAL_COUNT]> = Local::new([0; PORTAL_COUNT]);

static READY_LIST: Local<Option<Vec<usize>>> = Local::new(None);

#[inline(always)]
pub(crate) unsafe fn ready_list<R>(
    size: usize,
    f: impl FnOnce(&[usize]) -> R,
) -> R {
    READY_LIST.with(|ready_list| {
        let ready_list = ready_list.as_mut().unwrap_unchecked();

        ready_list.set_len(size);
        f(ready_list.as_slice())
    })
}

#[inline(always)]
pub(crate) fn init() {
    unsafe {
        READY_LIST.with(|state| {
            if state.is_some() {
                return;
            };

            PORTALS.with(|p| {
                let mut ready_list = Vec::with_capacity(READY_LIST_CAPACITY);

                #[cfg(feature = "log")]
                {
                    p[PORTAL_LOG] = sys::Portal::Log as u32;
                }
                #[cfg(feature = "prompt")]
                {
                    p[PORTAL_PROMPT] = sys::Portal::Prompt as u32;
                }

                let connect = &Connect {
                    ready_capacity: ready_list.capacity(),
                    ready_data: ready_list.as_mut_ptr(),
                    portals_size: p.len(),
                    portals_data: p.as_mut_ptr(),
                };
                let connect: *const _ = connect;

                let commands = [Command {
                    size: mem::size_of::<Connect>(),
                    data: connect.cast(),
                    // ignored because always immediately ready
                    ready: usize::MAX,
                    channel: 0,
                }];

                sys::ar(commands.len(), commands.as_ptr());

                *state = Some(ready_list);

                // API initialization
                #[cfg(feature = "log")]
                unsafe {
                    crate::api::log::init(p[PORTAL_LOG]);
                }
                #[cfg(feature = "prompt")]
                unsafe {
                    crate::api::prompt::init(p[PORTAL_PROMPT]);
                }
            });
        });
    }
}
