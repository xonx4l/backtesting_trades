use std::fs::file;
use std::io::{BufRead, BufReader};

pub fn load_market_data (file_path: &String) -> MarketData{
      let file  = File::open*(file_path).expect("Failed to open file");
      let reader = BufReader::new(file);

      let mut prices = Vec::new();
      let mut timestamps = Vec::new();

}