use super::{data::*, klwp, Item};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use wasm_bindgen::prelude::*;

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct Text {
    pub data: TextData,
    pub paint: paint::Paint,
    pub id: String,
    pub title: Option<String>,
    pub is_root: Option<bool>,
    pub anchor: anchor::Anchor,
    pub offset: offset::Offset,
    pub padding: padding::Padding,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct TextData {
    r#type: TextDataType,
    content: String,
    family: Option<String>,
    size: Option<f32>,
    width: Option<f32>,
    height: Option<f32>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "SCREAMING_SNAKE_CASE")]
pub enum TextDataType {
    FixedFontHeight,
    FixedWidth,
    FitWidth,
    FitToBox,
}

pub fn from_raw_klwp(text_raw: &klwp::item::Item, id: &str, is_root: bool) -> Item {
    let data = TextData {
        r#type: match &text_raw.text_size_type {
            Some(v) => match v {
                klwp::item::TextSizeType::FitToBox => TextDataType::FitToBox,
                klwp::item::TextSizeType::FitWidth => TextDataType::FitWidth,
                klwp::item::TextSizeType::FixedWidth => TextDataType::FixedWidth,
            },
            _ => TextDataType::FixedFontHeight,
        },
        content: text_raw
            .text_expression
            .clone()
            .unwrap_or("$df(hh:mm:ss)$".to_owned()),
        family: text_raw.text_family.clone(),
        size: match &text_raw.text_size_type {
            Some(v) => match v {
                klwp::item::TextSizeType::FixedWidth => text_raw.text_size.or(Some(20.0)),
                _ => None,
            },
            _ => text_raw.text_size.or(Some(20.0)),
        },
        width: match &text_raw.text_size_type {
            Some(_) => text_raw.text_width.or(Some(20.0)),
            _ => None,
        },
        height: match &text_raw.text_size_type {
            Some(v) => match v {
                klwp::item::TextSizeType::FitToBox => text_raw.text_height.or(Some(20.0)),
                _ => None,
            },
            _ => None,
        },
    };

    let paint = paint::from_raw_item(text_raw);

    let title = text_raw.internal_title.clone();
    let anchor = anchor::from_raw_item(text_raw, is_root);
    let offset = offset::from_raw_item(text_raw);
    let padding = padding::from_raw_item(text_raw);

    Item::Text(Text {
        data,
        paint,
        id: id.to_owned(),
        title,
        is_root: if is_root { Some(true) } else { None },
        anchor,
        offset,
        padding,
    })
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"export type TextSizeType =
    | 'FIXED_FONT_HEIGHT'
    | 'FIXED_WIDTH'
    | 'FIT_WIDTH'
    | 'FIT_TO_BOX';

export interface BaseTextData {
    content: string;
    type: TextSizeType;
    family?: string;
}

export interface TextDataFixedFontHeight extends BaseTextData {
    type: 'FIXED_FONT_HEIGHT';
    size: number;
}

export interface TextDataFitWidth extends BaseTextData {
    type: 'FIT_WIDTH';
    width: number;
}

export interface TextDataFixedWidth extends BaseTextData {
    type: 'FIXED_WIDTH';
    size: number;
    width: number;
}

export interface TextDataFitToBox extends BaseTextData {
    type: 'FIT_TO_BOX';
    width: number;
    height: number;
}

export type TextData =
    | TextDataFixedFontHeight
    | TextDataFixedWidth
    | TextDataFitWidth
    | TextDataFitToBox;

export interface Text {
    data: TextData,
    type: 'TEXT'
    paint: Paint,
    id: string,
    title: string,
    anchor: Anchor,
    offset: Offset,
    padding: Padding
}"#;

#[cfg(test)]
mod tests {
    use super::{from_raw_klwp, klwp, Item, TextDataType};

    #[test]
    fn it_parses_correctly() {
        let text_raw = klwp::item::tests::base_item(klwp::item::InternalType::TextModule);

        let item_parsed = from_raw_klwp(&text_raw, "0", false);
        match item_parsed {
            Item::Text(v) => {
                assert_eq!(v.id, "0");
                assert_eq!(v.title, None);
                assert_eq!(v.is_root, None);
            }
            _ => panic!("Expected parsed item to be Text!"),
        }
    }

