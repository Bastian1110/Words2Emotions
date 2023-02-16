use csv;
use dict::{Dict, DictIface};
use linfa_preprocessing::CountVectorizer;
use ndarray::prelude::*;

struct DataFrame {
    index: Vec<u128>,
    emotion: Vec<usize>,
    text: Vec<String>,
    clean_text: Vec<String>,
    emotions_map: Dict<String>,
    n_emotions: usize,
}
impl DataFrame {
    fn new() -> DataFrame {
        DataFrame {
            index: Vec::new(),
            emotion: Vec::new(),
            text: Vec::new(),
            clean_text: Vec::new(),
            emotions_map: Dict::<String>::new(),
            n_emotions: 0,
        }
    }
    fn read_csv(filepath: &str) -> DataFrame {
        // Open file
        let file = std::fs::File::open(filepath).unwrap();
        let mut rdr = csv::ReaderBuilder::new().from_reader(file);
        let mut data_frame = DataFrame::new();

        // push all the records
        for result in rdr.records().into_iter() {
            let record = result.unwrap();
            data_frame.push(&record);
        }
        return data_frame;
    }

    fn push(&mut self, row: &csv::StringRecord) {
        self.index.push(row[0].parse().unwrap());
        if !(self.emotions_map.contains_key(&row[1])) {
            self.emotions_map
                .add(row[1].to_string(), self.n_emotions.to_string());
            self.n_emotions = self.n_emotions + 1;
        }
        let emotion_number = self.emotions_map.get(&row[1]).unwrap();
        self.emotion.push(emotion_number.parse().unwrap());
        self.text.push(row[2].to_string());
        self.clean_text.push(row[3].to_string());
    }
}

fn main() {
    let data = DataFrame::read_csv("./dataset/test.csv");
    let vec_training_sentences: Vec<&str> = data.clean_text.iter().map(|x| &**x).collect();
    let arr_training_sentences = Array::from_vec(vec_training_sentences);
    println!(
        "Targets : {:?} \n Docs : {:?}",
        data.emotion, data.clean_text
    )
}
