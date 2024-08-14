//! Rust crate to interface with the [Daku API](https://github.com/ardaku/daku).
//! This crate only works on wasm32 platforms supporting the daku api.
//!
//! # Concurrency model of Daku
//! Daku doesn't use the concept of threads or shared memory.  You can spawn
//! isolated tasks with the API, which can then communicate exclusively over
//! channels.  If a task becomes unresponsive (too long between calls to
//! [`sys::ar()`]), then it will be killed.  For CPU intensive tasks, use
//! `spawn_blocking` (FIXME).

#![no_std]
#![doc(
    html_logo_url = "https://ardaku.github.io/mm/logo.svg",
    html_favicon_url = "https://ardaku.github.io/mm/icon.svg",
    html_root_url = "https://docs.rs/daku"
)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]

// FIXME Don't require target os
#[cfg(not(all(
    target_arch = "wasm32",
    target_endian = "little",
    target_env = "",
    target_family = "wasm",
    target_os = "daku",
    target_pointer_width = "32",
    target_vendor = "unknown",
)))]
compile_error!("Target is not wasm32-daku");

extern crate alloc;

pub mod api;
pub mod cmd;
pub mod run;
pub mod sys;
pub mod tls;

mod global;
mod portal;
mod sealed;

// FIXME: Remove
// pub mod cpu_info;