    #[test]
    fn it_parses_base_item_correctly() {
        let mut text_raw = klwp::item::tests::base_item(klwp::item::InternalType::TextModule);
        text_raw.internal_title = Some("TITLE".to_owned());

        let item_parsed = from_raw_klwp(&text_raw, "0", true);
        match item_parsed {
            Item::Text(v) => {
                assert_eq!(v.id, "0");
                assert_eq!(v.title, Some("TITLE".to_owned()));
                assert_eq!(v.is_root, Some(true));
                assert!(matches!(v.data.r#type, TextDataType::FixedFontHeight));
                assert_eq!(v.data.content, "$df(hh:mm:ss)$");
                assert_eq!(v.data.family, None);
                assert_eq!(v.data.size, Some(20.0));
                assert_eq!(v.data.width, None);
                assert_eq!(v.data.height, None);
            }
            _ => panic!("Expected parsed item to be Text!"),
        }
    }
    #[test]
    fn it_parses_fixed_font_height_correctly() {
        let mut text_raw = klwp::item::tests::base_item(klwp::item::InternalType::TextModule);
        text_raw.text_expression = Some("TEST CONTENT".to_owned());
        text_raw.text_family = Some("TEST FAMILY".to_owned());
        text_raw.text_size = Some(10.0);
        text_raw.text_width = Some(20.0);
        text_raw.text_height = Some(30.0);

        let item_parsed = from_raw_klwp(&text_raw, "0", true);
        match item_parsed {
            Item::Text(v) => {
                assert!(matches!(v.data.r#type, TextDataType::FixedFontHeight));
                assert_eq!(v.data.content, "TEST CONTENT");
                assert_eq!(v.data.family.unwrap(), "TEST FAMILY");
                assert_eq!(v.data.size, Some(10.0));
                assert_eq!(v.data.width, None);
                assert_eq!(v.data.height, None);
            }
            _ => panic!("Expected parsed item to be Text!"),
        }
    }

    #[test]
    fn it_parses_fixed_width_correctly() {
        let mut text_raw = klwp::item::tests::base_item(klwp::item::InternalType::TextModule);
        text_raw.text_size_type = Some(klwp::item::TextSizeType::FixedWidth);
        text_raw.text_expression = Some("TEST CONTENT".to_owned());
        text_raw.text_family = Some("TEST FAMILY".to_owned());
        text_raw.text_size = Some(10.0);
        text_raw.text_width = Some(20.0);
        text_raw.text_height = Some(30.0);

        let item_parsed = from_raw_klwp(&text_raw, "0", true);
        match item_parsed {
            Item::Text(v) => {
                assert!(matches!(v.data.r#type, TextDataType::FixedWidth));
                assert_eq!(v.data.content, "TEST CONTENT");
                assert_eq!(v.data.family.unwrap(), "TEST FAMILY");
                assert_eq!(v.data.size, Some(10.0));
                assert_eq!(v.data.width, Some(20.0));
                assert_eq!(v.data.height, None);
            }
            _ => panic!("Expected parsed item to be Text!"),
        }
    }

    #[test]
    fn it_parses_fit_width_correctly() {
        let mut text_raw = klwp::item::tests::base_item(klwp::item::InternalType::TextModule);
        text_raw.text_size_type = Some(klwp::item::TextSizeType::FitWidth);
        text_raw.text_expression = Some("TEST CONTENT".to_owned());
        text_raw.text_family = Some("TEST FAMILY".to_owned());
        text_raw.text_size = Some(10.0);
        text_raw.text_width = Some(20.0);
        text_raw.text_height = Some(30.0);

        let item_parsed = from_raw_klwp(&text_raw, "0", true);
        match item_parsed {
            Item::Text(v) => {
                assert!(matches!(v.data.r#type, TextDataType::FitWidth));
                assert_eq!(v.data.content, "TEST CONTENT");
                assert_eq!(v.data.family.unwrap(), "TEST FAMILY");
                assert_eq!(v.data.size, None);
                assert_eq!(v.data.width, Some(20.0));
                assert_eq!(v.data.height, None);
            }
            _ => panic!("Expected parsed item to be Text!"),
        }
    }

    #[test]
    fn it_parses_fit_to_box_correctly() {
        let mut text_raw = klwp::item::tests::base_item(klwp::item::InternalType::TextModule);
        text_raw.text_size_type = Some(klwp::item::TextSizeType::FitToBox);
        text_raw.text_expression = Some("TEST CONTENT".to_owned());
        text_raw.text_family = Some("TEST FAMILY".to_owned());
        text_raw.text_size = Some(10.0);
        text_raw.text_width = Some(20.0);
        text_raw.text_height = Some(30.0);

        let item_parsed = from_raw_klwp(&text_raw, "0", true);
        match item_parsed {
            Item::Text(v) => {
                assert!(matches!(v.data.r#type, TextDataType::FitToBox));
                assert_eq!(v.data.content, "TEST CONTENT");
                assert_eq!(v.data.family.unwrap(), "TEST FAMILY");
                assert_eq!(v.data.size, None);
                assert_eq!(v.data.width, Some(20.0));
                assert_eq!(v.data.height, Some(30.0));
            }
            _ => panic!("Expected parsed item to be Text!"),
        }
    }
}
