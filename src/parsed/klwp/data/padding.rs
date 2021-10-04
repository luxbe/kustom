use super::klwp;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Padding {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
}

pub fn from_raw_item(item_raw: &klwp::item::Item) -> Padding {
    Padding {
        top: item_raw.position_padding_top.unwrap_or(0.0),
        right: item_raw.position_padding_right.unwrap_or(0.0),
        bottom: item_raw.position_padding_bottom.unwrap_or(0.0),
        left: item_raw.position_padding_left.unwrap_or(0.0),
    }
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"export interface Padding {
    top: number;
    right: number;
    bottom: number;
    left: number;
}"#;

#[cfg(test)]
mod tests {
    use super::{from_raw_item, klwp};

    #[test]
    fn it_parses_none_correctly() {
        let shape_raw = klwp::item::tests::base_item(klwp::item::InternalType::ShapeModule);

        let padding = from_raw_item(&shape_raw);
        assert_eq!(padding.top, 0.0);
        assert_eq!(padding.right, 0.0);
        assert_eq!(padding.bottom, 0.0);
        assert_eq!(padding.left, 0.0);
    }

    #[test]
    fn it_parses_correctly() {
        let mut shape_raw = klwp::item::tests::base_item(klwp::item::InternalType::ShapeModule);

        shape_raw.position_padding_top = Some(10.0);
        shape_raw.position_padding_right = Some(20.0);
        shape_raw.position_padding_bottom = Some(30.0);
        shape_raw.position_padding_left = Some(40.0);

        let padding = from_raw_item(&shape_raw);
        assert_eq!(padding.top, 10.0);
        assert_eq!(padding.right, 20.0);
        assert_eq!(padding.bottom, 30.0);
        assert_eq!(padding.left, 40.0);
    }
}
