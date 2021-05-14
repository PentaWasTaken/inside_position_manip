use std::ffi::{CString, c_void};
use std::mem::size_of;

use winapi::shared::windef::{HWND, RECT, LPRECT};
use winapi::um::winuser::{FindWindowA, GetWindowRect};
use winapi::um::dwmapi::{DwmGetWindowAttribute, DWMWA_EXTENDED_FRAME_BOUNDS};

pub struct APIHandle {
    process_handle: HWND,
}

impl APIHandle {
    pub fn new() -> Result<Self, ()> {
        unsafe {
            let process_handle = FindWindowA(
                std::ptr::null(),
                CString::new("INSIDE").unwrap().as_ptr() as *const i8,
            );

            if process_handle.is_null() {
                return Err(());
            }

            Ok(APIHandle { process_handle })
        }
    }

    pub fn get_win_rect(&self) -> RECT {
        let lprect = &mut RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        unsafe {
            DwmGetWindowAttribute(self.process_handle, DWMWA_EXTENDED_FRAME_BOUNDS, lprect as LPRECT as *mut c_void, size_of::<RECT>() as u32);
        }
        *lprect
    }
}
