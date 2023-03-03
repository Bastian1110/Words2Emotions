#![allow(unused_variables)]
pub mod model;
mod utils;

use ciborium;
use ciborium::value::Value;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

//Strcut to implement a JS object that glues WASM to JS
#[wasm_bindgen]
pub struct EmotionRecognition {
    model: model::EmotionDetection,
}
#[wasm_bindgen]
impl EmotionRecognition {
    pub fn new() -> EmotionRecognition {
        let bytes = include_bytes!("pipeline.cbor");
        let model_data = bytes.to_vec();
        let model_value = ciborium::de::from_reader::<Value, _>(&model_data[..]).unwrap();
        let model: model::EmotionDetection = model_value.deserialized().unwrap();
        EmotionRecognition { model: model }
    }

    pub fn predict(&self, input: String) -> String {
        let prediction = self.model.predict_string(input);
        return prediction;
    }
}
