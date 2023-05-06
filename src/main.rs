enum OrderType {
    Bid,
    Ask

}
struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64
}

impl Price {
    fn new(price: f64) -> Price {
        let scalar = 100000;
        let integral = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;
        Price  {scalar, integral, fractional}
    }
}
struct Limit {
    price: Price,
    orders: Vec<Order>
}
struct Order {
    size: f64,
    order_type: OrderType
}

impl Order {
    fn new(order_type: OrderType, size: f64) -> Order {
        Order {order_type, size}
    }
}
fn main() {
    println!("Hello, world!"); 
}
