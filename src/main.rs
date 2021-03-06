mod dataframe;
mod trade;
mod strategy;
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    let df = dataframe::DataFrame::load_csv("trainTHETA-PERP.csv"); 
    let duration = start.elapsed();
    //backtest(&df);
    println!("Time taken to load csv file {:?}", duration);
}
