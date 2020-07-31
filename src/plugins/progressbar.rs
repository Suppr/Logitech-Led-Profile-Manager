use super::{
    color_serde, ocolor_serde, BackgroundColor, BufferKeysColors, Color, CreateEffect, Deserialize,
    Effect, Fraction, HashMap, Key, KeyboardLayout, RenderColor, Serialize, RGBA,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProgressOn {
    Winamp,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProgressBarSettings {
    pub keys: Option<Vec<Key>>,
    pub group: Option<String>,
    #[serde(with = "ocolor_serde", default)]
    pub delimiter: Option<Color>,
    #[serde(with = "color_serde")]
    pub barcolor: Color,
    #[serde(with = "ocolor_serde", default)]
    pub bgcolor: Option<Color>,
    pub bgopacity: Option<f32>,
    pub progresson: ProgressOn,
    #[serde(skip)]
    pub finalkeys: Vec<Key>,
    #[serde(skip)]
    pub effectname: String,
}

pub struct ProgressBar {
    keys: Vec<Key>,
    delimiter: Option<Color>,
    pbcolor: Color,
    background: BackgroundColor,
    pub percent: f32,
    pub progresson: ProgressOn,
}

impl ProgressBar {
    pub fn new(
        pbkeys: Vec<Key>,
        delimiter: Option<Color>,
        pbcolor: Color,
        background: BackgroundColor,
        progresson: ProgressOn,
    ) -> Self {
        Self {
            keys: pbkeys,
            delimiter,
            pbcolor,
            background,
            percent: 0.0,
            progresson,
        }
    }
}

impl CreateEffect for ProgressBarSettings {
    fn create_effect(&self, _: &KeyboardLayout) -> Effect {
        let bgcolor;
        if let Some(c) = &self.bgcolor {
            bgcolor = BackgroundColor::StaticColor(c.clone());
        } else if let Some(o) = self.bgopacity {
            bgcolor = BackgroundColor::Bgopacity(o.clone())
        } else {
            bgcolor = BackgroundColor::Bgopacity(0.0)
        }
        Effect::ProgressBar(ProgressBar::new(
            self.finalkeys.clone(),
            self.delimiter.clone(),
            self.barcolor.clone(),
            bgcolor,
            self.progresson.clone(),
        ))
    }
}

impl RenderColor for ProgressBar {
    fn render_colors(&self, mbuf: &mut BufferKeysColors) {
        let mut vkc = Vec::with_capacity(self.keys.len());

        let pbkeys;
        if let Some(delcolor) = &self.delimiter {
            vkc.push((*self.keys.first().unwrap(), delcolor.clone()));
            vkc.push((*self.keys.last().unwrap(), delcolor.clone()));
            pbkeys = self.keys[1..self.keys.len() - 1].to_vec();
        } else {
            pbkeys = self.keys.clone();
        }

        let value = self.percent * (pbkeys.len()) as f32;
        let alpha = value.fract() as f32;

        let mut i = 0;
        while i < (value.floor() as usize) {
            if i < pbkeys.len() {
                vkc.push((pbkeys[i], self.pbcolor.clone()));
            }
            i += 1;
        }
        if i >= pbkeys.len() {
            mbuf.update(
                vkc.iter()
                    .map(|(k, c)| (*k, c.clone()))
                    .collect::<HashMap<_, _>>(),
            );
            return;
        }
        let lastc =
            Color::black().mix::<RGBA<f64>>(&self.pbcolor, Fraction::from(alpha.powf(2.0) as f64));
        vkc.push((pbkeys[i], lastc));

        i += 1;

        while i < pbkeys.len() {
            match &self.background {
                BackgroundColor::StaticColor(bgcolor) => vkc.push((pbkeys[i], bgcolor.clone())),
                BackgroundColor::Bgopacity(opacity) => {
                    let bgcolor = mbuf.get_keycolor(&pbkeys[i]).darken(*opacity as f64);
                    vkc.push((pbkeys[i], bgcolor));
                }
            }
            i += 1;
        }
        mbuf.update(
            vkc.iter()
                .map(|(k, c)| (*k, c.clone()))
                .collect::<HashMap<_, _>>(),
        );
    }
}
