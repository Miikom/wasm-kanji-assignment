use std::fs::File;
use std::io::Read;
use wasm_bindgen_test::*;
use wasm_assignment::search_by_strokes;

#[wasm_bindgen_test]
fn test_search_kanji_by_strokes() {
    let kanji_object = json::parse(
        r#"{
                    "一": {
                        "strokes": 1,
                        "grade": 1,
                        "freq": 2,
                        "jlpt_old": 4,
                        "jlpt_new": 5,
                        "meanings": ["One","One Radical (no.1)"],
                        "readings_on": ["いち","いつ"],
                        "readings_kun": ["ひと-","ひと.つ"],
                        "wk_level": 1,
                        "wk_meanings": ["One"],
                        "wk_readings_on": ["いち","いつ"],
                        "wk_readings_kun": ["!ひと"],
                        "wk_radicals": ["Ground"]
                    }
                }"#).unwrap();
    let expected_result = kanji_object.dump();
    let result = search_by_strokes(&kanji_object.dump(), 1);
    assert_eq!(result, expected_result);
}