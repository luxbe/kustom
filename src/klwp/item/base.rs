use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub enum InternalType {
    OverlapLayerModule,
    ShapeModule,
    TextModule,
    StackLayerModule,
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"export type InternalType =
    | 'OverlapLayerModule'
    | 'StackLayerModule'
    | 'ShapeModule'
    | 'TextModule'"#;
    

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PositionAnchor {
    Top,
    TopLeft,
    TopRight,
    Center,
    CenterLeft,
    CENTERRight,
    Bottom,
    BottomLeft,
    BottomRight,
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"export type PositionAnchor =
    | 'TOP'
    | 'TOPLEFT'
    | 'TOPRIGHT'
    | 'CENTER'
    | 'CENTERLEFT'
    | 'CENTERRIGHT'
    | 'BOTTOM'
    | 'BOTTOMLEFT'
    | 'BOTTOMRIGHT'"#;