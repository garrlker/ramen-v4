//! Bindings to the Win32 API.

#![allow(bad_style)]

/* c scalars & winapi scalars */

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

/* opaque types */

pub type HBRUSH = *mut HBRUSH__;
pub enum HBRUSH__ {}
pub type HCURSOR = HICON;
pub type HHOOK = *mut HHOOK__;
pub enum HHOOK__ {}
pub type HICON = *mut HICON__;
pub enum HICON__ {}
/// Opaque handle to a module in memory.
pub type HINSTANCE = *mut HINSTANCE__;
pub enum HINSTANCE__ {}
pub type HMENU = *mut HMENU__;
pub enum HMENU__ {}
/// Opaque handle to a window.
pub type HWND = *mut HWND__;
pub enum HWND__ {}

/* other winapi types */

pub type ATOM = WORD;
pub type HOOKPROC = unsafe extern "system" fn(c_int, WPARAM, LPARAM) -> LRESULT;
pub type LPARAM = LONG_PTR;
pub type LRESULT = LONG_PTR;
pub type WPARAM = UINT_PTR;

/// A user-defined application window callback function.
pub type WNDPROC = unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT;

/* structs */

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

#[repr(C)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: UINT,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: DWORD,
    pub pt: POINT,
}

#[repr(C)]
pub struct POINT {
    pub x: LONG,
    pub y: LONG,
}

#[repr(C)]
pub struct RECT {
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
}

#[repr(C)]
pub struct WNDCLASSEXW {
    pub cbSize: UINT,
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: c_int,
    pub cbWndExtra: c_int,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: *const WCHAR,
    pub lpszClassName: *const WCHAR,
    pub hIconSm: HICON,
}

/* constants */

pub const CP_UTF8: DWORD = 65001;
pub const CS_OWNDC: UINT = 0x0020;
pub const ERROR_SUCCESS: DWORD = 0; // lol
pub const FALSE: BOOL = 0;
pub const GCL_CBCLSEXTRA: c_int = -20;
pub const HCBT_DESTROYWND: c_int = 4;
pub const WH_CBT: c_int = 5;

/* static linked functions */

#[link(name = "kernel32")]
extern "system" {
    pub fn GetLastError() -> DWORD;
    pub fn SetLastError(dwErrCode: DWORD);

    pub fn GetCurrentThreadId() -> DWORD;

    pub fn MultiByteToWideChar(
        CodePage: UINT,
        dwFlags: DWORD,
        lpMultiByteStr: *const CHAR,
        cbMultiByte: c_int,
        lpWideCharStr: *mut WCHAR,
        cchWideChar: c_int,
    ) -> c_int;
}

#[link(name = "user32")]
extern "system" {
    // Window creation
    pub fn GetClassInfoExW(hinst: HINSTANCE, lpszClass: *const WCHAR, lpwcx: *mut WNDCLASSEXW) -> BOOL;
    pub fn RegisterClassExW(lpWndClass: *const WNDCLASSEXW) -> ATOM;

    // Window management
    pub fn CreateWindowExW(
        dwExStyle: DWORD,
        lpClassName: *const WCHAR,
        lpWindowName: *const WCHAR,
        dwStyle: DWORD,
        x: c_int,
        y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: *mut c_void,
    ) -> HWND;
    pub fn SetWindowPos(
        hWnd: HWND,
        hWndInsertAfter: HWND,
        X: c_int,
        Y: c_int,
        cx: c_int,
        cy: c_int,
        uFlags: UINT,
    ) -> BOOL;
    pub fn AdjustWindowRectEx(lpRect: *mut RECT, dwStyle: DWORD, bMenu: BOOL, dwExStyle: DWORD) -> BOOL;
    pub fn ClientToScreen(hWnd: HWND, lpPoint: *mut POINT) -> BOOL;
    pub fn GetClientRect(hWnd: HWND, lpRect: *mut RECT) -> BOOL;
    pub fn GetWindowRect(hWnd: HWND, lpRect: *mut RECT) -> BOOL;
    // pub fn GetTitleBarInfo(hwnd: HWND, pti: *mut TITLEBARINFO) -> BOOL;
    pub fn WindowFromPoint(Point: POINT) -> HWND;
    pub fn DestroyWindow(hWnd: HWND) -> BOOL;

    // Window storage manipulation
    pub fn GetClassLongW(hWnd: HWND, nIndex: c_int) -> DWORD;
    pub fn SetClassLongW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG) -> DWORD;
    pub fn GetWindowLongW(hWnd: HWND, nIndex: c_int) -> LONG;
    pub fn SetWindowLongW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG) -> LONG;
    #[cfg(target_pointer_width = "64")]
    pub fn GetClassLongPtrW(hWnd: HWND, nIndex: c_int) -> ULONG_PTR;
    #[cfg(target_pointer_width = "64")]
    pub fn SetClassLongPtrW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR) -> ULONG_PTR;
    #[cfg(target_pointer_width = "64")]
    pub fn GetWindowLongPtrW(hWnd: HWND, nIndex: c_int) -> LONG_PTR;
    #[cfg(target_pointer_width = "64")]
    pub fn SetWindowLongPtrW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR) -> LONG_PTR;

    // Window message loop
    pub fn GetMessageW(lpMsg: *mut MSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> BOOL;
    pub fn PostMessageW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> BOOL;
    pub fn SendMessageW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn DefWindowProcW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn DispatchMessageW(lpmsg: *const MSG) -> LRESULT;
    pub fn PostQuitMessage(nExitCode: c_int);

    // Window message hooking api
    pub fn CallNextHookEx(hhk: HHOOK, nCode: c_int, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn SetWindowsHookExW(idHook: c_int, lpfn: HOOKPROC, hmod: HINSTANCE, dwThreadId: DWORD) -> HHOOK;
    pub fn UnhookWindowsHookEx(hhk: HHOOK) -> BOOL;
}
