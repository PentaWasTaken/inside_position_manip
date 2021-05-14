use std::ffi::{c_void, CString};

use winapi::shared::minwindef::DWORD;
use winapi::shared::windef::{HWND, RECT};
use winapi::um::{
    self, handleapi::GetHandleInformation, processthreadsapi::GetProcessId, winnt::HANDLE,
    winuser::{FindWindowA, GetWindowThreadProcessId},
};

pub struct APIHandle {
    process_handle: HWND,
}

impl APIHandle {
    pub fn new() -> Result<Self, ()> {
        unsafe {
            let process_handle = um::winuser::FindWindowA(
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
        unsafe { um::winuser::GetWindowRect(self.process_handle, lprect) };
        *lprect
    }
}
