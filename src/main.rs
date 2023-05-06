use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
enum OrderType {
    Bid,
    Ask,
}
#[derive(Debug)]
struct Orderbook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}
impl Orderbook {
    fn new() -> Orderbook {
        Orderbook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    fn add_order(&mut self, price: f64, order: Order) {
        match order.order_type  {
            OrderType::Bid => {
                let price = Price::new(price);
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

            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

impl Price {
    fn new(price: f64) -> Price {
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
struct Limit {
    price: Price,
    orders: Vec<Order>,
}

impl Limit {
    fn new(price: Price) -> Limit {
        Limit {
            price: price,
            orders: Vec::new(),
        }
    }
    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}
#[derive(Debug)]
struct Order {
    size: f64,
    order_type: OrderType,
}

impl Order {
    fn new(order_type: OrderType, size: f64) -> Order {
        Order {
            order_type: order_type,
            size: size,
        }
    }
}
fn main() {
    println!("Hello, world!");
    let buy_order = Order::new(OrderType::Bid, 55.5);
    let sell_order = Order::new(OrderType::Ask, 45.5);
}
