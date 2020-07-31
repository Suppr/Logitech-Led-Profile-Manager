use super::{
    color_serde, ocolor_serde, u64_to_duration, BackgroundColor, BufferKeysColors, Color,
    CreateEffect, Deserialize, Duration, Effect, HashMap, Instant, Key, KeyboardLayout,
    RenderColor, Serialize, TimedEffect, TriangleInterpolation, RGBA,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BreatheSettings {
    pub keys: Option<Vec<Key>>,
    pub group: Option<String>,
    #[serde(with = "color_serde")]
    pub color: Color,
    #[serde(with = "ocolor_serde", default)]
    pub bgcolor: Option<Color>,
    #[serde(deserialize_with = "u64_to_duration")]
    pub duration: Duration,
    pub infinite: Option<bool>,
    #[serde(skip)]
    pub finalkeys: Vec<Key>,
    #[serde(skip)]
    pub effectname: String,
}

pub struct Breathe {
    keys: Vec<(Key, TriangleInterpolation)>,
    color: Color,
    bgcolor: BackgroundColor,
    created: Instant,
    duration: Duration,
    infinite: bool,
}
impl Breathe {
    pub fn for_key(
        keys: Vec<Key>,
        color: Color,
        bgcolor: BackgroundColor,
        duration: Duration,
        infinite: Option<bool>,
    ) -> Self {
        let mut v = Vec::with_capacity(keys.len());
        let b;
        if let Some(o) = infinite {
            b = o;
        } else {
            b = true;
        }
        for key in keys {
            v.push((
                key,
                TriangleInterpolation::new(
                    Duration::from_millis(0),
                    duration / 2,
                    duration / 2,
                    (0.0, 1.0),
                ),
            ))
        }
        Self {
            keys: v,
            color,
            bgcolor,
            created: Instant::now(),
            duration,
            infinite: b,
        }
    }
}

impl CreateEffect for BreatheSettings {
    fn create_effect(&self, _: &KeyboardLayout) -> Effect {
        let bgcolor;
        if let Some(c) = &self.bgcolor {
            bgcolor = BackgroundColor::StaticColor(c.clone());
        } else {
            bgcolor = BackgroundColor::Bgopacity(0.0)
        }
        Effect::Breathe(Breathe::for_key(
            self.finalkeys.clone(),
            self.color.clone(),
            bgcolor,
            self.duration,
            self.infinite,
        ))
    }
}

impl RenderColor for Breathe {
    fn render_colors(&self, mbuf: &mut BufferKeysColors) {
        mbuf.update(self.keys
            .iter()
            .map(|(k, v)| match &self.bgcolor {
                BackgroundColor::Bgopacity(_) => {(
                    *k,
                    mbuf.get_keycolor(&k).mix::<RGBA<f64>>(&self.color,pastel::Fraction::from(v.interp(self.elapsed())as f64 ))
                    //blend_colors(
                    //    bitmap.get_bitmapkeycolor(*k),
                    //    self.color,
                    //    v.linear_interp(self.elapsed()),
                    //),
                )},
                BackgroundColor::StaticColor(c) => (
                    *k,
                    c.mix::<RGBA<f64>>(&self.color,pastel::Fraction::from(v.interp(self.elapsed())as f64 ))
                ),
            })
            .collect::<HashMap<_,_>>());
    }
}

impl TimedEffect for Breathe {
    fn elapsed(&self) -> Duration {
        self.created.elapsed()
    }
    fn get_endtime(&self) -> Duration {
        self.duration
    }
    fn reset(&mut self) {
        self.created = Instant::now();
    }
    fn is_infinite(&self) -> bool {
        self.infinite
    }
}
