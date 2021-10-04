use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum PositionAnchor {
    TOP,
    TOPLEFT,
    TOPRIGHT,
    CENTER,
    CENTERLEFT,
    CENTERRIGHT,
    BOTTOM,
    BOTTOMLEFT,
    BOTTOMRIGHT,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub internal_type: InternalType,
    pub internal_title: Option<String>,
    pub paint_color: Option<String>,
    pub position_anchor: Option<PositionAnchor>,

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
    pub position_offset_y: Option<f32>,
    pub position_offset_x: Option<f32>,

    // only if item is not a root item
    pub position_padding_left: Option<f32>,
    pub position_padding_right: Option<f32>,
    pub position_padding_top: Option<f32>,
    pub position_padding_bottom: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub enum InternalType {
    OverlapLayerModule,
    ShapeModule,
    TextModule,
    StackLayerModule,
}

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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub enum TextSizeType {
    FixedWidth,
    FitWidth,
    FitToBox,
}

#[cfg(test)]
pub mod tests {
    use super::{InternalType, Item};

    pub fn base_item(internal_type: InternalType) -> Item {
        Item {
            viewgroup_items: match internal_type {
                InternalType::OverlapLayerModule | InternalType::StackLayerModule => Some(vec![]),
                _ => None,
            },
            internal_title: None,
            internal_type,
            paint_color: None,
            shape_type: None,
            shape_corners: None,
            shape_width: None,
            shape_height: None,
            shape_angle: None,
            text_expression: None,
            text_family: None,
            text_size: None,
            text_width: None,
            text_height: None,
            text_size_type: None,
            position_offset_y: None,
            position_offset_x: None,
            position_padding_left: None,
            position_padding_right: None,
            position_padding_top: None,
            position_padding_bottom: None,
            position_anchor: None,
        }
    }
}
