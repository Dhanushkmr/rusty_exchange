use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub enum OrderType {
    Bid,
    Ask,
}
 
#[derive(Debug)]
pub struct Orderbook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

impl Orderbook {
    pub fn new() -> Orderbook {
        Orderbook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    pub fn add_order(&mut self, price: f64, order: Order) {
        let price = Price::new(price);
        match order.order_type  {
            OrderType::Bid => {
                match self.bids.get_mut(&price) {
                    Some(limit) => limit.add_order(order),
                    None  => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.bids.insert(price, limit);
                    }
                
                }   

            }
            OrderType::Ask => {
                match self.asks.get_mut(&price) {
                    Some(limit) => limit.add_order(order),
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.asks.insert(price, limit);
                    },
                }

            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

// Floats cannot be key in hashmap -> https://users.rust-lang.org/t/hashmap-key-cant-be-float-number-type-why/7892/2
impl Price {
    pub fn new(price: f64) -> Price {
        let scalar = 100000;
        let integral = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;
        Price {
            scalar: scalar,
            integral: integral,
            fractional: fractional,
        }
    }
}

#[derive(Debug)]
pub struct Limit {
    price: Price,
    orders: Vec<Order>,
}

impl Limit {
    pub fn new(price: Price) -> Limit {
        Limit {
            price: price,
            orders: Vec::new(),
        }
    }
    pub fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}
#[derive(Debug)]
pub struct Order {
    size: f64,
    order_type: OrderType,
}

impl Order {
    pub fn new(order_type: OrderType, size: f64) -> Order {
        Order {
            order_type: order_type,
            size: size,
        }
    }
}