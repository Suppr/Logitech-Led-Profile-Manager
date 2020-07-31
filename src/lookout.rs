use super::settings::Lookup;
use std::{ffi::OsString, os::windows::ffi::OsStringExt};
use winapi::{shared::windef::HWND, um::winuser};

pub fn get_active(lookup: Option<Lookup>) -> bool {
    if let Some(l) = lookup {
        let handle = unsafe { winuser::GetForegroundWindow() };
        if let Some(rec) = l.reclass {
            let actualclass = get_windowclass(handle);
            if !rec.is_match(&actualclass) {
                return false;
            }
        }
        if let Some(ret) = l.retitle {
            let actualtitle = get_windowtitle(handle);
            if !ret.is_match(&actualtitle) {
                return false;
            }
        }
    } else {
        return false;
    }
    return true;
}

pub fn get_windowtitle(handle: HWND) -> String {
    let length = unsafe { winuser::GetWindowTextLengthW(handle) } + 1;
    let mut lpstring = vec![0; length as usize];
    let i = unsafe { winuser::GetWindowTextW(handle, lpstring.as_mut_ptr(), length) };

    return OsString::from_wide(&lpstring[..i as usize])
        .to_str()
        .unwrap()
        .to_string();
}
pub fn get_windowclass(handle: HWND) -> String {
    let mut lpstring = [0; 50];
    let i = unsafe { winuser::GetClassNameW(handle, lpstring.as_mut_ptr(), 50) };

    return OsString::from_wide(&lpstring[..i as usize])
        .to_str()
        .unwrap()
        .to_string();
}

//mod tests {
//    use super::*;
//    #[test]
//    fn get_active_test() {
//        let a = get_windowclass(get_active());
//        sleep(Duration::from_millis(500));
//        dbg!(a);
//    }
//}
