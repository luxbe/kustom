use super::item;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct Root {
    pub background_color: Option<String>,
    pub viewgroup_items: Vec<item::Item>,
}

#[cfg(test)]
pub mod tests {
    use super::Root;

    pub fn raw_root() -> Root {
        Root {
            background_color: None,
            viewgroup_items: vec![],
        }
    }
}
