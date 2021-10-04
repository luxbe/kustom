mod data;
mod info;
mod item;
mod root;

use super::raw;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"export interface Preset {
    info: Info,
    root: Root
}"#;

#[derive(Serialize, Deserialize)]
pub struct Preset {
    pub info: info::Info,
    pub root: root::Root,
}

pub fn from_raw_klwp(preset_raw: raw::klwp::Preset) -> Preset {
    let info = info::from_raw_klwp(preset_raw.preset_info);
    let root = root::from_raw_klwp(preset_raw.preset_root);

    Preset { info, root }
}
