use super::{
    color_serde, BufferKeysColors, Color, CreateEffect, Deserialize, Effect, HashMap, Key,
    KeyboardLayout, RenderColor, Serialize,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FillSetings {
    pub keys: Option<Vec<Key>>,
    pub group: Option<String>,
    #[serde(with = "color_serde")]
    pub color: Color,
    #[serde(skip)]
    pub finalkeys: Vec<Key>,
    #[serde(skip)]
    pub effectname: String,
}

pub struct Fill {
    keys: Vec<Key>,
    color: Color,
}
impl Fill {
    pub fn for_keys(keys: Vec<Key>, color: Color) -> Self {
        Self { keys, color }
    }
}

impl CreateEffect for FillSetings {
    fn create_effect(&self, _: &KeyboardLayout) -> Effect {
        Effect::Fill(Fill::for_keys(self.finalkeys.clone(), self.color.clone()))
    }
}

impl RenderColor for Fill {
    fn render_colors(&self, mbuf: &mut BufferKeysColors) {
        mbuf.update(
            self.keys
                .iter()
                .map(|k| (*k, self.color.clone()))
                .collect::<HashMap<Key, Color>>(),
        );
    }
}
