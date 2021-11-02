mod base;
mod text;
mod shape;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use wasm_bindgen::prelude::*;

use base::{PositionAnchor, InternalType};
use text::TextSizeType;
use shape::ShapeType;

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct Item {
    pub internal_type: InternalType,
    pub internal_title: Option<String>,
    pub position_anchor: Option<PositionAnchor>,
    
    // Only if internal_type is ShapeModule or TextModule
    pub paint_color: Option<String>,

    // only if internal_type is ShapeModule
    pub shape_type: Option<ShapeType>,
    pub shape_corners: Option<f32>,
    pub shape_width: Option<f32>,
    pub shape_height: Option<f32>,
    pub shape_angle: Option<f32>,

    // only if internal_type is TextModule
    pub text_expression: Option<String>,
    pub text_family: Option<String>,
    pub text_size: Option<f32>,
    pub text_width: Option<f32>,
    pub text_height: Option<f32>,
    pub text_size_type: Option<TextSizeType>,

    // only if internal_type is OverlapLayerModule or StackLayerModule
    pub viewgroup_items: Option<Vec<Item>>,

    // only if the item is a root item
    pub position_offset_x: Option<f32>,
    pub position_offset_y: Option<f32>,

    // only if item is not a root item
    pub position_padding_top: Option<f32>,
    pub position_padding_right: Option<f32>,
    pub position_padding_bottom: Option<f32>,
    pub position_padding_left: Option<f32>,
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
interface BaseItem {
    internal_type: InternalType,
    internal_title?: string,

    position_anchor?: PositionAnchor,

    position_offset_x?: number,
    position_offset_y?: number,

    position_padding_top?: number;
    position_padding_right?: number;
    position_padding_bottom?: number;
    position_padding_left?: number;
}

export interface Overlap extends BaseItem {
    internal_type: 'OverlapLayerModule',
    viewgroup_items: Item[],
}

export interface Stack extends BaseItem {
    internal_type: 'StackLayerModule',
    viewgroup_items: Item[],
}

export interface Shape extends BaseItem {
    internal_type: 'ShapeModule',
    paint_color?: string;
    shape_type?: ShapeType,
    shape_corners?: number,
    shape_width?: number,
    shape_height?: number,
    shape_angle?: number,
}

export interface Text extends BaseItem {
    internal_type: 'TextModule',
    paint_color?: string;
    text_size_type?: TextSizeType,
    text_expression?: string,
    text_family?: string,
    text_size?: number,
    text_width?: number,
    text_height?: number,
}


export type Item = Overlap | Stack | Shape | Text "#;