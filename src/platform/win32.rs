//! Win32-specific definitions and API extensions.

mod ffi;
mod imp;
mod util;

// public re-exports
pub use self::{
    ffi::{HINSTANCE, HWND, WNDPROC},
    util::base_hinstance,
};

// platform `imp` glue
pub(crate) use imp::WindowImpl;

/// Win32-specific API extensions to [`WindowBuilder`](crate::window::WindowBuilder).
pub trait WindowBuilderExt {
    /// Sets `CS_OWNDC` for the window class, making it so a unique device context
    /// is created for every window instantiated with this window class.
    ///
    /// This function is `unsafe` because if the class was already registered, this does nothing,
    /// therefore you should be consistent with this when re-using the same window class name.
    ///
    /// More info: https://devblogs.microsoft.com/oldnewthing/20060601-06/?p=31003
    ///
    /// Defaults to `true`.
    unsafe fn set_cs_owndc(&mut self, cs_owndc: bool) -> &mut Self;
}
