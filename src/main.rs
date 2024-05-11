use std:;collections::Hashmap;

struct MarketData {
    prices: Vec<f64>,
    timestamps: Vec<u64>
}

struct TradingStrategy {
    short_window: usize,
    long_window: usize,

    rsi_window:usize,  //Relative Strength Index (RSI) window
    rsi_overbought: f64,
    rsi_oversold: f64,

    risk_per_trade:f64,
    leverage: f64,

    commission: f64,
    slippage: f64,
}

struct TradingPosition {
      entry_price: f64,
      exit_price: f64,
      transaction_fee: f64,
      profit: f64,
      loss:f64,
      quantity: u32,
}
