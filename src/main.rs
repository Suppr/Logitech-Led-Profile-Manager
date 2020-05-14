//#![windows_subsystem = "windows"]

use winamp;
mod keyboard;

fn get_winamptime() -> Option<f64> {
    let winamp1 = winamp::Winamp::new()?;

    if winamp1.get_status() != 1 {
        //get_status return : 0 paused, 1 playing, 2 stop
        return None;
    }
    let currentpos = winamp1.get_elapsedtime();
    let tracklength = winamp1.get_tracklength();
    return Some(currentpos as f64 / tracklength as f64);
}

fn main() {
    let mut logitech = keyboard::Keyboard::new().expect("SDK init fail");

    loop {
        if let Some(percent) = get_winamptime() {
            logitech.progress(percent).expect("Progress bar fail");
        } else {
            logitech.k2000().expect("k2000 fail");
        }
    }
}
