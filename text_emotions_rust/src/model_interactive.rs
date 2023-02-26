use ciborium::cbor;
use dict::{Dict, DictIface};
use linfa::prelude::*;
use linfa_logistic::MultiLogisticRegression;
use linfa_preprocessing::CountVectorizer;
use ndarray::prelude::*;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;

//Dataframe for reading the emotion dataset
struct DataFrame {
    emotions: Vec<usize>,
    text: Vec<String>,
    emotions_map: Dict<String>,
    n_emotions: usize,
}
impl DataFrame {
    fn new() -> DataFrame {
        DataFrame {
            emotions: Vec::new(),
            text: Vec::new(),
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
        if !(self.emotions_map.contains_key(&row[0])) {
            self.emotions_map
                .add(row[0].to_string(), self.n_emotions.to_string());
            self.n_emotions = self.n_emotions + 1;
        }
        let emotion_number = self.emotions_map.get(&row[0]).unwrap();
        self.emotions.push(emotion_number.parse().unwrap());
        self.text.push(row[1].to_string());
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    //Read the data into a datafram
    let data_from_csv = DataFrame::read_csv("./dataset/gooddata.csv");

    let text_vec: Vec<&str> = data_from_csv.text.iter().map(|x| &**x).collect();
    let text_arr = Array::from_vec(text_vec);

    let vectorizer = CountVectorizer::params().fit(&text_arr).unwrap();

    println!("Vectorizer with : {} entries", vectorizer.nentries());
    let records = vectorizer.transform(&text_arr).to_dense();
    let records = records.mapv(|c| c as f64);

    let emotion_arr = Array::from_vec(data_from_csv.emotions);

    //We create a dataset with the vectorized text and the targets
    let emotion_dataset = Dataset::new(records, emotion_arr);

    let (train, valid) = emotion_dataset.split_with_ratio(0.9);

    //Multi fitted logistic regression
    let model = MultiLogisticRegression::default()
        .max_iterations(100)
        .fit(&train)
        .unwrap();

    //Testing the model
    let pred = model.predict(&valid);

    let cm = pred.confusion_matrix(&valid).unwrap();

    println!("accuracy {}, MCC {}", cm.accuracy(), cm.mcc());

    println!("Emotion Table");
    for o in data_from_csv.emotions_map {
        println!("{} - {}", o.key, o.val);
    }

    let mut keep_alive = true;
    while keep_alive {
        println!("New test?");
        let mut keep = String::new();
        io::stdin().read_line(&mut keep)?;
        if keep == "y".to_string() {
            keep_alive = false;
        }
        if keep_alive {
            println!("Enter test input : ");
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let test_string = array![input.as_str()];
            let test_vector = vectorizer.transform(&test_string).to_dense();
            let test_vector = test_vector.mapv(|c| c as f64);
            let test_prediction = model.predict(&test_vector);
            println!("Prediction : {:?}", test_prediction);
        }
    }

    Ok(())
}
