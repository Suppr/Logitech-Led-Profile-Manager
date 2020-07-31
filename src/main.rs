//#![windows_subsystem = "windows"]
extern crate logitech_led as led;

use std::collections::hash_map::HashMap;
use std::thread::sleep;
use std::time::Duration;

use pastel::{Color, Fraction, RGBA};
use winamp;
mod keyboard;
mod lookout;
pub mod plugins;
mod settings;
use keyboard::layout::{Key, KeyboardLayout, KeydataType};

use plugins::Effect;

fn get_winamptime() -> Option<f32> {
    let winamp1 = winamp::Winamp::new()?;

    if winamp1.get_status() != 1 {
        //get_status return : 0 paused, 1 playing, 2 stop
        return None;
    }
    let currentpos = winamp1.get_elapsedtime();
    let tracklength = winamp1.get_tracklength();
    return Some(currentpos as f32 / tracklength as f32);
}

fn run_effect(
    effects: &mut HashMap<String, Vec<Effect>>,
    effectslist: &Vec<String>,
    mbuf: &mut keyboard::BufferKeysColors,
) {
    for n in effectslist {
        for e in effects.get_mut(n).unwrap() {
            match e {
                Effect::MultipleWaveLayout(msl) => {
                    mbuf.run_timedeffect(msl);
                }
                Effect::K2000Layout(kl) => {
                    mbuf.run_timedeffect(kl);
                }
                Effect::ProgressBar(pb) => match pb.progresson {
                    plugins::progressbar::ProgressOn::Winamp => {
                        if let Some(percent) = get_winamptime() {
                            pb.percent = percent;
                            mbuf.run_staticeffect(pb);
                        }
                    }
                },
                Effect::Breathe(be) => {
                    mbuf.run_timedeffect(be);
                }
                Effect::Fill(fe) => mbuf.run_staticeffect(fe),
            }
        }
    }
}

fn main() {
    let mut logitech = keyboard::Logitech::new().expect("SDK init fail");
    let keyboard = KeyboardLayout::new();

    let settings = settings::Settings::new();

    let mut effects = settings.get_effect(keyboard);

    loop {
        let mut mbuf = keyboard::BufferKeysColors::new();

        let default = &settings.profiles["__default__"].effects;
        run_effect(&mut effects, default, &mut mbuf);

        let profilelist = settings
            .profiles
            .iter()
            .clone()
            .filter(|(a, _)| a != &"__default__" && a != &"__overlay__")
            .collect::<HashMap<_, _>>();
        for (_, profile) in profilelist.iter() {
            if lookout::get_active(profile.lookup.clone()) {
                run_effect(&mut effects, &profile.effects, &mut mbuf)
            }
        }

        if let Some(overlay) = &settings.profiles.get("__overlay__") {
            run_effect(&mut effects, &overlay.effects, &mut mbuf);
        }

        logitech
            .render(mbuf)
            .expect("Unable to update Color -- SDK ERROR");

        sleep(Duration::from_millis(50));
    }
}
