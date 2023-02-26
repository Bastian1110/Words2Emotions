pub mod model;
pub mod test;
use ciborium;
use ciborium::value::Value;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("./pipeline.cbor").unwrap();
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data).unwrap();

    let model_value = ciborium::de::from_reader::<Value, _>(&data[..]).unwrap();

    let model: model::EmotionDetection = model_value.deserialized().unwrap();
    model.predict_string("Today im going to the movie cinema with my girlfriend!".to_string())
}
