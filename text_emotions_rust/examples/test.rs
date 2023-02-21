use dict::{Dict, DictIface};
use linfa::composing::MultiClassModel;
use linfa::prelude::*;
use linfa_preprocessing::CountVectorizer;
use linfa_svm::{error::Result, Svm};
use ndarray::{prelude::*, OwnedRepr};

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

fn main() -> Result<()> {
    //Read the data into a datafram
    let data_from_csv = DataFrame::read_csv("./data/test.csv");

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

    let params = Svm::<_, Pr>::params()
        .pos_neg_weights(5000., 500.)
        .gaussian_kernel(80.0);

    let model: MultiClassModel<ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>, usize> = train
        .one_vs_all()?
        .into_iter()
        .map(|(l, x)| (l, params.fit(&x).unwrap()))
        .collect::<MultiClassModel<_, _>>();

    let pred = model.predict(&valid);

    // create a confusion matrix
    let cm = pred.confusion_matrix(&valid)?;

    // Print the confusion matrix
    println!("{:?}", cm);

    // Calculate the accuracy and Matthew Correlation Coefficient (cross-correlation between
    // predicted and targets)
    println!("accuracy {}, MCC {}", cm.accuracy(), cm.mcc());

    Ok(())
}
