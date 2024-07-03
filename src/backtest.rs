use std::fs::file;
use std::io::{BufRead, BufReader};
use plotters::prelude::*;

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
     fn run(&mut self) {
        // Run backtest
        for i in 0..self.data.len() {
            let position = self.positions.get(i);
            let data = self.data.get(i);
        }
    } 
    pub fn backtesting_assets() {
    }
    impl BacktestEngine2 {
    fn plot_equity_curve(&self, file_path: &str) {
        let mut equity = self.cash;
        let mut equity_curve: Vec<(u64, f64)> = Vec::new();

        for (i, position) in self.positions.iter().enumerate() {
            equity -= position.entry_price as f64 * position.quantity as f64;
            equity_curve.push((self.market_data.timestamps[i], equity));

            equity += position.exit_price as f64 * position.quantity as f64;
            equity_curve.push((self.market_data.timestamps[i + 1], equity));
        }

        let root = BitMapBackend::new(file_path, (1024, 768)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption("Equity Curve", ("sans-serif", 50).into_font())
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 60)
            .build_cartesian_2d(
                self.market_data.timestamps[0]..self.market_data.timestamps.last().unwrap() + 1,
                equity_curve.iter().map(|(_, y)| *y).min()..equity_curve.iter().map(|(_, y)| *y).max(),
            )
            .unwrap();

        chart
            .configure_mesh()
            .x_desc("Timestamp")
            .y_desc("Equity")
            .draw()
            .unwrap();

        chart
            .draw_series(
                equity_curve
                    .iter()
                    .map(|(x, y)| Circle::new((*x, *y), 2, &BLACK)),
            )
            .unwrap()
            .label("Equity Curve")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        root.present().unwrap();
    }
}


    
}
