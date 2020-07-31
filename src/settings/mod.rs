use super::{plugins::CreateEffect, plugins::PluginSettings, Effect, Key, KeyboardLayout};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OneEffect {
    pub groups: Option<HashMap<String, Vec<Key>>>,
    pub plugins: Vec<PluginSettings>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lookup {
    pub class: Option<String>,
    pub title: Option<String>,
    #[serde(skip)]
    pub reclass: Option<Regex>,
    #[serde(skip)]
    pub retitle: Option<Regex>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub lookup: Option<Lookup>,
    pub effects: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub groups: Option<HashMap<String, Vec<Key>>>,
    pub effects: HashMap<String, OneEffect>,
    pub profiles: HashMap<String, Profile>,
}
impl Settings {
    pub fn from_file() -> Self {
        let s = std::fs::read_to_string("config.yml").expect("Fail loading config file");
        serde_yaml::from_str(&s).expect("failed to parse config file")
    }
    pub fn get_final(&mut self) {
        for (effectname, effect) in self.effects.iter_mut() {
            for plugin in effect.plugins.iter_mut() {
                match plugin {
                    PluginSettings::K2000(ks) => {
                        if let Some(k) = ks.keys.clone() {
                            ks.finalkeys = k;
                        } else if let Some(gname) = ks.group.clone() {
                            if let Some(geffect) = effect.groups.clone() {
                                if !geffect[&gname].is_empty() {
                                    ks.finalkeys = geffect[&gname].clone()
                                }
                            } else if let Some(gsettings) = self.groups.clone() {
                                if !gsettings[&gname].is_empty() {
                                    ks.finalkeys = gsettings[&gname].clone()
                                }
                            } else {
                                println!("Can't find group name")
                            }
                        }
                        if ks.finalkeys.is_empty() {
                            ks.finalkeys = Key::allkeys()
                        }
                        ks.effectname = effectname.to_string();
                    }
                    PluginSettings::MultipleWave(mws) => {
                        mws.effectname = effectname.to_string();
                    }
                    PluginSettings::ProgressBar(pbs) => {
                        if let Some(k) = pbs.keys.clone() {
                            pbs.finalkeys = k;
                        } else if let Some(gname) = pbs.group.clone() {
                            if let Some(geffect) = effect.groups.clone() {
                                if !geffect[&gname].is_empty() {
                                    pbs.finalkeys = geffect[&gname].clone();
                                }
                            } else if let Some(gsettings) = self.groups.clone() {
                                if !gsettings[&gname].is_empty() {
                                    pbs.finalkeys = gsettings[&gname].clone();
                                }
                            } else {
                                println!("Can't find group name")
                            }
                        }
                        if pbs.finalkeys.is_empty() {
                            pbs.finalkeys = Key::allkeys();
                        }
                        pbs.effectname = effectname.to_string();
                    }
                    PluginSettings::Breathe(bes) => {
                        if let Some(k) = bes.keys.clone() {
                            bes.finalkeys = k;
                        } else if let Some(gname) = bes.group.clone() {
                            if let Some(geffect) = effect.groups.clone() {
                                if !geffect[&gname].is_empty() {
                                    bes.finalkeys = geffect[&gname].clone();
                                }
                            } else if let Some(gsettings) = self.groups.clone() {
                                if !gsettings[&gname].is_empty() {
                                    bes.finalkeys = gsettings[&gname].clone();
                                }
                            } else {
                                println!("Can't find group name")
                            }
                        }
                        if bes.finalkeys.is_empty() {
                            bes.finalkeys = Key::allkeys();
                        }
                        bes.effectname = effectname.to_string();
                    }
                    PluginSettings::Fill(fs) => {
                        if let Some(k) = fs.keys.clone() {
                            fs.finalkeys = k;
                        } else if let Some(gname) = fs.group.clone() {
                            if let Some(geffect) = effect.groups.clone() {
                                if !geffect[&gname].is_empty() {
                                    fs.finalkeys = geffect[&gname].clone();
                                }
                            } else if let Some(gsettings) = self.groups.clone() {
                                if !gsettings[&gname].is_empty() {
                                    fs.finalkeys = gsettings[&gname].clone();
                                }
                            } else {
                                println!("Can't find group name")
                            }
                        }
                        if fs.finalkeys.is_empty() {
                            fs.finalkeys = Key::allkeys();
                        }
                        fs.effectname = effectname.to_string();
                    }
                }
            }
        }
        for (_, p) in self.profiles.iter_mut() {
            if let Some(l) = &mut p.lookup {
                if let Some(c) = &l.class {
                    l.reclass = Some(Regex::new(c).unwrap());
                } else {
                    l.reclass = None;
                }
                if let Some(t) = &l.title {
                    l.retitle = Some(Regex::new(t).unwrap());
                } else {
                    l.retitle = None;
                }
            }
        }
    }
    pub fn new() -> Self {
        let mut a = Settings::from_file();
        a.get_final();
        return a;
    }

    pub fn get_effect(&self, keyboardlayout: KeyboardLayout) -> HashMap<String, Vec<Effect>> {
        let mut h = HashMap::new();
        for (effectname, effect) in self.effects.iter() {
            let mut v = Vec::new();
            for plugin in effect.plugins.iter() {
                match plugin {
                    PluginSettings::K2000(ks) => v.push(ks.create_effect(&keyboardlayout)),
                    PluginSettings::MultipleWave(mws) => v.push(mws.create_effect(&keyboardlayout)),
                    PluginSettings::ProgressBar(pbs) => v.push(pbs.create_effect(&keyboardlayout)),
                    PluginSettings::Breathe(bes) => v.push(bes.create_effect(&keyboardlayout)),
                    PluginSettings::Fill(fs) => v.push(fs.create_effect(&keyboardlayout)),
                }
            }
            h.insert(effectname.clone(), v);
        }
        return h;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn load_settings() {
        let s = include_str!("test.yml");
        let mut settings: Settings = serde_yaml::from_str(s).expect("failed to parse yml");
        settings.get_final();
        println!("{:#?}", settings);
    }
}
