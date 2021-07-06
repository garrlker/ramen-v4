//! Bindings to the Win32 API.

#![allow(bad_style)]

// c scalars & winapi scalars

pub use core::ffi::c_void;
pub type c_char = i8;
pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_long = i32;
pub type c_ulong = u32;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type wchar_t = u16;

pub type BOOL = c_int;
pub type BYTE = c_uchar;
pub type CHAR = c_char;
pub type DWORD = c_ulong;
pub type INT = c_int;
pub type LONG = c_long;
pub type LONG_PTR = isize;
pub type SHORT = c_short;
pub type UINT = c_uint;
pub type UINT_PTR = usize;
pub type ULONG_PTR = usize;
pub type USHORT = c_ushort;
pub type WCHAR = wchar_t;
pub type WORD = c_ushort;

// opaque types

/// Opaque handle to a module in memory.
pub type HINSTANCE = *mut HINSTANCE__;
pub enum HINSTANCE__ {}

/// Opaque handle to a window.
pub type HWND = *mut HWND__;
pub enum HWND__ {}

// structs

#[repr(C)]
pub struct IMAGE_DOS_HEADER {
    pub e_magic: u16,
    pub e_cblp: u16,
    pub e_cp: u16,
    pub e_crlc: u16,
    pub e_cparhdr: u16,
    pub e_minalloc: u16,
    pub e_maxalloc: u16,
    pub e_ss: u16,
    pub e_sp: u16,
    pub e_csum: u16,
    pub e_ip: u16,
    pub e_cs: u16,
    pub e_lfarlc: u16,
    pub e_ovno: u16,
    pub e_res: [u16; 4],
    pub e_oemid: u16,
    pub e_oeminfo: u16,
    pub e_res2: [u16; 10],
    pub e_lfanew: i32,
}

// constants

pub const CP_UTF8: DWORD = 65001;

#[link(name = "kernel32")]
extern "system" {
    pub fn MultiByteToWideChar(
        CodePage: UINT,
        dwFlags: DWORD,
        lpMultiByteStr: *const CHAR,
        cbMultiByte: c_int,
        lpWideCharStr: *mut WCHAR,
        cchWideChar: c_int,
    ) -> c_int;
}
