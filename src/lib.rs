mod utils;

pub mod parsed;
pub mod raw;

use std::io::{Cursor, Read};
use wasm_bindgen::prelude::*;
use zip::ZipArchive;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start, skip_typescript)]
pub fn initialize() {
    utils::set_panic_hook();
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"/**
 * @param {Uint8Array} buf - A FileInput result as an ArrayBuffer
 * @returns {Preset}
 */
export function parseKLWPFile(buf: Uint8Array): Preset;"#;
#[wasm_bindgen(js_name = parseKLWPFile, skip_typescript)]
pub fn parse_klwp_file(buf: Vec<u8>) -> JsValue {
    let reader = Cursor::new(buf);
    let mut zip = ZipArchive::new(reader).unwrap();

    let mut preset_file = zip.by_name("preset.json").unwrap();

    let mut s = String::new();
    preset_file.read_to_string(&mut s).unwrap();

    let preset_raw = raw::klwp::from_json(&s);
    let preset_parsed = parsed::klwp::from_raw_klwp(preset_raw);

    JsValue::from_serde(&preset_parsed).unwrap()
}
