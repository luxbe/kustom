pub mod info;
pub mod root;
pub mod item;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Preset {
    pub preset_info: info::Info,
    pub preset_root: root::Root,
}

pub fn from_json(s: &str) -> Preset {
    serde_json::from_str(s).unwrap()
}
