use std::{
    os::windows::ffi::OsStrExt,
    ptr::{null, null_mut},
};
use std::iter::once;
use winapi::{shared, um::winuser};
use shared::minwindef::{LPARAM, LRESULT, WPARAM};
use winamp_sys;

pub struct Winamp {
    handle: winapi::shared::windef::HWND,
}
impl Winamp {
    pub fn new() -> Option<Self> {
        let string2 = "Winamp v1.x";
        let wide_string2: Vec<u16> = std::ffi::OsStr::new(string2)
            .encode_wide()
            .chain(once(0))
            .collect();

        let handle = unsafe { winuser::FindWindowW(wide_string2.as_ptr(), null()) };
        if handle == null_mut() {
            return None;
        }
        return Some(Winamp { handle });
    }
    pub fn usercommand(&self, data: Option<WPARAM>, id: LPARAM) -> LRESULT {
        unsafe { winuser::SendMessageA(*&self.handle, winuser::WM_USER, data.unwrap_or(0), id) }
    }

    pub fn get_status(&self) -> isize {
        self.usercommand(None, winamp_sys::IPC_ISPLAYING)
    }
    pub fn get_tracklength(&self) -> isize {
        self.usercommand(Some(1), winamp_sys::IPC_GETOUTPUTTIME) * 1000
    }
    pub fn get_elapsedtime(&self) -> isize {
        self.usercommand(None, winamp_sys::IPC_GETOUTPUTTIME)
    }
}