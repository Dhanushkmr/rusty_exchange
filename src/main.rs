mod matching_engine;
use matching_engine::orderbook::{Order, OrderType, Orderbook};
use matching_engine::engine::{TradingPair, MatchingEngine};

fn main() {
    println!("Hello, world!");
    
    // Buy orders init
    let buy_order_one = Order::new(OrderType::Bid, 55.5);
    let buy_order_two = Order::new(OrderType::Bid, 69.5);
    let buy_order_three = Order::new(OrderType::Bid, 62.5);

    // Sell orders init
    let sell_order_one = Order::new(OrderType::Ask, 45.5);
    let sell_order_two = Order::new(OrderType::Ask, 75.5);

    let mut orderbook = Orderbook::new();
    // orderbook.add_order(4.4, buy_order_one);
    orderbook.add_order(4.4, buy_order_two);

    orderbook.add_order(25.5, sell_order_one);
    orderbook.add_order(25.5, sell_order_two);

    print!("{:?}\n", orderbook);

    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    let eth_pair = TradingPair::new("ETH".to_string(), "USD".to_string());
    engine.add_new_market(pair.clone());

    engine.place_limit_order(pair, 20.99, buy_order_one).unwrap();
    engine.place_limit_order(eth_pair, 20.99, buy_order_three).unwrap()

}
