use csv;
use dict::{Dict, DictIface};
use linfa::{metrics::ToConfusionMatrix, traits::Fit, traits::Predict, DatasetBase};
use linfa_bayes::GaussianNb;
//use linfa_logistic::LogisticRegression;
use linfa_preprocessing::CountVectorizer;
use ndarray::prelude::*;
use std::prelude::*;

//Dataframe for reading the emotion dataset
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
    //Importing the dataset from csv
    let data = DataFrame::read_csv("./dataset/emotions_clean.csv");

    //Vectorizing the text from dataset
    let vec_sentences: Vec<&str> = data.text.iter().map(|x| &**x).collect();
    let arr_sentences = Array::from_vec(vec_sentences);
    println!(
        "Targets : {:?}  Docs : {:?}",
        data.emotion.len(),
        data.text.len()
    );
    let vectorizer = CountVectorizer::params().fit(&arr_sentences).unwrap();
    println!(
        "We obtain a vocabulary with {} entries",
        vectorizer.nentries()
    );

    println!("Emotion Table");
    for o in data.emotions_map {
        println!("{} - {}", o.key, o.val);
    }

    //Making the training datset with records and targets
    let records = vectorizer.transform(&arr_sentences).to_dense();
    let records = records.mapv(|c| c as f32);

    let targets = Array1::from_shape_vec(data.emotion.len(), data.emotion).unwrap();

    let (train, valid) = DatasetBase::new(records, targets)
        .split_with_ratio(0.9)
        .into();

    //Making the model
    let model = GaussianNb::params().fit(&train).unwrap();
    let training_prediction = model.predict(&train);

    //Acurracy
    let cm = training_prediction.confusion_matrix(&train).unwrap();
    let accuracy = cm.f1_score();
    println!(
        "The fitted model has a training Training score of {}",
        accuracy
    );

    let valid_prediction = model.predict(&valid);

    let cm_valid = valid_prediction.confusion_matrix(&valid).unwrap();
    let valid_accuracy = cm_valid.f1_score();
    println!(
        "The fitted model has a training Validation score of {}",
        valid_accuracy
    );

    while true {
        println!("Enter an input");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");
        let input_string = array![input.as_str()];
        let input_array = vectorizer.transform(&input_string).to_dense();
        let input_array = input_array.mapv(|c| c as f32);

        println!("Test : {}", input_string);
        let input_prediction = model.predict(&input_array);
        println!("{:?} Prediction of last entry", input_prediction.first());
    }
}
