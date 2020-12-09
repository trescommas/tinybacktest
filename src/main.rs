use serde::Deserialize;

#[derive(Deserialize)]
struct Record{
	time: u16,
	prediction: i8,
	price: u8
}

struct DataFrame{
	time: Vec<u16>,
	prediction: Vec<i8>,
	price: Vec<u16>
}

impl DataFrame{
	
	fn new() -> DataFrame{
		DataFrame{
			time: Vec::new(),
			prediction: Vec::new(),
			price: Vec::new(),
		}
	}

	fn push(&mut self, row: &csv::StringRecord){
		self.time.push(row[0].to_string());
		self.prediction.push(row[1].to_string());
		self.price.push(row[2].to_string());
	}

	fn load_csv(path: &str) -> DataFrame {
		
		let file = std::fs::File::open(path).unwrap();
		let mut file_reader = csv::ReaderBuilder::new()
			.from_reader(file);	
		
		let mut dataframe = Dataframe::new();

		for row in file_reader.records().into_iter() {
			data_frame.push(&row);
		}
		
		return dataframe;
	}
	
}


fn main() {
	let df: DataFrame = DataFrame::load_csv(".trainTHETA-PERP.csv"); 
}
