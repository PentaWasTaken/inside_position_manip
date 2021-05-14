use std::ffi::CString;

use winapi::shared::windef::{HWND, RECT};
use winapi::um::winuser::{FindWindowA, GetWindowRect};

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
        unsafe { GetWindowRect(self.process_handle, lprect) };
        *lprect
    }
}
