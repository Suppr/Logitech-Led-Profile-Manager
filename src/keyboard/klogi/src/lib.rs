

pub mod keys;
pub mod model;

pub fn keyboard_to_keys(m: model::Keyboard) -> Vec<(Option<keys::KBlock>, model::Keydata)> {
    m.zones
        .into_iter()
        .map(|z| {
            let b = z.zone.clone();
            z.keys.into_iter().map(move |z1| (b, z1.clone()))
        })
        .flatten()
        .collect::<Vec<_>>()
}

pub fn parse_layout() -> model::Keyboard {
    let yml_str = include_str!("c32b00000000_0005.yaml");
    serde_yaml::from_str(yml_str).expect("failed to parse yml")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let yml_str = include_str!("c32b00000000_0005.yaml");
        let keyboard_fr: model::Keyboard =
            serde_yaml::from_str(yml_str).expect("failed to parse yml");
        println!("{:#?}", keyboard_fr);
    }
}
