// use std::{
    // collections::HashMap,
    // sync::{Arc, Mutex},
// };

// use ccxt::exchange::Exchange;

// struct Token {
//     pub name: String,
//     pub exchanges: HashMap<String, i64>,
// }

// struct TokenFetched {
//     pub name: String,
//     pub exchanges: HashMap<String, i64>,
// }

// use tokio::task;

pub async fn arbitrage() {
    // let tokens: HashMap<String, HashMap<String, i64>> = HashMap::new();

    // for (token_key, token_value) in &tokens {
    //     task::spawn(async {
    //         for (exchange_key, exchange_value) in token_value {
    //             task::spawn(async { tokens[token_key].insert(exchange_key.clone(), 3); });
    //         }
    //     });
    // }
    //
    // let exchange = Exchange::new("defx").build()?;
    // let ticker = exchange.fetch_ticker("BTC/USDT")?;
    // println!("Price: {}", ticker.ask);
}
