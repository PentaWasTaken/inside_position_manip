use std::ffi::{c_void, CString};
use std::mem::{size_of, size_of_val};
use std::ptr;

use winapi::shared::windef::{HWND, LPRECT, RECT};
use winapi::um::dwmapi::{DwmGetWindowAttribute, DWMWA_EXTENDED_FRAME_BOUNDS};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::memoryapi::{ReadProcessMemory, WriteProcessMemory};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::psapi::EnumProcessModules;
use winapi::um::winnt::{HANDLE, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ, PROCESS_VM_WRITE};
use winapi::um::winuser::{FindWindowA, GetForegroundWindow, GetWindowThreadProcessId};

pub struct APIHandle {
    window_handle: HWND,
    handle: HANDLE,
    pub base_address: usize,
}

#[allow(temporary_cstring_as_ptr)]
impl APIHandle {
    pub fn new() -> Result<Self, ()> {
        unsafe {
            let window_handle = FindWindowA(
                ptr::null(),
                CString::new("INSIDE").unwrap().as_ptr() as *const i8,
            );

            let pid = &mut 0u32;
            GetWindowThreadProcessId(window_handle, pid as *mut u32);

            let handle = OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION | PROCESS_VM_WRITE, 0, *pid);

            if handle.is_null() {
                return Err(());
            }

            let mut module_buffer = vec![ptr::null_mut(); 1024];

            EnumProcessModules(
                handle,
                module_buffer.as_mut_ptr(),
                size_of_val(&module_buffer) as u32,
                ptr::null_mut(),
            );

            Ok(APIHandle {
                window_handle,
                handle,
                base_address: module_buffer[0] as usize,
            })
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
            DwmGetWindowAttribute(
                self.window_handle,
                DWMWA_EXTENDED_FRAME_BOUNDS,
                lprect as LPRECT as *mut c_void,
                size_of::<RECT>() as u32,
            );
        }
        *lprect
    }

    pub fn read_memory_f32(&self, offsets: &[usize]) -> f32 {
        let address = self.get_final_address(offsets);
        let bytes = self.read_bytes(address, size_of::<f32>());
        f32::from_le_bytes(bytes)
    }

    pub fn write_memory_f32(&self, offsets: &[usize], val: f32) {
        let address = self.get_final_address(offsets);
        let buffer = val.to_le_bytes();

        println!("{:x}", address);

        unsafe {
            WriteProcessMemory(self.handle, address as *mut _, buffer.as_ptr() as *const _, buffer.len(), ptr::null_mut());
        }
    }

    pub fn get_final_address(&self, offsets: &[usize]) -> usize {
        let mut base = self.base_address;
        for offset in offsets.iter().take(offsets.len() - 1) {
            base += offset;
            let bytes = self.read_bytes(base, size_of::<usize>());
            base = usize::from_le_bytes(bytes);
        }
        base + offsets.get(offsets.len() - 1).unwrap_or(&0)
    }

    pub fn read_bytes<const N: usize>(&self, address: usize, bytes_to_read: usize) -> [u8; N] {
        let mut buffer = [0u8; N];
        unsafe {
            ReadProcessMemory(
                self.handle,
                address as *const c_void,
                buffer.as_mut_ptr() as *mut _,
                bytes_to_read,
                ptr::null_mut(),
            );
        }
        buffer
    }
}
