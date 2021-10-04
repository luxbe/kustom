use super::{item, raw::klwp};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Root {
    items: Vec<String>,
    paint: super::data::paint::Paint,
    data: HashMap<String, item::Item>,
}

pub fn from_raw_klwp(root_raw: klwp::root::Root) -> Root {
    let paint = super::data::paint::from_raw_root(&root_raw);

    // root items - list of references
    let mut items = Vec::with_capacity(root_raw.viewgroup_items.len());
    let mut data = HashMap::new();
    for (i, item_raw) in root_raw.viewgroup_items.into_iter().enumerate() {
        let id = i.to_string();
        items.push(id.clone());
        let item = item::from_raw_klwp(&item_raw, &id, &mut data, true);
        data.insert(id, item);
    }

    Root { items, paint, data }
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"export interface Root {
    items: string[],
    paint: Paint,
    data: {
        [key: string]: Item
    }
}"#;
