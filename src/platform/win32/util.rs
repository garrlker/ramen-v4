//! Utility functions for interacting with the Win32 API.

use crate::platform::win32::ffi::{
    c_int, CHAR, CP_UTF8, HINSTANCE, IMAGE_DOS_HEADER, MultiByteToWideChar, WCHAR,
};
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

/// Converts a `&str` to an `LPCWSTR` compatible wide string.
///
/// If the length is 0 (aka `*ret == 0x00`) then no allocation was made (it points to a static empty string).
pub fn str_to_wstr(src: &str, buffer: &mut Vec<WCHAR>) -> *const WCHAR {
    // NOTE: Yes, indeed, `std::os::windows::OsStr(ing)ext` does exist in the standard library,
    // but it requires you to fit your data in the OsStr(ing) model and it's not hyper optimized
    // unlike MultiByteToWideChar with (allegedly) handwritten SSE, alongside being the native conversion

    // MultiByteToWideChar can't actually handle a length of 0 because returning 0 means error...
    if src.is_empty() || src.len() > c_int::max_value() as usize {
        return [0x00].as_ptr()
    }

    unsafe {
        let mut str_sanitized: Option<String> = None;
        let mut str_ptr: *const CHAR = src.as_ptr().cast();
        let str_len = src.len() as c_int;

        // if we have nulls (we don't like that in C) allocate a sanitized copy
        if src.bytes().any(|x| x == 0x00) {
            let san = src.replace('\0', " ");
            str_ptr = san.as_ptr().cast();
            str_sanitized = Some(san);
        }

        // calculate buffer size
        let req_buffer_size = MultiByteToWideChar(
            CP_UTF8, 0,
            str_ptr, str_len,
            ptr::null_mut(), 0, // `lpWideCharStr == NULL` means query size
        ) as usize + 1; // +1 for null terminator

        // ensure buffer capacity
        buffer.clear();
        buffer.reserve(req_buffer_size);

        // write to our buffer
        let chars_written = MultiByteToWideChar(
            CP_UTF8, 0,
            str_ptr, str_len,
            buffer.as_mut_ptr(), req_buffer_size as c_int,
        ) as usize;

        // drop sanitized buffer, if at all allocated
        mem::drop(str_sanitized);

        // add null terminator & yield
        *buffer.as_mut_ptr().add(chars_written) = 0x00;
        buffer.set_len(req_buffer_size);
        buffer.as_ptr()
    }
}
