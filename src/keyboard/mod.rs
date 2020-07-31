use super::plugins::{RenderColor, TimedEffect};
use super::{led, Color};
use led::Driver;
use std::collections::hash_map::HashMap;
use std::thread::sleep;
use std::time::Duration;

pub mod layout;
use layout::Key;

#[derive(Clone, Copy)]
pub struct Bitmap {
    pub bitmap: [u8; led::BITMAP_SIZE],
}

impl Bitmap {
    pub fn new() -> Self {
        Self {
            bitmap: [0u8; led::BITMAP_SIZE],
        }
    }
    pub fn set_bitmapkeycolor(&mut self, keyname: Key, color: &Color) {
        let tempc = color.to_rgba();
        let c1 = [tempc.b, tempc.g, tempc.r, (tempc.alpha * 255.0) as u8];
        let key = Key::from(keyname);
        return self.bitmap[(key as usize * led::BITMAP_BYTES_PER_KEY)
            ..(key as usize * led::BITMAP_BYTES_PER_KEY + led::BITMAP_BYTES_PER_KEY)]
            .copy_from_slice(&c1);
    }
}

pub struct BufferKeysColors {
    mkeyscolors: HashMap<Key, Color>,
}
impl BufferKeysColors {
    pub fn new() -> Self {
        Self {
            mkeyscolors: HashMap::new(),
        }
    }
    pub fn update(&mut self, map: HashMap<Key, Color>) {
        self.mkeyscolors.extend(map);
    }
    pub fn get_keycolor(&self, key: &Key) -> Color {
        if self.mkeyscolors.contains_key(key) {
            return self.mkeyscolors[&key].clone();
        } else {
            return Color::black();
        }
    }
    pub fn run_timedeffect<T: RenderColor + TimedEffect>(&mut self, effect: &mut T) {
        let elapsed = effect.elapsed();
        if elapsed > effect.get_endtime() && effect.is_infinite() {
            effect.reset();
        }
        return self.run_staticeffect(effect);
    }
    pub fn run_staticeffect<T: RenderColor>(&mut self, effect: &mut T) {
        effect.render_colors(self)
    }
}

pub struct Logitech {
    pub driver: led::Driver,
    pub np_basecolor: Color,
    pub np_pulsecolor: Color,
    pub pb_delimitercolor: Color,
    pub pb_basecolor: Color,
    pub pb_barcolor: Color,
    pub refreshtimer: u64,
}
impl Logitech {
    pub fn new() -> Result<Self, logitech_led::Error> {
        let mut driver = Driver::init()?;
        sleep(Duration::from_millis(100));
        //a supprimer
        let refreshtimer =
            driver.config_option_num("Global config/Refresh (in second)", 1.0)? as u64;
        let np_bc = led::BGRA::from(
            driver
                .config_option_color("No progression/Base color", led::Color::new(0.0, 0.0, 0.0))?,
        );
        let np_pc =
            led::BGRA::from(driver.config_option_color(
                "No progression/Pulse color",
                led::Color::new(1.0, 0.0, 0.0),
            )?);
        let pb_dc = led::BGRA::from(driver.config_option_color(
            "Progress bar/Delimiter color",
            led::Color::new(0.0, 1.0, 0.0),
        )?);
        let pb_bc = led::BGRA::from(
            driver
                .config_option_color("Progress bar/Base color", led::Color::new(0.0, 0.0, 0.0))?,
        );
        let pb_pc = led::BGRA::from(
            driver.config_option_color("Progress bar/Bar color", led::Color::new(0.0, 0.0, 1.0))?,
        );

        return Ok(Logitech {
            driver,
            np_basecolor: Color::from_rgba(np_bc[2], np_bc[1], np_bc[0], np_bc[3] as f64 / 255.0),
            np_pulsecolor: Color::from_rgba(np_pc[2], np_pc[1], np_pc[0], np_pc[3] as f64 / 255.0),
            pb_delimitercolor: Color::from_rgba(
                pb_dc[2],
                pb_dc[1],
                pb_dc[0],
                pb_dc[3] as f64 / 255.0,
            ),
            pb_basecolor: Color::from_rgba(pb_bc[2], pb_bc[1], pb_bc[0], pb_bc[3] as f64 / 255.0),
            pb_barcolor: Color::from_rgba(pb_pc[2], pb_pc[1], pb_pc[0], pb_pc[3] as f64 / 255.0),
            refreshtimer,
        });
    }
    pub fn render(&mut self, mbuf: BufferKeysColors) -> Result<(), logitech_led::Error> {
        let mut b = Bitmap::new();
        for (k, c) in mbuf.mkeyscolors {
            match &k {
                Key::G_1
                | Key::G_2
                | Key::G_3
                | Key::G_4
                | Key::G_5
                | Key::G_6
                | Key::G_7
                | Key::G_8
                | Key::G_9
                | Key::G_LOGO
                | Key::G_BADGE => {
                    let t = c.to_rgba_float();
                    self.driver.set_lighting_for_key(
                        layout::to_logitechkey(&k),
                        led::Color::from((t.r as f32, t.g as f32, t.b as f32)),
                    )?;
                }
                _ => b.set_bitmapkeycolor(k, &c),
            }
        }
        self.driver.set_lighting_from_bitmap(&b.bitmap)?;
        return Ok(());
    }
}
