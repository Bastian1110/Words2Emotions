use ciborium::cbor;
use dict::{Dict, DictIface};
use linfa::prelude::*;
use linfa_logistic::{MultiFittedLogisticRegression, MultiLogisticRegression};
use linfa_preprocessing::CountVectorizer;
use ndarray::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;

//Author : Sebastian Mora
//Main model for reconizing emptions with a very primitive NLP algorithm
//using a logistic regression and a count vextorizer

//Serializable "pipeline" for texporting the complete model
#[derive(Serialize, Deserialize, Debug)]
pub struct EmotionDetection {
    vectorizer: CountVectorizer,
    regression: MultiFittedLogisticRegression<f64, usize>,
    labels: Vec<String>,
}
impl EmotionDetection {
    pub fn new(
        new_vectorizer: CountVectorizer,
        new_regression: MultiFittedLogisticRegression<f64, usize>,
        new_labels: Vec<String>,
    ) -> EmotionDetection {
        EmotionDetection {
            vectorizer: new_vectorizer,
            regression: new_regression,
            labels: new_labels,
        }
    }

    pub fn predict_string(&self, input: String) -> String {
        let input_str = array![input.as_str()];
        let input_vector = self.vectorizer.transform(&input_str).to_dense();
        let input_vector = input_vector.mapv(|c| c as f64);

        let input_predition = self.regression.predict(&input_vector);
        let emotion_number = input_predition.get(0).unwrap().to_owned();
        let emotion = self.labels.get(emotion_number).unwrap().to_owned();
        return emotion;
    }
}

//Dataframe for reading the emotion dataset
struct DataFrame {
    emotions: Vec<usize>,
    text: Vec<String>,
    emotions_map: Dict<String>,
    n_emotions: usize,
    labels: Vec<String>,
}
impl DataFrame {
    fn new() -> DataFrame {
        DataFrame {
            emotions: Vec::new(),
            text: Vec::new(),
            emotions_map: Dict::<String>::new(),
            n_emotions: 0,
            labels: Vec::new(),
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
            self.labels.push(row[0].to_string());
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
    println!("test");
    println!("{:?}", data_from_csv.labels);

    let pipeline = EmotionDetection::new(vectorizer, model, data_from_csv.labels);

    //Exporting the model
    let pipeline_value = cbor!(pipeline).unwrap();
    let mut vec_pipeline = Vec::new();
    let _cebor_writer = ciborium::ser::into_writer(&pipeline_value, &mut vec_pipeline);

    //Esporting it to a .cbor file
    let path: &Path = Path::new("./pipeline.cbor");
    fs::write(path, vec_pipeline).unwrap();

    Ok(())
}
