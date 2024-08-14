//! FFI bindings to the Daku API.

use core::marker::PhantomData;

use crate::sealed::{Addr, Float, Str};

/// Ready list of size `N`, where `N` must be a power of 2
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ReadyList<const N: usize>(pub [*mut (); N]);

/// Portal IDs
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum Portal {
    /// Logging API (stdout/printf)
    Log = 0x00,
    /// Developer command API (stdin/scanf)
    Prompt = 0x01,
    ///
    Fetch = 0x02,
    ///
    Serve = 0x03,
    ///
    Speakers = 0x04,
    ///
    Microphone = 0x05,
    ///
    Screen = 0x06,
    ///
    Camera = 0x07,
    ///
    Window = 0x08,
    ///
    Spawn = 0x09,
    /// Set user information API (username, display name, localization)
    User = 0x0A,
    /// Get user information API (username, display name, localization)
    Preferences = 0x0B,
    /// Create new users, settings for all users
    System = 0x0C,
    /// Get system information and settings
    About = 0x0D,
    ///
    File = 0x0E,
    ///
    Hid = 0x0F,
    ///
    Timer = 0x10,
    ///
    Clock = 0x11,
    ///
    Gpu = 0x12,
    ///
    Location = 0x13,
}

/// Channel Zero Command
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Connect {
    /// The number of new portals
    pub portals_size: usize,
    /// in: List of new portal IDs - out: List of new portal channel IDs
    pub portals_data: *mut u32,
    /// The capacity of the ready list
    pub ready_capacity: usize,
    /// Reference to uninitialized ready list
    pub ready_data: *mut usize,
}

/// A queued command
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Command {
    /// Data buffer size
    pub size: usize,
    /// Data buffer reference
    pub data: *const (),
    /// Channel id to use
    pub channel: u32,
    /// Ready index for when command completes
    pub ready: usize,
}

#[link(wasm_import_module = "daku")]
extern "C" {
    /// Unsafe asynchronous request FFI call
    pub fn ar(size: usize, data: *const Command) -> usize;
    /// Unstable debug call
    pub fn dbg(size: usize, text: *const u8);
}

/// Log level, pretty much copied from
/// [`log::Level`](https://docs.rs/log/0.4.17/log/enum.Level.html).
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Level {
    /// The “trap” level.
    ///
    /// Trigger a trap without panicking.
    Fail = b'F',
    /// The “error” level.
    ///
    /// Designates very serious errors.
    Error = b'E',
    /// The “warn” level.
    ///
    /// Designates hazardous situations.
    Warn = b'W',
    /// The “info” level.
    ///
    /// Designates useful information.
    Info = b'I',
    /// The “debug” level.
    ///
    /// Designates lower priority information.
    Debug = b'D',
    /// The “trace” level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    Trace = b'T',
}

/// Portal Log Command
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Log {
    /// Log message
    pub message: Text,
    /// Log target
    pub target: Text,
}

/// UTF-8 text
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Text(List<u8>);

impl Text {
    /// Create a new UTF-8 text.
    #[inline(always)]
    pub fn new(str: impl Str) -> Text {
        Self(List::new(str.len(), str.to_addr()))
    }

    /// Get the length of the UTF-8 text in bytes.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Get a pointer to the UTF-8 text's data.
    #[inline(always)]
    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    /// Get a mutable pointer to the UTF-8 text's data.
    #[inline(always)]
    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.0.as_mut_ptr()
    }

    /// Set the length of the UTF-8 text in bytes.
    #[inline(always)]
    pub fn set_len(&mut self, len: usize) {
        self.0.set_len(len);
    }

    /// Set the address of the UTF-8 text.
    #[inline(always)]
    pub fn set_addr(&mut self, addr: impl Addr<u8>) {
        self.0.set_addr(addr);
    }
}

/// Developer console prompt
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Prompt {
    /// Capacity (in/out)
    pub capacity: *mut usize,
    /// Text (in/out)
    pub text: *mut Text,
}

