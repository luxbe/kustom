pub mod klwp;
mod utils;

use std::io::{Cursor, Read, Write};
use wasm_bindgen::prelude::*;
use zip::{ZipArchive, ZipWriter};

#[wasm_bindgen(start)]
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

    let mut preset_str = String::new();
    preset_file.read_to_string(&mut preset_str).unwrap();
    
    let preset_raw = serde_json::from_str::<klwp::Preset>(&preset_str).unwrap();

    serde_wasm_bindgen::to_value(&preset_raw).unwrap()
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"/**
 * @param {Preset} preset - The KLWP Preset to export
 * @returns {Uint8Array}
 */
export function exportKLWPFile(preset: Preset): {Uint8Array};"#;
#[wasm_bindgen(js_name = exportKLWPFile, skip_typescript)]
pub fn export_klwp_file(preset_raw: JsValue) -> Vec<u8> {
    let preset = preset_raw.into_serde::<klwp::Preset>().unwrap();
    let preset_str = serde_json::to_string(&preset).unwrap();

    let buf: Vec<u8> = vec![];
    let writer = Cursor::new(buf);
    let mut zip = ZipWriter::new(writer);

    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored);
    zip.start_file("preset.json", options).unwrap();
    zip.write(&preset_str.as_bytes()).unwrap();
    zip.finish().unwrap().into_inner()
}
