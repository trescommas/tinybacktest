use serde::Deserialize;

pub struct DataFrame{
    pub time: Vec<String>,
    pub prediction: Vec<i8>,
    pub price: Vec<f64>
}

impl DataFrame{
	
    pub fn new() -> DataFrame{
        DataFrame{
            time: Vec::new(),
            prediction: Vec::new(),
            price: Vec::new(),
        }
    }

    pub fn push(&mut self, row: &csv::StringRecord){
        self.time.push(row[0].to_string());
        self.prediction.push(row[1].to_string().parse::<i8>().unwrap());
        self.price.push(row[2].to_string().parse::<f64>().unwrap());
    }

    pub fn load_csv(path: &str) -> DataFrame {
        
        let file = std::fs::File::open(path).unwrap();
        let mut file_reader = csv::ReaderBuilder::new()
            .from_reader(file);	
        
        let mut dataframe = DataFrame::new();

        for row in file_reader.records().into_iter() {
            let record= row.unwrap();
            dataframe.push(&record);
        }
        
        return dataframe;
    }
	
}
