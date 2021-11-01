use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum ShapeType {
    Rect,
    Circle,
    Oval,
    Triangle,
    RTriangle,
    Exagon,
    Slice,
    Arc,
    Squircle,
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"export type ShapeType =
    | 'RECT'
    | 'CIRCLE'
    | 'OVAL'
    | 'TRIANGLE'
    | 'RTRIANGLE'
    | 'EXAGON'
    | 'SLICE'
    | 'ARC'
    | 'SQUIRCLE'"#;