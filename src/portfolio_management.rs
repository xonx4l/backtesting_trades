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

fn buy(&mut self, symbol: String, price: f64, quantity: u32, stop_loss: Option<f64>) {
        let total_cost = price * quantity as f64;
        if self.cash >= total_cost {
            self.cash -= total_cost;
            let entry = self.positions.entry(symbol.clone()).or_insert(0);
            *entry += quantity;
            
            if let Some(stop_loss_price) = stop_loss {
                self.stop_losses.insert(symbol.clone(), stop_loss_price);
            }

            self.trade_history.push(Trade {
                trade_type: "buy".to_string(),
                symbol,
                price,
                quantity,
                stop_loss,
            });
        } else {
            println!("Not enough cash to complete the purchase.");
        }
    }



fn sell(&mut self, symbol: &str, price: f64, quantity: u32) -> Result<(), &'static str> {
        let pos = self.positions.get_mut(symbol);
        if let Some(pos) = pos {
            if *pos < quantity {
                return Err("Not enough shares to sell");
            }
            *pos -= quantity;
            self.cash += price * quantity as f64;
            if *pos == 0 {
                self.positions.remove(symbol);
                self.stop_losses.remove(symbol); // Remove stop-loss if no shares left
            }
            self.trade_history.push(Trade {
                trade_type: "sell".to_string(),
                symbol: symbol.to_string(),
                price,
                quantity,
                stop_loss: None,
            });
            Ok(())
        } else {
            Err("Symbol not found in portfolio")
        }
    }



