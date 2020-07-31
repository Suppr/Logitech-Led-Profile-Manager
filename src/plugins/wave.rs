use super::{
    color_serde, ou64_to_oduration, u64_to_duration, BufferKeysColors, Color, CreateEffect,
    Deserialize, Duration, Effect, Fraction, HashMap, Instant, Key, KeyboardLayout, KeysData,
    RenderColor, Serialize, TimedEffect, TriangleInterpolation, RGBA,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MultipleWaveSettings {
    #[serde(with = "color_serde")]
    pub basecolor: Color,
    #[serde(with = "color_serde")]
    pub targetcolor: Color,
    #[serde(deserialize_with = "ou64_to_oduration", default)]
    pub starttime: Option<Duration>,
    #[serde(deserialize_with = "u64_to_duration")]
    pub effectdur: Duration,
    #[serde(deserialize_with = "u64_to_duration")]
    pub decalage: Duration,
    #[serde(deserialize_with = "u64_to_duration")]
    pub fadeindur: Duration,
    #[serde(deserialize_with = "u64_to_duration")]
    pub fadeoutdur: Duration,
    pub infinite: Option<bool>,
    #[serde(skip)]
    pub effectname: String,
}

pub struct WaveLayout {
    keys: Vec<(Key, TriangleInterpolation)>,
    created: Instant,
    basecolor: Color,
    targetcolor: Color,
    endtime: Duration,
    infinite: bool,
}
impl WaveLayout {
    pub fn from_keysdata(
        keystofade: &KeysData,
        basecolor: &Color,
        targetcolor: &Color,
        starttime: Duration,
        effectdur: Duration,
        fadeindur: Duration,
        fadeoutdur: Duration,
        idc: i64,
        infinite: Option<bool>,
    ) -> Self {
        let mut keys = Vec::with_capacity(keystofade.keys.len() * 2);
        let mut progresstimer = starttime;

        for (key, keydata) in keystofade.keys.iter() {
            let mult = (keydata.x + (keydata.width / 2)) as f32 / idc as f32;
            progresstimer = starttime + effectdur.mul_f32(mult);
            keys.push((
                *key,
                TriangleInterpolation::new(progresstimer, fadeindur, fadeoutdur, (0.0, 1.0)),
            ));
        }

        let b;
        if let Some(o) = infinite {
            b = o;
        } else {
            b = true;
        }

        let endtime = progresstimer + fadeindur + fadeoutdur;
        Self {
            keys,
            created: Instant::now(),
            basecolor: basecolor.clone(),
            targetcolor: targetcolor.clone(),
            endtime,
            infinite: b,
        }
    }
}
impl RenderColor for WaveLayout {
    fn render_colors(&self, mbuf: &mut BufferKeysColors) {
        let elapsed = self.elapsed();
        mbuf.update(
            self.keys
                .iter()
                .map(|(k, v)| {
                    (
                        *k,
                        self.basecolor.mix::<RGBA<f64>>(
                            &self.targetcolor,
                            Fraction::from(v.interp(elapsed) as f64),
                        ),
                    )
                })
                .collect::<HashMap<_, _>>(),
        );
    }
}
impl TimedEffect for WaveLayout {
    fn elapsed(&self) -> Duration {
        self.created.elapsed()
    }

    fn get_endtime(&self) -> Duration {
        self.endtime
    }
    fn reset(&mut self) {
        self.created = Instant::now();
    }
    fn is_infinite(&self) -> bool {
        self.infinite
    }
}

pub struct MultipleWaveLayout {
    pub vwaveeffect: Vec<WaveLayout>,
    pub created: Instant,
    infinite: bool, //pub effectdur: Duration,
}
impl MultipleWaveLayout {
    pub fn from_keysdata(
        vkeystofade: Vec<KeysData>,
        basecolor: &Color,
        targetcolor: &Color,
        starttime: Duration,
        effectdur: Duration,
        decalage: Duration,
        fadeindur: Duration,
        fadeoutdur: Duration,
        infinite: Option<bool>,
    ) -> Self {
        let mut vwaveeffect = Vec::with_capacity(vkeystofade.len());

        let b;
        if let Some(o) = infinite {
            b = o;
        } else {
            b = true;
        }

        let idc = vkeystofade
            .iter()
            .map(|k| k.get_keyboardlenx())
            .max()
            .unwrap();

        for (i, keystofade) in vkeystofade.iter().rev().enumerate() {
            vwaveeffect.push(WaveLayout::from_keysdata(
                keystofade,
                basecolor,
                targetcolor,
                starttime + (decalage * i as u32),
                effectdur,
                fadeindur,
                fadeoutdur,
                idc,
                infinite,
            ));
        }

        Self {
            vwaveeffect,
            created: Instant::now(),
            infinite: b,
        }
    }
}

impl CreateEffect for MultipleWaveSettings {
    fn create_effect(&self, k: &KeyboardLayout) -> Effect {
        Effect::MultipleWaveLayout(MultipleWaveLayout::from_keysdata(
            vec![
                KeysData::from_keysandlayout(Key::gline(), &k),
                KeysData::from_keysandlayout(Key::firstline(), &k),
                KeysData::from_keysandlayout(Key::secondline(), &k),
                KeysData::from_keysandlayout(Key::thirdline(), &k),
                KeysData::from_keysandlayout(Key::fourthline(), &k),
                KeysData::from_keysandlayout(Key::fifthline(), &k),
                KeysData::from_keysandlayout(Key::sixthline(), &k),
                KeysData::from_keysandlayout(Key::gbadge(), &k),
            ],
            &self.basecolor,
            &self.targetcolor,
            self.starttime.unwrap_or(Duration::from_secs(0)),
            self.effectdur,
            self.decalage,
            self.fadeindur,
            self.fadeoutdur,
            self.infinite,
        ))
    }
}

impl RenderColor for MultipleWaveLayout {
    fn render_colors(&self, mbuf: &mut BufferKeysColors) {
        for waveeffect in &self.vwaveeffect {
            waveeffect.render_colors(mbuf);
        }
    }
}
impl TimedEffect for MultipleWaveLayout {
    fn elapsed(&self) -> Duration {
        self.created.elapsed()
    }

    fn get_endtime(&self) -> Duration {
        self.vwaveeffect.iter().map(|a| a.endtime).max().unwrap()
    }
    fn reset(&mut self) {
        self.created = Instant::now();
        for waveeffect in &mut self.vwaveeffect {
            waveeffect.created = self.created;
        }
    }
    fn is_infinite(&self) -> bool {
        self.infinite
    }
}
