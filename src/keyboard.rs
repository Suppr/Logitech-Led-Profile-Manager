extern crate logitech_led as led;

use led::{Color, Driver, Key};
use std::thread::sleep;
use std::time::Duration;

fn apply_alpha(basecolor: Color, alpha: f32) -> Color {
    if (0.0 > alpha) || (1.0 < alpha) {
        return basecolor;
    }
    Color::new(
        basecolor.r * alpha,
        basecolor.g * alpha,
        basecolor.b * alpha,
    )
}
pub struct Keyboard {
    driver: led::Driver,
    pb_keys: Vec<Key>,
    np_basecolor: Color,
    np_pulsecolor: Color,
    pb_delimitercolor: Color,
    pb_basecolor: Color,
    pb_barcolor: Color,
    refreshtimer: u64,
}
impl Keyboard {
    pub fn new() -> Result<Self, logitech_led::Error> {
        let mut driver = Driver::init()?;
        sleep(Duration::from_millis(100));
        let refreshtimer =
            driver.config_option_num("Global config/Refresh (in second)", 1.0)? as u64;
        let np_basecolor =
            driver.config_option_color("No progression/Base color", Color::new(0.0, 0.0, 0.0))?;
        let np_pulsecolor =
            driver.config_option_color("No progression/Pulse color", Color::new(1.0, 0.0, 0.0))?;
        let pb_delimitercolor = driver
            .config_option_color("Progress bar/Delimiter color", Color::new(0.0, 1.0, 0.0))?;
        let pb_basecolor =
            driver.config_option_color("Progress bar/Base color", Color::new(0.0, 0.0, 0.0))?;
        let pb_barcolor =
            driver.config_option_color("Progress bar/Bar color", Color::new(0.0, 0.0, 1.0))?;
        let pb_keys = vec![
            Key::ESC,
            Key::F1,
            Key::F2,
            Key::F3,
            Key::F4,
            Key::F5,
            Key::F6,
            Key::F7,
            Key::F8,
            Key::F9,
            Key::F10,
            Key::F11,
            Key::F12,
            Key::PRINT_SCREEN,
            Key::SCROLL_LOCK,
            Key::PAUSE_BREAK,
        ];
        return Ok(Keyboard {
            driver,
            pb_keys,
            np_basecolor,
            np_pulsecolor,
            pb_delimitercolor,
            pb_basecolor,
            pb_barcolor,
            refreshtimer,
        });
    }
    pub fn k2000(&mut self) -> Result<(), logitech_led::Error> {
        for onkey in self.pb_keys.iter() {
            self.driver
                .set_lighting_for_key(*onkey, self.np_basecolor)?;
        }
        sleep(Duration::from_secs(self.refreshtimer));
        for onekey in self.pb_keys.iter() {
            self.driver.pulse_single_key(
                *onekey,
                self.np_basecolor,
                self.np_pulsecolor,
                Duration::from_millis(1000),
                false,
            )?;
            sleep(Duration::from_millis(100));
        }
        for onekey in self.pb_keys.iter().rev() {
            self.driver.pulse_single_key(
                *onekey,
                self.np_basecolor,
                self.np_pulsecolor,
                Duration::from_millis(1000),
                false,
            )?;
            sleep(Duration::from_millis(100));
        }
        sleep(Duration::from_secs(self.refreshtimer));
        return Ok(());
    }
    pub fn progress(&mut self, percent: f64) -> Result<(), logitech_led::Error> {
        self.driver
            .set_lighting_for_key(self.pb_keys[0], self.pb_delimitercolor)?;
        for i in 1..self.pb_keys.len() - 1 {
            self.driver
                .set_lighting_for_key(self.pb_keys[i], self.pb_basecolor)?;
        }
        self.driver
            .set_lighting_for_key(self.pb_keys[self.pb_keys.len() - 1], self.pb_delimitercolor)?;

        let value = percent * (self.pb_keys.len() - 2) as f64;// - 0.5 + 1.0;
        let alpha = (value - (value.floor())) as f32;

        let mut i = 1;
        while i <= (value.floor() as usize) {
            self.driver
                .set_lighting_for_key(self.pb_keys[i], self.pb_barcolor)?;
            i += 1;
        }
        let lastc = apply_alpha(self.pb_barcolor, alpha);

        let i = std::cmp::min(i, self.pb_keys.len() - 2);
        self.driver.set_lighting_for_key(self.pb_keys[i], lastc)?;
        sleep(Duration::from_millis(100));
        return Ok(());
    }
}
