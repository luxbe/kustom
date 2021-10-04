use super::raw::klwp;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Info {
    pub title: String,
    pub description: String,
    pub author: String,
    pub email: String,
    pub width: u32,
    pub height: u32,

    // unused for now
    features: String,
    release: u32,
    locked: bool,
    pflags: u8,
}

pub fn from_raw_klwp(info_raw: klwp::info::Info) -> Info {
    let title = info_raw.title;
    let description = info_raw.description;
    let author = info_raw.author;
    let email = info_raw.email;
    let width = info_raw.width;
    let height = info_raw.height;
    let features = info_raw.features;
    let release = info_raw.release;
    let locked = info_raw.locked;
    let pflags = info_raw.pflags;

    Info {
        title,
        description,
        author,
        email,
        width,
        height,

        features,
        release,
        locked,
        pflags,
    }
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"export interface Info {
    title: string,
    description: string,
    author: string,
    email: string,
    width: number,
    height: number,
    
    features: string,
    release: number,
    locked: boolean,
    pflags: number
}"#;
