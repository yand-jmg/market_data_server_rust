use simple_logger::SimpleLogger;
use tungstenite::connect;
use url::Url;

// TODO Need to handle all exchanges...
static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";
static BINANCE_INSTRUMENT: &str = "ethbtc";
static BINANCE_INSTRUMENT_DETAILS: &str = "depth5@100ms";

fn main() {
    SimpleLogger::new().init().unwrap();
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
    loop {
        let msg = socket.read_message().expect(&format!("Error reading message from: '{}'.", binance_url));
        let msg = match msg {
            tungstenite::Message::Text(s) => s,
            _ => {
                // TODO need to just log this as an error, possibly re-connect....
                panic!("Error getting text from: '{}'.", binance_url);
            }
        };

        let parsed_data: serde_json::Value = serde_json::from_str(&msg).expect(&format!("Unable to parse message from: '{}'.", binance_url));
        log::info!("{:?}", parsed_data);
        // TODO Need to push this to gRPC
    }
    // TODO need CTRL-C handler.
    log::info!("Finished receiving market-data, exiting.");
}