/// Daku list (similar to Rust's slice)
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Default)]
pub struct List<T> {
    size: usize,
    addr: usize,
    data: PhantomData<[T]>,
}

impl<T> List<T> {
    /// Create a new list.
    #[inline(always)]
    pub fn new(size: usize, addr: impl Ptr<T>) -> Self {
        let data = PhantomData;
        let addr = addr.as_usize();

        Self { size, addr, data }
    }

    /// Get the length of the list.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.size
    }

    /// Get a pointer to the list's data.
    #[inline(always)]
    pub fn as_ptr(&self) -> *const T {
        self.addr as *const T
    }

    /// Get a mutable pointer to the list's data.
    #[inline(always)]
    pub fn as_mut_ptr(&self) -> *mut T {
        self.addr as *mut T
    }

    /// Set the length of the list.
    #[inline(always)]
    pub fn set_len(&mut self, len: usize) {
        self.size = len;
    }

    /// Set the address of the list.
    #[inline(always)]
    pub fn set_addr(&mut self, addr: impl Addr<T>) {
        self.addr = addr.as_usize();
    }
}

/// Memory Address (implemented for pointer types)
pub trait Ptr<T>: Addr<T> {}

impl<A, T> Ptr<T> for A where A: Addr<T> {}

/// A 4-dimensional floating-point vector
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vector {
    /// Create a new 4-dimensional vector
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            x: x.clean(),
            y: y.clean(),
            z: z.clean(),
            w: w.clean(),
        }
    }

    /// Get x
    pub fn x(&self) -> f32 {
        self.x
    }

    /// Get y
    pub fn y(&self) -> f32 {
        self.y
    }

    /// Get z
    pub fn z(&self) -> f32 {
        self.z
    }

    /// Get w
    pub fn w(&self) -> f32 {
        self.w
    }

    /// Set x
    pub fn set_x(&mut self, x: f32) {
        self.x = x.clean();
    }

    /// Set y
    pub fn set_y(&mut self, y: f32) {
        self.y = y.clean();
    }

    /// Set z
    pub fn set_z(&mut self, z: f32) {
        self.z = z.clean();
    }

    /// Set w
    pub fn set_w(&mut self, w: f32) {
        self.w = w.clean();
    }
}

/// A mapping of channels to speaker positions
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Positions(List<Vector>);

impl Positions {
    /// Create a positions mapping
    pub fn new(size: usize, addr: impl Ptr<Vector>) -> Self {
        Self(List::new(size, addr))
    }

    /// Get the length of the UTF-8 text in bytes.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Get a pointer to the UTF-8 text's data.
    #[inline(always)]
    pub fn as_ptr(&self) -> *const Vector {
        self.0.as_ptr()
    }

    /// Get a mutable pointer to the UTF-8 text's data.
    #[inline(always)]
    pub fn as_mut_ptr(&self) -> *mut Vector {
        self.0.as_mut_ptr()
    }

    /// Set the length of the UTF-8 text in bytes.
    #[inline(always)]
    pub fn set_len(&mut self, len: usize) {
        self.0.set_len(len);
    }

    /// Set the address of the UTF-8 text.
    #[inline(always)]
    pub fn set_addr(&mut self, addr: impl Addr<Vector>) {
        self.0.set_addr(addr);
    }
}

/// Number (implemented for [`f32`])
pub trait Num: Float {}

impl<T> Num for T where T: Float {}

/// A 2-D integer width/height pair.
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Dimensions {
    width: u16,
    height: u16,
}

impl Dimensions {
    /// Create a new `Dimensions`.
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    /// Get width dimension.
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Get height dimension.
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Set width dimension.
    pub fn set_width(&mut self, width: u16) {
        self.width = width;
    }

    /// Set height dimension.
    pub fn set_height(&mut self, height: u16) {
        self.height = height;
    }
}
