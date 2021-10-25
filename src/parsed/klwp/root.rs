use super::{
    data::{padding, paint},
    item,
    raw::klwp,
};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct Root {
    items: Vec<String>,
    data: HashMap<String, item::Item>,
    paint: Option<paint::Paint>,
    padding: Option<padding::Padding>,
}

pub fn from_raw_klwp(root_raw: klwp::root::Root) -> Root {
    let paint = paint::from_raw_root(&root_raw);
    let padding = padding::from_raw_root(&root_raw);

    // root items - list of references
    let mut items = Vec::with_capacity(root_raw.viewgroup_items.len());
    let mut data = HashMap::new();
    for (i, item_raw) in root_raw.viewgroup_items.into_iter().enumerate() {
        let id = i.to_string();
        items.push(id.clone());
        let item = item::from_raw_klwp(&item_raw, &id, &mut data, Some(true));
        data.insert(id, item);
    }

    Root {
        items,
        data,
        paint,
        padding,
    }
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"export interface Root {
    items: string[],
    data: {
        [key: string]: Item
    },
    paint?: Paint,
    padding?: Padding
}"#;
