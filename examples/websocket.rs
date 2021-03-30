use crate::huobi_future::{models::Subscription, models::WebsocketEvent, HuobiWebsocket};
use failure::Fallible;
use huobi_future_async as huobi_future;
use std::collections::HashMap;
extern crate simple_logger;

#[tokio::main]
async fn main() -> Fallible<()> {
    // simple_logger::init().unwrap();
    let mut access_key = "";
    let mut secret_key = "";

    // 从命令行参数获取火币交易平台API访问秘钥
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 3 {
        access_key = &args[1];
        secret_key = &args[2];
    }

    let mut ws: HuobiWebsocket =
        HuobiWebsocket::new(access_key, secret_key, |event: WebsocketEvent| {
            match event {
                WebsocketEvent::OrderBook(orderbook) => println!("orderbook:{:?}", orderbook),
                WebsocketEvent::Kline(kline) => println!("kline:{:?}", kline),
                WebsocketEvent::IncrementalOrderBook(incremental_orderbook) => {
                    println!("incremental orderbook:{:?}", incremental_orderbook)
                }
                WebsocketEvent::BBO(bbo) => println!("bbo:{:?}", bbo),
                WebsocketEvent::TradeDetail(trade_detail) => {
                    println!("trade_detail:{:?}", trade_detail)
                }

                WebsocketEvent::Order(order) => println!("order:{:?}", order),
                WebsocketEvent::MatchOrder(order) => println!("Match order:{:?}", order),
                WebsocketEvent::Position(position) => println!("position:{:?}", position),
                WebsocketEvent::Account(account) => println!("account:{:?}", account),
                WebsocketEvent::ContractInfo(contract_info) => {
                    println!("contract_info:{:?}", contract_info)
                }
                WebsocketEvent::Liquidation(liquidation_orders) => {
                    println!("liquidation orders:{:?}", liquidation_orders)
                }
                WebsocketEvent::TriggerOrder(order) => println!("Trigger order:{:?}", order),

                WebsocketEvent::Index(index_price) => println!("index_price:{:?}", index_price),
                WebsocketEvent::Basis(basis) => println!("basis:{:?}", basis),

                _ => (),
            };

            Ok(())
        });

    let mut subs: HashMap<Subscription, Vec<&str>> = HashMap::new();

    let market_topics = vec![
        "market.BTC_NW.kline.1min",
        "market.BTC_NW.depth.step0",
        "market.btc_cw.depth.size_20.high_freq",
        "market.BTC_NW.bbo",
        "market.BTC_NW.trade.detail",
    ];

    subs.insert(Subscription::Market, market_topics);

    let account_topics = vec![
        "orders.btc",
        "matchOrders.btc",
        "accounts.btc",
        "positions.btc",
        "public.BTC.liquidation_orders",
        "public.btc.contract_info",
        "trigger_order.BTC",
    ];

    subs.insert(Subscription::Account, account_topics);

    let index_topics = vec!["market.BTC-USD.index.1min", "market.BTC_CW.basis.1min.open"];

    subs.insert(Subscription::Index, index_topics);

    if let Err(e) = ws.connect(subs).await {
        println!("### websocket error: {:?}", e);
    }

    Ok(())
}
