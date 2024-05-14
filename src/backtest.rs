use std::fs::file;
use std::io::{BufRead, BufReader};

pub fn load_market_data (file_path: &String) -> MarketData{
      let file  = File::open*(file_path).expect("Failed to open file");
      let reader = BufReader::new(file);

      let mut prices = Vec::new();
      let mut timestamps = Vec::new();

}

impl BacktestEngine {
    fn calculate_metrics(&self) -> HashMap<String, f64> {
        let mut metrics = HashMap::new();

        // Calculates total profit/loss
        let total_profit_loss: f64 = self
            .positions
            .iter()
            .map(|p| (p.exit_price - p.entry_price) as f64 * p.quantity as f64)
            .sum();
        metrics.insert("Total Profit/Loss".to_string(), total_profit_loss);

        // Calculates win rate
        let total_trades = self.positions.len();
        let winning_trades: usize = self
            .positions
            .iter()
            .filter(|p| p.exit_price > p.entry_price)
            .count();
        let win_rate = if total_trades > 0 {
            (winning_trades as f64 / total_trades as f64) * 100.0
        } else {
            0.0
        };
        metrics.insert("Win Rate (%)".to_string(), win_rate);


        metrics
    }

    fn historical_returns(&self) -> Vec<f64> {
        let mut returns = Vec::new();
            returns = self.positions.iter().map(|p| p.exit_price - p.entry_price).collect();
            historical_returns.push(returns);
    }

    
}
