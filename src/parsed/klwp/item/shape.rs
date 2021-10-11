use super::{data::*, klwp, Item};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use wasm_bindgen::prelude::*;

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct Shape {
    pub data: ShapeData,
    pub paint: paint::Paint,
    pub id: String,
    pub title: Option<String>,
    #[serde(rename(serialize = "isRoot"))]
    pub is_root: Option<bool>,
    pub anchor: anchor::Anchor,
    pub offset: offset::Offset,
    pub padding: padding::Padding,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct ShapeData {
    r#type: ShapeDataType,
    width: f32,
    height: Option<f32>,
    angle: Option<f32>,
    corners: Option<f32>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "SCREAMING_SNAKE_CASE")]
pub enum ShapeDataType {
    Rectangle,
    Square,
    Circle,
    Oval,
    Triangle,
    RightTriangle,
    Hexagon,
    Slice,
    Arc,
    Squircle,
}

pub fn from_raw_klwp(shape_raw: &klwp::item::Item, id: &str, is_root: bool) -> Item {
    let data = ShapeData {
        r#type: match &shape_raw.shape_type {
            Some(v) => match v {
                klwp::item::ShapeType::Arc => ShapeDataType::Arc,
                klwp::item::ShapeType::Circle => ShapeDataType::Circle,
                klwp::item::ShapeType::Exagon => ShapeDataType::Hexagon,
                klwp::item::ShapeType::Oval => ShapeDataType::Oval,
                klwp::item::ShapeType::Rect => ShapeDataType::Rectangle,
                klwp::item::ShapeType::RTriangle => ShapeDataType::RightTriangle,
                klwp::item::ShapeType::Slice => ShapeDataType::Slice,
                klwp::item::ShapeType::Squircle => ShapeDataType::Squircle,
                klwp::item::ShapeType::Triangle => ShapeDataType::Triangle,
            },
            _ => ShapeDataType::Square,
        },
        width: shape_raw.shape_width.unwrap_or(20.0),
        height: match &shape_raw.shape_type {
            Some(v) => match v {
                klwp::item::ShapeType::Circle => None,
                _ => shape_raw.shape_height.or(Some(20.0)),
            },
            _ => None,
        },
        angle: match &shape_raw.shape_type {
            Some(v) => match v {
                klwp::item::ShapeType::Slice | klwp::item::ShapeType::Arc => {
                    shape_raw.shape_angle.or(Some(45.0))
                }
                _ => None,
            },
            _ => None,
        },
        corners: shape_raw.shape_corners,
    };

    let paint = paint::from_raw_item(shape_raw);

    let title = shape_raw.internal_title.clone();
    let anchor = anchor::from_raw_item(shape_raw, is_root);
    let offset = offset::from_raw_item(shape_raw);
    let padding = padding::from_raw_item(shape_raw);

    Item::Shape(Shape {
        data,
        paint,
        id: id.to_owned(),
        title,
        is_root: if is_root { Some(true) } else { None },
        anchor,
        offset,
        padding,
    })
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"export type ShapeDataType =
    | 'RECT'
    | 'SQUARE'
    | 'CIRCLE'
    | 'OVAL'
    | 'TRIANGLE'
    | 'RIGHT_TRIANGLE'
    | 'HEXAGON'
    | 'SLICE'
    | 'ARC'
    | 'SQUIRCLE';

export interface BaseShapeData {
    type: ShapeDataType,
    width: number,
    corners?: number,
}

export interface ShapeDataRectangle extends BaseShapeData {
    type: 'RECT';
    height: number;
}

export interface ShapeDataSquare extends BaseShapeData {
    type: 'SQUARE';
}

export interface ShapeDataCircle extends BaseShapeData {
    type: 'CIRCLE';
}

export interface ShapeDataOval extends BaseShapeData {
    type: 'OVAL';
    height: number;
}

