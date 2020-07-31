use super::keyboard::BufferKeysColors;
use super::{Color, Fraction, Key, KeyboardLayout, KeydataType, RGBA};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::hash_map::HashMap;
use std::time::{Duration, Instant};

pub mod breathe;
pub mod fill;
pub mod k2000;
pub mod progressbar;
pub mod wave;

mod maths;
use maths::TriangleInterpolation;

pub struct KeysData {
    pub keys: Vec<KeydataType>,
}

impl KeysData {
    pub fn from_keysandlayout(keys: Vec<Key>, keyboardlayout: &KeyboardLayout) -> Self {
        Self {
            keys: keyboardlayout.get_keysdata(keys),
        }
    }

    fn get_firstx(&self) -> i64 {
        self.keys.iter().map(|(_, f)| f.x).min().unwrap()
    }

    fn get_keyboardlenx(&self) -> i64 {
        let lastx = self
            .keys
            .iter()
            .map(|(_, f)| f.x + (f.width))
            .max()
            .unwrap();

        return lastx - self.get_firstx();
    }
}

pub enum BackgroundColor {
    StaticColor(Color),
    Bgopacity(f32),
}

//>>>>>>>

pub trait RenderColor {
    fn render_colors(&self, mbufkeycolor: &mut BufferKeysColors);
}

pub trait TimedEffect {
    fn elapsed(&self) -> Duration;
    fn get_endtime(&self) -> Duration;
    fn reset(&mut self);
    fn is_infinite(&self) -> bool;
}

pub enum Effect {
    K2000Layout(k2000::K2000Layout),
    MultipleWaveLayout(wave::MultipleWaveLayout),
    ProgressBar(progressbar::ProgressBar),
    Breathe(breathe::Breathe),
    Fill(fill::Fill),
}
//<<<<<<<<<<<< TODO : Dyon ending code

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "plugin")]
pub enum PluginSettings {
    K2000(k2000::K2000Settings),
    MultipleWave(wave::MultipleWaveSettings),
    ProgressBar(progressbar::ProgressBarSettings),
    Breathe(breathe::BreatheSettings),
    Fill(fill::FillSetings),
}

//impl PluginSettings {
//    fn create_effect (self, k: KeyboardLayout) -> Effect{
//
//    }
//}

pub trait CreateEffect {
    fn create_effect(&self, k: &KeyboardLayout) -> Effect;
}

//serde
pub fn u64_to_duration<'de, D: Deserializer<'de>>(d: D) -> Result<Duration, D::Error> {
    let s = u64::deserialize(d)?;
    Ok(Duration::from_millis(s))
}
pub fn ou64_to_oduration<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Duration>, D::Error> {
    let o: Option<u64> = Option::deserialize(d)?;
    Ok(o.map(|s| Duration::from_millis(s)))
}
mod ocolor_serde {
    use super::Color;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(color: &Option<Color>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(ref c) = *color {
            return s.serialize_str(&c.to_rgb_string(pastel::Format::NoSpaces));
        }
        s.serialize_none()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Color>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        if let Some(s) = s {
            return Ok(pastel::parser::parse_color(&s));
        }
        Ok(None)
    }
}
mod color_serde {
    use super::Color;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(color: &Color, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        return s.serialize_str(&color.to_rgb_string(pastel::Format::NoSpaces));
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        return Ok(pastel::parser::parse_color(&s).expect("Cannot parse Color"));
    }
}
