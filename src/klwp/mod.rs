pub mod info;
pub mod root;
pub mod item;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Preset {
    pub preset_info: info::Info,
    pub preset_root: root::Root,
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"export interface Preset {
    preset_info: Info,
    preset_root: Root,
}"#;


