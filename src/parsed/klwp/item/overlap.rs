use super::{data::*, klwp, Item};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct Overlap {
    pub items: Vec<String>,

    pub id: String,
    pub title: Option<String>,
    #[serde(rename(serialize = "isRoot"))]
    pub is_root: Option<bool>,
    pub anchor: Option<anchor::Anchor>,
    pub offset: Option<offset::Offset>,
    pub padding: Option<padding::Padding>,
}

pub fn from_raw_klwp(
    overlap_raw: &klwp::item::Item,
    id: &str,
    data: &mut HashMap<String, super::Item>,
    is_root: Option<bool>,
) -> Item {
    let title = overlap_raw.internal_title.clone();
    let anchor = anchor::from_raw_item(overlap_raw);
    let offset = offset::from_raw_item(overlap_raw);
    let padding = padding::from_raw_item(overlap_raw);

    let items = match &overlap_raw.viewgroup_items {
        Some(items_raw) => {
            let mut res = Vec::with_capacity(items_raw.len());
            for (i, item_raw) in items_raw.into_iter().enumerate() {
                let id = id.to_string() + "-" + &i.to_string();
                let item = super::from_raw_klwp(&item_raw, &id, data, None);
                res.push(id.clone());

                data.insert(id, item);
            }

            res
        }
        _ => panic!("Expected field 'viewgroup_items'!"),
    };

    Item::Overlap(Overlap {
        items,
        id: id.to_owned(),
        title,
        is_root,
        anchor,
        offset,
        padding,
    })
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"export interface Overlap {
    type: 'OVERLAP',
    items: string[],
    
    id: string,
    title?: string,
    isRoot?: boolean,
    anchor?: Anchor,
    offset?: Offset,
    padding?: Padding
}"#;

#[cfg(test)]
mod tests {
    use super::{from_raw_klwp, klwp, Item};
    use std::{collections::HashMap, matches};

    #[test]
    fn it_parses_correctly() {
        let mut data = HashMap::new();
        let overlap_raw =
            klwp::item::tests::base_item(klwp::item::InternalType::OverlapLayerModule);

        let item_parsed = from_raw_klwp(&overlap_raw, "0", &mut data, None);
        match item_parsed {
            Item::Overlap(v) => {
                assert_eq!(v.id, "0");
                assert_eq!(v.title, None);
                assert_eq!(v.is_root, None);
                assert_eq!(v.items.len(), 0);
            }
            _ => panic!("Expected parsed item to be Overlap!"),
        }
    }

    #[test]
    fn it_parses_data_correctly() {
        let mut data = HashMap::new();
        let mut overlap_raw =
            klwp::item::tests::base_item(klwp::item::InternalType::OverlapLayerModule);

        let item_raw_1 = klwp::item::tests::base_item(klwp::item::InternalType::ShapeModule);
        let item_raw_2 = klwp::item::tests::base_item(klwp::item::InternalType::TextModule);
        overlap_raw.viewgroup_items = Some(vec![item_raw_1, item_raw_2]);
        overlap_raw.internal_title = Some("TITLE".to_owned());

        let item_parsed = from_raw_klwp(&overlap_raw, "0", &mut data, Some(true));
        match item_parsed {
            Item::Overlap(v) => {
                assert_eq!(v.id, "0");
                assert_eq!(v.title, Some("TITLE".to_owned()));
                assert_eq!(v.is_root, Some(true));
                assert_eq!(v.items.len(), 2);
                assert_eq!(data.len(), 2);
                assert!(matches!(data.get("0-0").unwrap(), Item::Shape(_)));
                assert!(matches!(data.get("0-1").unwrap(), Item::Text(_)));
            }
            _ => panic!("Expected parsed item to be Overlap!"),
        }
    }
}
