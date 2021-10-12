mod overlap;
mod shape;
mod stack;
mod text;

use super::{data, raw::klwp};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "UPPERCASE")]
pub enum Item {
    Overlap(overlap::Overlap),
    Stack(stack::Stack),
    Shape(shape::Shape),
    Text(text::Text),
}

pub fn from_raw_klwp(
    item_raw: &klwp::item::Item,
    id: &str,
    data: &mut HashMap<String, Item>,
    is_root: Option<bool>,
) -> Item {
    match item_raw.internal_type {
        klwp::item::InternalType::OverlapLayerModule => {
            overlap::from_raw_klwp(&item_raw, &id, data, is_root)
        }
        klwp::item::InternalType::StackLayerModule => {
            stack::from_raw_klwp(&item_raw, &id, data, is_root)
        }
        klwp::item::InternalType::ShapeModule => shape::from_raw_klwp(&item_raw, &id, is_root),
        klwp::item::InternalType::TextModule => text::from_raw_klwp(&item_raw, &id, is_root),
    }
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"export type Item = 
    | Overlap
    | Stack
    | Shape
    | Text;"#;

#[cfg(test)]
mod tests {
    use super::{from_raw_klwp, klwp, Item};
    use std::{collections::HashMap, matches};

    #[test]
    fn it_parses_raw_overlap_correctly() {
        let mut data = HashMap::new();
        let overlap_raw =
            klwp::item::tests::base_item(klwp::item::InternalType::OverlapLayerModule);

        let overlap_parsed = from_raw_klwp(&overlap_raw, "0", &mut data, None);
        assert!(matches!(overlap_parsed, Item::Overlap(_)));
    }

    #[test]
    fn it_parses_raw_stack_correctly() {
        let mut data = HashMap::new();
        let stack_raw = klwp::item::tests::base_item(klwp::item::InternalType::StackLayerModule);

        let stack_parsed = from_raw_klwp(&stack_raw, "0", &mut data, None);
        assert!(matches!(stack_parsed, Item::Stack(_)));
    }

    #[test]
    fn it_parses_raw_shape_correctly() {
        let mut data = HashMap::new();
        let shape_raw = klwp::item::tests::base_item(klwp::item::InternalType::ShapeModule);

        let shape_parsed = from_raw_klwp(&shape_raw, "0", &mut data, None);
        assert!(matches!(shape_parsed, Item::Shape(_)));
    }

    #[test]
    fn it_parses_raw_text_correctly() {
        let mut data = HashMap::new();
        let text_raw = klwp::item::tests::base_item(klwp::item::InternalType::TextModule);

        let text_parsed = from_raw_klwp(&text_raw, "0", &mut data, None);
        assert!(matches!(text_parsed, Item::Text(_)));
    }
}
