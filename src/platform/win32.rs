//! Win32-specific definitions and API extensions.

mod ffi;
mod imp;
mod util;

// public re-exports
pub use self::{
    ffi::{HINSTANCE, HWND},
    util::base_hinstance,
};

// platform `imp` glue
pub(crate) use imp::WindowImpl;
