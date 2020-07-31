use super::{
    color_serde, ou64_to_oduration, u64_to_duration, BufferKeysColors, Color, CreateEffect,
    Deserialize, Duration, Effect, Fraction, HashMap, Instant, Key, KeyboardLayout, KeysData,
    RenderColor, Serialize, TimedEffect, TriangleInterpolation, RGBA,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct K2000Settings {
    pub keys: Option<Vec<Key>>,
    pub group: Option<String>,
    #[serde(with = "color_serde")]
    pub basecolor: Color,
    #[serde(with = "color_serde")]
    pub pulsecolor: Color,
    #[serde(deserialize_with = "ou64_to_oduration", default)]
    pub starttime: Option<Duration>,
    #[serde(deserialize_with = "u64_to_duration")]
    pub effectdur: Duration,
    #[serde(deserialize_with = "u64_to_duration")]
    pub fadeindur: Duration,
    #[serde(deserialize_with = "u64_to_duration")]
    pub fadeoutdur: Duration,
    pub infinite: Option<bool>,
    #[serde(skip)]
    pub finalkeys: Vec<Key>,
    #[serde(skip)]
    pub effectname: String,
}

pub struct K2000Layout {
    keys: Vec<(Key, TriangleInterpolation)>,
    created: Instant,
    basecolor: Color,
    targetcolor: Color,
    endtime: Duration,
    reverse: Duration,
    infinite: bool,
}
impl K2000Layout {
    pub fn from_keysdata(
        keystofade: KeysData,
        basecolor: &Color,
        targetcolor: &Color,
        starttime: Duration,
        effectdur: Duration,
        fadeindur: Duration,
        fadeoutdur: Duration,
        infinite: Option<bool>,
    ) -> Self {
        let mut keys = Vec::with_capacity(keystofade.keys.len() * 2);
        let mut progresstimer = starttime;

        let idc = keystofade.get_keyboardlenx();

        let effectdurdiv2 = effectdur.div_f32(2.0);
        for (k, d) in keystofade.keys.iter() {
            let mult = ((d.x + (d.width / 2)) - keystofade.get_firstx()) as f32 / idc as f32;

            progresstimer = starttime + effectdurdiv2.mul_f32(mult);
            keys.push((
                *k,
                TriangleInterpolation::new(progresstimer, fadeindur, fadeoutdur, (0.0, 1.0)),
            ));
        }

        let reverse = starttime + (effectdur.div_f32(2.0) + fadeindur);

        for (k, d) in keystofade.keys.iter().rev() {
            let mult = ((d.x + (d.width / 2)) - keystofade.get_firstx()) as f32 / idc as f32;

            progresstimer = (starttime + effectdurdiv2) + effectdurdiv2.mul_f32(1.0 - mult);

            keys.push((
                *k,
                TriangleInterpolation::new(progresstimer, fadeindur, fadeoutdur, (0.0, 1.0)),
            ));
        }
        debug_assert!((keys.len() & 1) == 0);
        let endtime = progresstimer + fadeindur + fadeoutdur;

        let b;
        if let Some(o) = infinite {
            b = o;
        } else {
            b = true;
        }

        Self {
            keys,
            created: Instant::now(),
            basecolor: basecolor.clone(),
            targetcolor: targetcolor.clone(),
            //starttime,
            //fadeindur,
            //fadeoutdur,
            endtime,
            reverse,
            infinite: b,
        }
    }
}

impl CreateEffect for K2000Settings {
    fn create_effect(&self, k: &KeyboardLayout) -> Effect {
        Effect::K2000Layout(K2000Layout::from_keysdata(
            KeysData {
                keys: k.get_keysdata(self.finalkeys.clone()),
            },
            &self.basecolor,
            &self.pulsecolor,
            self.starttime.unwrap_or(Duration::from_secs(1)),
            self.effectdur,
            self.fadeindur,
            self.fadeoutdur,
            self.infinite,
        ))
    }
}

impl RenderColor for K2000Layout {
    fn render_colors(&self, mbuf: &mut BufferKeysColors) {
        let elapsed = self.elapsed();

        let a = if self.reverse > elapsed {
            &self.keys[..&self.keys.len() / 2]
        } else {
            &self.keys[&self.keys.len() / 2..]
        };
        mbuf.update(
            a.iter()
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

impl TimedEffect for K2000Layout {
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
