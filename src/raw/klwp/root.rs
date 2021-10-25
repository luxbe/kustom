use super::item;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct Root {
    pub background_color: Option<String>,
    pub viewgroup_items: Vec<item::Item>,

    pub position_padding_top: Option<f32>,
    pub position_padding_right: Option<f32>,
    pub position_padding_bottom: Option<f32>,
    pub position_padding_left: Option<f32>,
}

#[cfg(test)]
pub mod tests {
    use super::Root;

    pub fn raw_root() -> Root {
        Root {
            background_color: None,
            viewgroup_items: vec![],
            position_padding_top: None,
            position_padding_right: None,
            position_padding_bottom: None,
            position_padding_left: None,
        }
    }
}
