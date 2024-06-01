use std::collections::HashMap;

#[derive(Debug)] 
struct Trade {
    trade_type:String,
    symbol:String,
    price:f64,
    quantity: u32,
    stop_loss: Option<f64>,
}

#[derive(Debug)] 
struct Portfolio {
    cash:f64,
    positions: HashMap<String,u32>,
    stop_losses:HashMap<String,f64>,
    trade_history: Vec<Trade>,
    portfolio_value_history: Vec<f64>,
}

impl Portfolio {
     fn new (initial_cash: f64) -> Portfolio {
           Portfolio {
               cash: initial_cash,
               positions: HashMap::new(),
               stop_losses: HashMap::new(),
               trade_history: Vec::new(),
               portfolio_value_history: Vec::new(),
           }
     }
}





