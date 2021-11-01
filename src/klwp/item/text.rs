use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TextSizeType {
    FixedWidth,
    FitWidth,
    FitToBox,
}
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"export type TextSizeType =
    | 'FIXED_WIDTH'
    | 'FIT_WIDTH'
    | 'FIT_TO_BOX'"#;