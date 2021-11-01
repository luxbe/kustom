use serde::{Deserialize, Serialize};

use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Info {
    pub version: u8,
    pub title: String,
    pub description: String,
    pub author: String,
    pub email: String,
    pub width: u32,
    pub height: u32,
    pub features: String,
    pub release: u32,
    pub locked: bool,
    pub pflags: u8,
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"export interface Info {
    version: number,
    title: string,
    description: string,
    author: string,
    email: string,
    width: number,
    height: number,
    features: string,
    release: number,
    locked: boolean,
    pflags: number,
}"#;      