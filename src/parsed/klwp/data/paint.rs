use super::klwp;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Paint {
    color: String,
}

pub fn from_raw_root(root_raw: &klwp::root::Root) -> Paint {
    Paint {
        color: root_raw
            .background_color
            .clone()
            .unwrap_or(String::from("#FF000000")),
    }
}

pub fn from_raw_item(item_raw: &klwp::item::Item) -> Paint {
    Paint {
        color: item_raw
            .paint_color
            .clone()
            .unwrap_or(String::from("#FFFFFFFF")),
    }
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"export interface Paint {
    color?: string
}"#;

#[cfg(test)]
mod tests {
    use super::{from_raw_item, from_raw_root, klwp};

    #[test]
    fn it_parses_item_none_correctly() {
        let shape_raw = klwp::item::tests::base_item(klwp::item::InternalType::ShapeModule);

        let paint = from_raw_item(&shape_raw);
        assert_eq!(paint.color, "#FFFFFFFF");
    }

    #[test]
    fn it_parses_item_correctly() {
        let mut shape_raw = klwp::item::tests::base_item(klwp::item::InternalType::ShapeModule);

        shape_raw.paint_color = Some(String::from("#FF000000"));

        let paint = from_raw_item(&shape_raw);
        assert_eq!(paint.color, "#FF000000");
    }

    #[test]
    fn it_parses_root_none_correctly() {
        let root_raw = klwp::root::tests::raw_root();

        let paint = from_raw_root(&root_raw);
        assert_eq!(paint.color, "#FF000000");
    }

    #[test]
    fn it_parses_root_correctly() {
        let mut root_raw = klwp::root::tests::raw_root();

        root_raw.background_color = Some("#AARRGGBB".to_owned());

        let paint = from_raw_root(&root_raw);
        assert_eq!(paint.color, "#AARRGGBB");
    }
}
