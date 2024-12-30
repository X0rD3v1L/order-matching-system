use serde::Deserialize;
use reqwest;

#[derive(Debug, Deserialize)]
pub struct Price {
    pub symbol: String,
    pub price: String,
}

pub fn get_prices() -> Vec<Price> {
    let url = "https://api4.binance.com/api/v3/ticker/price";
    let response = reqwest::blocking::get(url)
        .expect("Request failed")
        .text()
        .expect("Failed to read response body");

    serde_json::from_str(&response).expect("Failed to parse JSON")
}

pub fn order_matching(ask: &[u32], bid: &[u32]) -> Vec<(u32,u32)>{

    let matched_orders: Vec<(u32, u32)> = bid
        .iter()
        .zip(ask.iter())
        .filter(|(&b, &a)| b >= a)
        .map(|(&b, &a)| (a, b))
        .collect();

    matched_orders
}

pub fn order_execution(matched_orders: Vec<(u32, u32)>) {
    let mut avg_execution_price_sum: f32 = 0.0;

    for (ask_price, bid_price) in matched_orders.iter() {
        let mid_point = (*ask_price + *bid_price) as f32 / 2.0;
        avg_execution_price_sum += mid_point;
        println!("Ask: {}, Bid: {}, Mid-point: {:.2}", ask_price, bid_price, mid_point);
    }

    // Avoiding division by zero
    let avg_execution_price = if !matched_orders.is_empty() {
        avg_execution_price_sum / matched_orders.len() as f32
    } else {
        0.0
    };

    println!("Executed orders: {}", matched_orders.len());
    println!("Average execution price: {:.2}", avg_execution_price);
}

pub fn main() {
    let prices = get_prices();
    let mut mark_price = 0.0;
    for price in prices {
        if price.symbol == "BTCUSDT" {
            mark_price =  price.price.parse::<f32>().expect("Invalid float format");
        }
    }

    println!("Mark Price:: {}", mark_price);
    let ask = [100, 102, 103];
    let bid = [101, 99, 105];
    let matched_orders = order_matching(&ask, &bid);

    order_execution(matched_orders);
    
}

/*
Mark Price:: 93864
Ask: 100, Bid: 101, Mid-point: 100.50
Ask: 103, Bid: 105, Mid-point: 104.00
Executed orders: 2
Average execution price: 102.25
*/



// use std::cmp::max;

// fn maximize_profit(prices: &[u32]) -> u32{

//     let mut min_price = &prices[0]; //7
//     let mut profit = 0;
//     for price in prices {
//         if price < min_price {
//             min_price = price;
//         }
//         profit = max(profit, price - min_price);
//     }

//     return profit
// }

// fn main() {
//     let prices: Vec<u32> = vec![7, 6, 4, 3, 1];
//     println!("{}", maximize_profit(&prices));
// }

// /*
// 7 1 5 3 6 4

// 7
// 1
// */