export interface ShapeDataTriangle extends BaseShapeData {
    type: 'TRIANGLE';
    height: number;
}

export interface ShapeDataRightTriangle extends BaseShapeData {
    type: 'RIGHT_TRIANGLE';
    height: number;
}

export interface ShapeDataHexagon extends BaseShapeData {
    type: 'HEXAGON';
}

export interface ShapeDataSlice extends BaseShapeData {
    type: 'SLICE';
    height: number;
    angle: number;
}

export interface ShapeDataArc extends BaseShapeData {
    type: 'ARC';
    height: number;
    angle: number;
}

export interface ShapeDataSquircle extends BaseShapeData {
    type: 'SQUIRCLE';
    height: number;
}

export type ShapeData =
    | ShapeDataRectangle
    | ShapeDataSquare
    | ShapeDataCircle
    | ShapeDataOval
    | ShapeDataTriangle
    | ShapeDataRightTriangle
    | ShapeDataHexagon
    | ShapeDataSlice
    | ShapeDataArc
    | ShapeDataSquircle;

export interface Shape {
    type: 'SHAPE',
    data: ShapeData,
    paint: Paint,
    id: string,
    title: string,
    isRoot: boolean,
    anchor: Anchor,
    offset: Offset,
    padding: Padding
}"#;

#[cfg(test)]
mod tests {
    use super::{from_raw_klwp, klwp, Item, ShapeDataType};
    use std::matches;

