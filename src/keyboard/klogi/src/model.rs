// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};
use crate::keys::KBlock;
#[derive(Debug, Serialize, Deserialize)]
pub struct Keyboard {
    pub layout: String,
    pub spurious: Vec<Spurious>,
    #[serde(rename="keyboards")]
    pub zones: Vec<Zone>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Zone {
    #[serde(with = "kblock_serde", default)]
    pub zone: Option<KBlock>,
    pub keys: Vec<Keydata>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Keydata {
    pub code: i64,
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
    pub glyph: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Spurious {
    pub zone: i64,
    pub code: i64,
}


mod kblock_serde {
    use super::KBlock;
    use serde::{self, Deserialize, Serializer, Deserializer};

    pub fn serialize<S>(date: &Option<KBlock>, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        if let Some(ref d) = *date {
            return s.serialize_i64(*d as i64)
        }
        s.serialize_none()
    }

    pub fn deserialize<'de, D>(deserializer: D)
        -> Result<Option<KBlock>, D::Error>
        where D: Deserializer<'de> {
        let s: Option<i64> = Option::deserialize(deserializer)?;
        if let Some(s) = s {
            return Ok(std::convert::TryFrom::try_from(s).ok())
        }

        Ok(None)
    }
}
