// Derived from https://tms-dev-blog.com/easily-connect-to-binance-websocket-streams-with-rust/

use signal_hook::{consts::SIGINT, consts::SIGTERM};
use simple_logger::SimpleLogger;
use std::{error::Error};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tungstenite::connect;
use url::Url;

mod models;

use tonic::{transport::Server, Request, Response, Status};
use orderbook::{Empty, Summary, Level, orderbook_aggregator_server::{OrderbookAggregator, OrderbookAggregatorServer}};

pub mod orderbook {
    tonic::include_proto!("orderbook");
}

#[derive(Debug, Default)]
pub struct OrderbookAggregatorService {}

#[tonic::async_trait]
impl OrderbookAggregator for OrderbookAggregatorService {
    async fn book_summary(&self, request: Request<Empty>) -> Result<Response<Summary>, Status> {
        // TODO: put the right values in...
        let bids_update=vec![
            Level {
                exchange: "TODO1".to_string(),
                price: 42.0,
                amount: 67.0
            }
        ];
        let asks_update=vec![
            Level {
                exchange: "TODO2".to_string(),
                price: 42.0,
                amount: 67.0
            }
        ];
        let response = Summary {
            spread: 68.0,
            bids: bids_update,
            asks: asks_update
        };
        Ok(Response::new(response))
    }
}

// TODO Need to handle all exchanges...
static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";
static BINANCE_INSTRUMENT: &str = "ethbtc";
static BINANCE_INSTRUMENT_DETAILS: &str = "depth10@100ms";

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init().unwrap();
    log::info!("Market-data server version: {}", env!("GIT_HASH"));
    let exit_main = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(SIGTERM|SIGINT, Arc::clone(&exit_main))?;
    // TODO consider cmmand-line arguments for passing in the exchange URLs. Also some form of help.
    // TODO Need to handle all exchanges...
    let binance_url = format!("{}/ws/{}@{}", BINANCE_WS_API, BINANCE_INSTRUMENT, BINANCE_INSTRUMENT_DETAILS);
    let (mut socket, response) =
        connect(Url::parse(&binance_url).unwrap()).expect(&format!("Cannot connect to: '{}'.", binance_url));
    log::info!("Connected to binance stream: '{}'.", binance_url);
    log::info!("HTTP status code: {}", response.status());
    log::info!("Response headers:");
    for (ref header, header_value) in response.headers() {
        log::info!("- {}: {:?}", header, header_value);
    }
    log::info!("Commencing receiving market-data.");
    while !exit_main.load(Ordering::Relaxed) {
        let raw_msg = socket.read_message().expect(&format!("Error reading message from: '{}'.", binance_url));
        let msg = match raw_msg {
            tungstenite::Message::Text(s) => s,
            _ => {
                // TODO need to just log this as an error, possibly re-connect...
                log::warn!("Error getting text from: '{}'.", binance_url);
                // TODO need to try re-connecting?
                break;
            }
        };

//        let parsed_data: serde_json::Value = serde_json::from_str(&msg).expect(&format!("Unable to parse message from: '{}'.", binance_url));
//        log::info!("{:?}", parsed_data);
        let parsed: models::DepthStreamData = serde_json::from_str(&msg).expect(&format!("Unable to parse message from: '{}'.", binance_url));
        for i in 0..parsed.asks.len() {
            log::info!(
                "{}. ask: {}, size: {}. bid: {}, size: {}",
                i, parsed.asks[i].price, parsed.asks[i].size, parsed.bids[i].price, parsed.bids[i].size
            );
        }
        // TODO Need to push this to gRPC...
    }
    log::info!("Finished receiving market-data, exiting.");
    Ok(())
}