    #[test]
    fn it_parses_base_item_correctly() {
        let shape_raw = klwp::item::tests::base_item(klwp::item::InternalType::ShapeModule);

        let item_parsed = from_raw_klwp(&shape_raw, "0", false);
        match item_parsed {
            Item::Shape(v) => {
                assert_eq!(v.id, "0");
                assert_eq!(v.title, None);
                assert_eq!(v.is_root, None);
                assert!(matches!(v.data.r#type, ShapeDataType::Square));
                assert_eq!(v.data.width, 20.0);
                assert_eq!(v.data.height, None);
                assert_eq!(v.data.angle, None);
                assert_eq!(v.data.corners, None);
            }
            _ => panic!("Expected parsed item to be Shape!"),
        }
    }

    #[test]
    fn it_parses_square_correctly() {
        let mut shape_raw = klwp::item::tests::base_item(klwp::item::InternalType::ShapeModule);

        shape_raw.shape_width = Some(10.0);
        shape_raw.shape_height = Some(20.0);
        shape_raw.shape_angle = Some(30.0);
        shape_raw.shape_corners = Some(40.0);

        let item_parsed = from_raw_klwp(&shape_raw, "0", false);
        match item_parsed {
            Item::Shape(v) => {
                assert!(matches!(v.data.r#type, ShapeDataType::Square));
                assert_eq!(v.data.width, 10.0);
                assert_eq!(v.data.height, None);
                assert_eq!(v.data.angle, None);
                assert_eq!(v.data.corners, Some(40.0));
            }
            _ => panic!("Expected parsed item to be Shape!"),
        }
    }

    #[test]
    fn it_parses_rect_correctly() {
        let mut shape_raw = klwp::item::tests::base_item(klwp::item::InternalType::ShapeModule);

        shape_raw.shape_type = Some(klwp::item::ShapeType::Rect);
        shape_raw.shape_width = Some(10.0);
        shape_raw.shape_height = Some(20.0);
        shape_raw.shape_angle = Some(30.0);
        shape_raw.shape_corners = Some(40.0);

        let item_parsed = from_raw_klwp(&shape_raw, "0", false);
        match item_parsed {
            Item::Shape(v) => {
                assert!(matches!(v.data.r#type, ShapeDataType::Rectangle));
                assert_eq!(v.data.width, 10.0);
                assert_eq!(v.data.height, Some(20.0));
                assert_eq!(v.data.angle, None);
                assert_eq!(v.data.corners, Some(40.0));
            }
            _ => panic!("Expected parsed item to be Shape!"),
        }
    }

    #[test]
    fn it_parses_circle_correctly() {
        let mut shape_raw = klwp::item::tests::base_item(klwp::item::InternalType::ShapeModule);

        shape_raw.shape_type = Some(klwp::item::ShapeType::Circle);
        shape_raw.shape_width = Some(10.0);
        shape_raw.shape_height = Some(20.0);
        shape_raw.shape_angle = Some(30.0);
        shape_raw.shape_corners = Some(40.0);

        let item_parsed = from_raw_klwp(&shape_raw, "0", false);
        match item_parsed {
            Item::Shape(v) => {
                assert!(matches!(v.data.r#type, ShapeDataType::Circle));
                assert_eq!(v.data.width, 10.0);
                assert_eq!(v.data.height, None);
                assert_eq!(v.data.angle, None);
                assert_eq!(v.data.corners, Some(40.0));
            }
            _ => panic!("Expected parsed item to be Shape!"),
        }
    }
    #[test]
    fn it_parses_oval_correctly() {
        let mut shape_raw = klwp::item::tests::base_item(klwp::item::InternalType::ShapeModule);

        shape_raw.shape_type = Some(klwp::item::ShapeType::Oval);
        shape_raw.shape_width = Some(10.0);
        shape_raw.shape_height = Some(20.0);
        shape_raw.shape_angle = Some(30.0);
        shape_raw.shape_corners = Some(40.0);

        let item_parsed = from_raw_klwp(&shape_raw, "0", false);
        match item_parsed {
            Item::Shape(v) => {
                assert!(matches!(v.data.r#type, ShapeDataType::Oval));
                assert_eq!(v.data.width, 10.0);
                assert_eq!(v.data.height, Some(20.0));
                assert_eq!(v.data.angle, None);
                assert_eq!(v.data.corners, Some(40.0));
            }
            _ => panic!("Expected parsed item to be Shape!"),
        }
    }
    #[test]
    fn it_parses_triangle_correctly() {
        let mut shape_raw = klwp::item::tests::base_item(klwp::item::InternalType::ShapeModule);

        shape_raw.shape_type = Some(klwp::item::ShapeType::Triangle);
        shape_raw.shape_width = Some(10.0);
        shape_raw.shape_height = Some(20.0);
        shape_raw.shape_angle = Some(30.0);
        shape_raw.shape_corners = Some(40.0);

        let item_parsed = from_raw_klwp(&shape_raw, "0", false);
        match item_parsed {
            Item::Shape(v) => {
                assert!(matches!(v.data.r#type, ShapeDataType::Triangle));
                assert_eq!(v.data.width, 10.0);
                assert_eq!(v.data.height, Some(20.0));
                assert_eq!(v.data.angle, None);
                assert_eq!(v.data.corners, Some(40.0));
            }
            _ => panic!("Expected parsed item to be Shape!"),
        }
    }
    #[test]
    fn it_parses_right_triangle_correctly() {
        let mut shape_raw = klwp::item::tests::base_item(klwp::item::InternalType::ShapeModule);

        shape_raw.shape_type = Some(klwp::item::ShapeType::RTriangle);
        shape_raw.shape_width = Some(10.0);
        shape_raw.shape_height = Some(20.0);
        shape_raw.shape_angle = Some(30.0);
        shape_raw.shape_corners = Some(40.0);

        let item_parsed = from_raw_klwp(&shape_raw, "0", false);
        match item_parsed {
            Item::Shape(v) => {
                assert!(matches!(v.data.r#type, ShapeDataType::RightTriangle));
                assert_eq!(v.data.width, 10.0);
                assert_eq!(v.data.height, Some(20.0));
                assert_eq!(v.data.angle, None);
                assert_eq!(v.data.corners, Some(40.0));
            }
            _ => panic!("Expected parsed item to be Shape!"),
        }
    }
    #[test]
    fn it_parses_hexagon_correctly() {
        let mut shape_raw = klwp::item::tests::base_item(klwp::item::InternalType::ShapeModule);

        shape_raw.shape_type = Some(klwp::item::ShapeType::Exagon);
        shape_raw.shape_width = Some(10.0);
        shape_raw.shape_height = Some(20.0);
        shape_raw.shape_angle = Some(30.0);
        shape_raw.shape_corners = Some(40.0);

        let item_parsed = from_raw_klwp(&shape_raw, "0", false);
        match item_parsed {
            Item::Shape(v) => {
                assert!(matches!(v.data.r#type, ShapeDataType::Hexagon));
                assert_eq!(v.data.width, 10.0);
                assert_eq!(v.data.height, Some(20.0));
                assert_eq!(v.data.angle, None);
                assert_eq!(v.data.corners, Some(40.0));
            }
            _ => panic!("Expected parsed item to be Shape!"),
        }
    }
    #[test]
    fn it_parses_slice_correctly() {
        let mut shape_raw = klwp::item::tests::base_item(klwp::item::InternalType::ShapeModule);

        shape_raw.shape_type = Some(klwp::item::ShapeType::Slice);
        shape_raw.shape_width = Some(10.0);
        shape_raw.shape_height = Some(20.0);
        shape_raw.shape_angle = Some(30.0);
        shape_raw.shape_corners = Some(40.0);

        let item_parsed = from_raw_klwp(&shape_raw, "0", false);
        match item_parsed {
            Item::Shape(v) => {
                assert!(matches!(v.data.r#type, ShapeDataType::Slice));
                assert_eq!(v.data.width, 10.0);
                assert_eq!(v.data.height, Some(20.0));
                assert_eq!(v.data.angle, Some(30.0));
                assert_eq!(v.data.corners, Some(40.0));
            }
            _ => panic!("Expected parsed item to be Shape!"),
        }
    }
    #[test]
    fn it_parses_arc_correctly() {
        let mut shape_raw = klwp::item::tests::base_item(klwp::item::InternalType::ShapeModule);

        shape_raw.shape_type = Some(klwp::item::ShapeType::Arc);
        shape_raw.shape_width = Some(10.0);
        shape_raw.shape_height = Some(20.0);
        shape_raw.shape_angle = Some(30.0);
        shape_raw.shape_corners = Some(40.0);

        let item_parsed = from_raw_klwp(&shape_raw, "0", false);
        match item_parsed {
            Item::Shape(v) => {
                assert!(matches!(v.data.r#type, ShapeDataType::Arc));
                assert_eq!(v.data.width, 10.0);
                assert_eq!(v.data.height, Some(20.0));
                assert_eq!(v.data.angle, Some(30.0));
                assert_eq!(v.data.corners, Some(40.0));
            }
            _ => panic!("Expected parsed item to be Shape!"),
        }
    }
    #[test]
    fn it_parses_squircle_correctly() {
        let mut shape_raw = klwp::item::tests::base_item(klwp::item::InternalType::ShapeModule);

        shape_raw.shape_type = Some(klwp::item::ShapeType::Squircle);
        shape_raw.shape_width = Some(10.0);
        shape_raw.shape_height = Some(20.0);
        shape_raw.shape_angle = Some(30.0);
        shape_raw.shape_corners = Some(40.0);

        let item_parsed = from_raw_klwp(&shape_raw, "0", false);
        match item_parsed {
            Item::Shape(v) => {
                assert!(matches!(v.data.r#type, ShapeDataType::Squircle));
                assert_eq!(v.data.width, 10.0);
                assert_eq!(v.data.height, Some(20.0));
                assert_eq!(v.data.angle, None);
                assert_eq!(v.data.corners, Some(40.0));
            }
            _ => panic!("Expected parsed item to be Shape!"),
        }
    }
}
