use serde::Deserialize;
use std::time::{Duration, Instant};

struct DataFrame{
	time: Vec<String>,
	prediction: Vec<i8>,
	price: Vec<f64>
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
		self.prediction.push(row[1].to_string().parse::<i8>().unwrap());
		self.price.push(row[2].to_string().parse::<f64>().unwrap());
	}

	fn load_csv(path: &str) -> DataFrame {
		
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

// Tracks the stats of each trade item
struct Trade{
    entry_price: f64,
    exit_price: f64,
    entry_time: Vec<String>,
    exit_time: Vec<String>,
}

trait Strategy {
    fn backtest(&self, df: &DataFrame); // change it to a result struct
    fn take_profit(&self);
    fn take_stop_loss(&self);
    fn check_entry(&self);
    fn check_exit(&self);
}

struct BasicStrategy{
    trades: Vec<Trade>,
    net_pnl: f64,
    cost: f64,
    balance: f64,
    equity: f64,
}

impl BasicStrategy{
    
    fn new() -> BasicStrategy{
        BasicStrategy{
            trades: Vec::new(),
            net_pnl: 0.0,
            cost: 0.0,
            balance: 0.0,
            equity: 0.0
        }
    }
    
    /// The main backtest loop
    ///
    fn backtest(&mut self, df: &DataFrame) -> f64 {
        
        for (i, date) in df.time.iter().enumerate() {
            continue;
        }

        return 42.0;
    }

}




fn main() {
    let start = Instant::now();
	let df: DataFrame = DataFrame::load_csv("/Users/abi/tinybacktest/src/trainTHETA-PERP.csv"); 
    let duration = start.elapsed();
    //backtest(&df);
    println!("Time taken to load csv file {:?}", duration);
}
