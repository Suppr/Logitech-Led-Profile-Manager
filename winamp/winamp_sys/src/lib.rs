#![allow(bad_style)]
use winapi::shared::minwindef::{LPARAM, WPARAM, HINSTANCE, HMODULE};
use winapi::shared::windef::{HWND,HBITMAP,RECT};
use winapi::shared::ntdef::HANDLE;
use winapi::shared::guiddef::GUID;
use winapi::um::winuser::NMHDR;
include!("from_bindgen.rs");
