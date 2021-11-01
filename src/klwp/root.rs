use super::item;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use wasm_bindgen::prelude::*;

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct Root {
    pub background_color: Option<String>,
    pub viewgroup_items: Vec<item::Item>,

    pub position_padding_top: Option<f32>,
    pub position_padding_right: Option<f32>,
    pub position_padding_bottom: Option<f32>,
    pub position_padding_left: Option<f32>,
}


#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"export interface Root {
    background_color?: string,
    viewgroup_items: Item[],

    position_padding_top?: number;
    position_padding_right?: number;
    position_padding_bottom?: number;
    position_padding_left?: number;
}"#;