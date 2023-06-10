use super::orderbook::{Orderbook, self, Order};
use std::{collections::HashMap};

// Example -> BTC/USD
// BTC -> Base
// USD -> Quote

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TradingPair {
    base: String,
    quote: String
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair{base, quote}
    }
}

#[derive(Debug)]
pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, Orderbook>,

}
impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine{orderbooks: HashMap::new()}
    }

    pub fn add_new_market(&mut self, pair: TradingPair) {
        self.orderbooks.insert(pair.clone(), Orderbook::new());
        println!("Opening new orderbook for market {:?}", pair);
    }

    pub fn place_limit_order(&mut self, pair: TradingPair, price: f64, order: Order) -> Result<(), String> {
        match self.orderbooks.get_mut(&pair) {
            Some(orderbook) => {
                orderbook.add_order(price, order);

                println!("Placing order for price level {:?}\n", price);
                Ok(())
            },
            None => {
                Err(format!("The orderbook for the given trading pair {}-{} does not exist\n", pair.base, pair.quote))
            }
        }
    }
}
