//! Utility functions for interacting with the Win32 API.

// TODO deglob
use crate::platform::win32::ffi::*;
use std::{mem, ptr};

/// Retrieves the base module [`HINSTANCE`].
#[inline]
pub fn base_hinstance() -> HINSTANCE {
    extern "system" {
        // Microsoft's linkers provide a static HINSTANCE to not have to query it at runtime.
        // More info: https://devblogs.microsoft.com/oldnewthing/20041025-00/?p=37483
        static __ImageBase: IMAGE_DOS_HEADER;
    }
    (unsafe { &__ImageBase }) as *const IMAGE_DOS_HEADER as HINSTANCE
}

// (Get/Set)(Class/Window)Long(A/W) all take LONG, a 32-bit type.
// When MS went from 32 to 64 bit, they realized how big of a mistake this was,
// seeing as some of those values need to be as big as a pointer is (like uintptr_t).
// To make things worse, they exported the 32-bit ones on 64-bit with mismatching signatures.
// These functions wrap both of those function sets to `usize`, which matches on 32 & 64 bit.
#[cfg(target_pointer_width = "32")]
#[inline]
pub unsafe fn get_class_data(hwnd: HWND, offset: c_int) -> usize {
    GetClassLongW(hwnd, offset) as usize
}
#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn get_class_data(hwnd: HWND, offset: c_int) -> usize {
    GetClassLongPtrW(hwnd, offset) as usize
}
#[cfg(target_pointer_width = "32")]
#[inline]
pub unsafe fn set_class_data(hwnd: HWND, offset: c_int, data: usize) -> usize {
    SetClassLongW(hwnd, offset, data as LONG) as usize
}
#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn set_class_data(hwnd: HWND, offset: c_int, data: usize) -> usize {
    SetClassLongPtrW(hwnd, offset, data as LONG_PTR) as usize
}
#[cfg(target_pointer_width = "32")]
#[inline]
pub unsafe fn get_window_data(hwnd: HWND, offset: c_int) -> usize {
    GetWindowLongW(hwnd, offset) as usize
}
#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn get_window_data(hwnd: HWND, offset: c_int) -> usize {
    GetWindowLongPtrW(hwnd, offset) as usize
}
#[cfg(target_pointer_width = "32")]
#[inline]
pub unsafe fn set_window_data(hwnd: HWND, offset: c_int, data: usize) -> usize {
    SetWindowLongW(hwnd, offset, data as LONG) as usize
}
#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn set_window_data(hwnd: HWND, offset: c_int, data: usize) -> usize {
    SetWindowLongPtrW(hwnd, offset, data as LONG_PTR) as usize
}

/// Converts a `&str` to an `LPCWSTR` compatible wide string.
///
/// If the length is 0 (aka `*ret == 0x00`) then no allocation was made (it points to a static empty string).
pub fn str_to_wstr(src: &str, buffer: &mut Vec<WCHAR>) -> *const WCHAR {
    // NOTE: Yes, indeed, `std::os::windows::OsStr(ing)ext` does exist in the standard library,
    // but it requires you to fit your data in the OsStr(ing) model and it's not hyper optimized
    // unlike MultiByteToWideChar with (allegedly) handwritten SSE, alongside being the native conversion

    // Always clear the buffer
    buffer.clear();

    unsafe {
        let mut str_san: Option<String> = None;
        let mut str_ptr: *const CHAR = src.as_ptr().cast();
        let mut str_len = src.len();

        // if we have nulls (we don't like that in C) allocate a sanitized copy
        if src.bytes().any(|x| x == 0x00) {
            let san = src.replace('\0', " ");
            str_len = san.len();
            str_ptr = san.as_ptr().cast();
            str_san = Some(san);
        }

        // MultiByteToWideChar can't actually handle a length of 0 because returning 0 means error...
        if src.is_empty() || str_len > c_int::max_value() as usize {
            return [0x00].as_ptr()
        }
        let str_len = str_len as c_int;

        // calculate buffer size
        let req_buffer_size = MultiByteToWideChar(
            CP_UTF8, 0,
            str_ptr, str_len,
            ptr::null_mut(), 0, // `lpWideCharStr == NULL` means query size
        ) as usize + 1; // +1 for null terminator

        // ensure buffer capacity
        buffer.reserve(req_buffer_size);

        // write to our buffer
        let chars_written = MultiByteToWideChar(
            CP_UTF8, 0,
            str_ptr, str_len,
            buffer.as_mut_ptr(), req_buffer_size as c_int,
        ) as usize;

        // drop sanitized buffer, if at all allocated
        mem::drop(str_san);

        // add null terminator & yield
        *buffer.as_mut_ptr().add(chars_written) = 0x00;
        buffer.set_len(req_buffer_size);
        buffer.as_ptr()
    }
}
