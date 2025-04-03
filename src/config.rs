use std::fs;

pub struct Config {
    pub aimbot_enabled: bool,
    pub speed_hack_enabled: bool,
}

impl Config {
    pub fn load() -> Self {
        let data = fs::read_to_string("config.json").unwrap();
        let config: Config = serde_json::from_str(&data).unwrap();
        config
    }
}