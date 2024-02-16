// This is the main file for the Rust code. It will be compiled to WebAssembly.
use json;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn search_by_strokes(json_string: &str, strokes: u32) -> String {
    let kanji_obj = json::parse(json_string).unwrap();

    let mut result_kanji_obj = json::JsonValue::new_object();

    for (key, value) in kanji_obj.entries() {
        for (key2, value2) in value.entries() {
            if key2 == "strokes"{
                if value2.as_u32().unwrap() == strokes{
                        result_kanji_obj[key] = value.clone();
                }
            }
        }
    }
    json::stringify(result_kanji_obj)
}