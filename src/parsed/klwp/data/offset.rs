use super::klwp;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Offset {
    x: f32,
    y: f32,
}

pub fn from_raw_item(item_raw: &klwp::item::Item) -> Option<Offset> {
    if item_raw.position_offset_x.is_none() && item_raw.position_offset_y.is_none() {
        return None;
    }
    Some(Offset {
        x: item_raw.position_offset_x.unwrap_or(0.0),
        y: item_raw.position_offset_y.unwrap_or(0.0),
    })
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"export interface Offset {
    x: number;
    y: number;
}"#;

#[cfg(test)]
mod tests {
    use super::{from_raw_item, klwp};

    #[test]
    fn it_parses_none_correctly() {
        let shape_raw = klwp::item::tests::base_item(klwp::item::InternalType::ShapeModule);

        let offset_raw = from_raw_item(&shape_raw);
        assert!(offset_raw.is_none());
    }

    #[test]
    fn it_parses_correctly() {
        let mut shape_raw = klwp::item::tests::base_item(klwp::item::InternalType::ShapeModule);

        shape_raw.position_offset_x = Some(20.0);
        shape_raw.position_offset_y = Some(30.0);

        let offset_raw = from_raw_item(&shape_raw);
        assert!(offset_raw.is_some());
        let offset = offset_raw.unwrap();
        assert_eq!(offset.x, 20.0);
        assert_eq!(offset.y, 30.0);

        shape_raw.position_offset_x = None;
        shape_raw.position_offset_y = Some(20.0);

        let offset_raw = from_raw_item(&shape_raw);
        assert!(offset_raw.is_some());
        let offset = offset_raw.unwrap();
        assert_eq!(offset.x, 0.0);
        assert_eq!(offset.y, 20.0);

        shape_raw.position_offset_x = Some(20.0);
        shape_raw.position_offset_y = None;

        let offset_raw = from_raw_item(&shape_raw);
        assert!(offset_raw.is_some());
        let offset = offset_raw.unwrap();
        assert_eq!(offset.x, 20.0);
        assert_eq!(offset.y, 0.0);
    }
}
