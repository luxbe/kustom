use super::klwp;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AnchorType {
    Start,
    Center,
    End,
}

#[derive(Serialize, Deserialize)]
pub struct Anchor {
    h: AnchorType,
    v: AnchorType,
}

pub fn from_raw_item(item_raw: &klwp::item::Item) -> Option<Anchor> {
    match &item_raw.position_anchor {
        None => None,
        Some(v) => {
            let h = match v {
                klwp::item::PositionAnchor::TOP
                | klwp::item::PositionAnchor::CENTER
                | klwp::item::PositionAnchor::BOTTOM => AnchorType::Center,
                klwp::item::PositionAnchor::TOPLEFT
                | klwp::item::PositionAnchor::CENTERLEFT
                | klwp::item::PositionAnchor::BOTTOMLEFT => AnchorType::Start,
                klwp::item::PositionAnchor::TOPRIGHT
                | klwp::item::PositionAnchor::CENTERRIGHT
                | klwp::item::PositionAnchor::BOTTOMRIGHT => AnchorType::End,
            };

            let v = match v {
                klwp::item::PositionAnchor::CENTER
                | klwp::item::PositionAnchor::CENTERLEFT
                | klwp::item::PositionAnchor::CENTERRIGHT => AnchorType::Center,
                klwp::item::PositionAnchor::TOP
                | klwp::item::PositionAnchor::TOPLEFT
                | klwp::item::PositionAnchor::TOPRIGHT => AnchorType::Start,
                klwp::item::PositionAnchor::BOTTOM
                | klwp::item::PositionAnchor::BOTTOMLEFT
                | klwp::item::PositionAnchor::BOTTOMRIGHT => AnchorType::End,
            };
            Some(Anchor { h, v })
        }
    }
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"export type AnchorType =
    | 'START'
    | 'CENTER'
    | 'END'

export interface Anchor {
    h: AnchorType
    v: AnchorType;
}"#;

#[cfg(test)]
mod tests {
    use super::{from_raw_item, klwp, AnchorType};
    use std::matches;

    #[test]
    fn it_parses_none_correctly() {
        let shape_raw = klwp::item::tests::base_item(klwp::item::InternalType::ShapeModule);

        let anchor_raw = from_raw_item(&shape_raw);
        assert!(anchor_raw.is_none());
    }

    #[test]
    fn it_parses_existing_correctly() {
        let mut shape_raw = klwp::item::tests::base_item(klwp::item::InternalType::ShapeModule);

        shape_raw.position_anchor = Some(klwp::item::PositionAnchor::TOP);
        let anchor_raw = from_raw_item(&shape_raw);
        assert!(anchor_raw.is_some());
        let anchor = anchor_raw.unwrap();
        assert!(matches!(anchor.h, AnchorType::Center));
        assert!(matches!(anchor.v, AnchorType::Start));

        shape_raw.position_anchor = Some(klwp::item::PositionAnchor::TOPLEFT);
        let anchor_raw = from_raw_item(&shape_raw);
        assert!(anchor_raw.is_some());
        let anchor = anchor_raw.unwrap();
        assert!(matches!(anchor.h, AnchorType::Start));
        assert!(matches!(anchor.v, AnchorType::Start));

        shape_raw.position_anchor = Some(klwp::item::PositionAnchor::TOPRIGHT);
        let anchor_raw = from_raw_item(&shape_raw);
        assert!(anchor_raw.is_some());
        let anchor = anchor_raw.unwrap();
        assert!(matches!(anchor.h, AnchorType::End));
        assert!(matches!(anchor.v, AnchorType::Start));

        shape_raw.position_anchor = Some(klwp::item::PositionAnchor::CENTER);
        let anchor_raw = from_raw_item(&shape_raw);
        assert!(anchor_raw.is_some());
        let anchor = anchor_raw.unwrap();
        assert!(matches!(anchor.h, AnchorType::Center));
        assert!(matches!(anchor.v, AnchorType::Center));

        shape_raw.position_anchor = Some(klwp::item::PositionAnchor::CENTERLEFT);
        let anchor_raw = from_raw_item(&shape_raw);
        assert!(anchor_raw.is_some());
        let anchor = anchor_raw.unwrap();
        assert!(matches!(anchor.h, AnchorType::Start));
        assert!(matches!(anchor.v, AnchorType::Center));

        shape_raw.position_anchor = Some(klwp::item::PositionAnchor::CENTERRIGHT);
        let anchor_raw = from_raw_item(&shape_raw);
        assert!(anchor_raw.is_some());
        let anchor = anchor_raw.unwrap();
        assert!(matches!(anchor.h, AnchorType::End));
        assert!(matches!(anchor.v, AnchorType::Center));

        shape_raw.position_anchor = Some(klwp::item::PositionAnchor::BOTTOM);
        let anchor_raw = from_raw_item(&shape_raw);
        assert!(anchor_raw.is_some());
        let anchor = anchor_raw.unwrap();
        assert!(matches!(anchor.h, AnchorType::Center));
        assert!(matches!(anchor.v, AnchorType::End));

        shape_raw.position_anchor = Some(klwp::item::PositionAnchor::BOTTOMLEFT);
        let anchor_raw = from_raw_item(&shape_raw);
        assert!(anchor_raw.is_some());
        let anchor = anchor_raw.unwrap();
        assert!(matches!(anchor.h, AnchorType::Start));
        assert!(matches!(anchor.v, AnchorType::End));

        shape_raw.position_anchor = Some(klwp::item::PositionAnchor::BOTTOMRIGHT);
        let anchor_raw = from_raw_item(&shape_raw);
        assert!(anchor_raw.is_some());
        let anchor = anchor_raw.unwrap();
        assert!(matches!(anchor.h, AnchorType::End));
        assert!(matches!(anchor.v, AnchorType::End));
    }
}
