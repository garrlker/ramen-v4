use crate::{
    error::Error,
    platform::win32::{
        // TODO unglob
        ffi::*,
        util,
        WindowBuilderExt,
    },
    sync::{Condvar, LazyCell, Mutex, cvar_notify_one, cvar_wait, mutex_lock},
    window::WindowBuilder,
};
use std::{mem, ptr, sync::Arc, thread};

/* API extensions */

impl WindowBuilderExt for WindowBuilder {
    unsafe fn set_cs_owndc(&mut self, cs_owndc: bool) -> &mut Self {
        self.cs_owndc = cs_owndc;
        self
    }
}

pub(crate) struct WindowImpl {
    thread: Option<thread::JoinHandle<()>>,
}

/// Sent to `thread::spawn` as a nice package.
struct WindowImplThreadParams {
    builder: *const WindowBuilder,
    class_name: *const WCHAR,
    title: *const WCHAR,
    response: Arc<(Condvar, Mutex<Option<Result<WindowImpl, Error>>>)>,
}
unsafe impl Send for WindowImplThreadParams {}

/// Marker value stored in `cbClsExtra` to filter user windows in hooking functions
const RAMEN_WINDOW_MARKER: u32 = u32::from_be_bytes(*b"viri");

impl WindowImpl {
    pub(crate) fn new(builder: &WindowBuilder) -> Result<Self, Error> {
        let response = Arc::new((Condvar::new(), Mutex::new(None)));

        // XXX: no-panic allocator api
        // Allocate these on the main thread to avoid panicking on the window thread.
        let mut buf_class_name = Vec::new();
        let mut buf_title = Vec::new();

        let thread_params = WindowImplThreadParams {
            builder: builder,
            class_name: util::str_to_wstr(builder.class_name.as_ref(), &mut buf_class_name),
            title: util::str_to_wstr(builder.title.as_ref(), &mut buf_title),
            response: Arc::clone(&response),
        };
        let thread = thread::spawn(move || unsafe {
            static CLASS_REGISTRY_LOCK: LazyCell<Mutex<()>> = LazyCell::new(Default::default);

            /* Register the window class (unless it's already been registered) */
            /* A global lock prevents two windows from trying to do it at the same time */
            let mut class_info = mem::MaybeUninit::<WNDCLASSEXW>::uninit();
            let mut class_created_this_thread = false; // see usages below
            let class_registry_lock = mutex_lock(&CLASS_REGISTRY_LOCK);
            (*class_info.as_mut_ptr()).cbSize = mem::size_of_val(&class_info) as DWORD;
            if GetClassInfoExW(
                util::base_hinstance(),
                thread_params.class_name,
                class_info.as_mut_ptr(),
            ) == FALSE {
                // The window class not existing sets the thread global error flag, but it's okay
                SetLastError(ERROR_SUCCESS);

                // Fill in & register class (`cbSize` is set above already)
                let class = &mut *class_info.as_mut_ptr();
                if (&*thread_params.builder).cs_owndc {
                    // See `win32::WindowBuilderExt` for an explanation
                    class.style = CS_OWNDC;
                } else {
                    class.style = 0;
                }
                class.lpfnWndProc = window_proc;
                class.cbClsExtra = mem::size_of::<usize>() as c_int;
                class.cbWndExtra = 0;
                class.hInstance = util::base_hinstance();
                class.hIcon = ptr::null_mut();
                class.hCursor = ptr::null_mut();
                class.hbrBackground = ptr::null_mut();
                class.lpszMenuName = ptr::null_mut();
                // TODO: Filter reserved class names
                class.lpszClassName = thread_params.class_name;
                class.hIconSm = ptr::null_mut();

                // TODO handle properly
                let atom = RegisterClassExW(class);
                assert_ne!(0, atom);
                class_created_this_thread = true;
            }
            mem::drop(class_registry_lock);

            let hwnd: HWND = ptr::null_mut();

            // If we're the thread that created the class, we have to set some properties for it
            // Unfortunately the API doesn't allow you to do this until you have a window handle
            if class_created_this_thread {
                let _ = util::set_class_data(hwnd, 0, RAMEN_WINDOW_MARKER as usize);
            }
        });

        /* Wait for the thread to return the window or an error */
        let (cvar, mutex) = &*response;
        let mut lock = mutex_lock(&mutex);
        loop {
            if let Some(result) = (&mut *lock).take() {
                break result.map(|mut window| {
                    window.thread = Some(thread);
                    window
                })
            } else {
                cvar_wait(&cvar, &mut lock);
            }
        }
    }
}

pub unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    DefWindowProcW(hwnd, msg, wparam, lparam)
}
