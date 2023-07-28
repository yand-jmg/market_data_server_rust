// Derived from https://blog.logrocket.com/rust-and-grpc-a-complete-guide/ & https://www.thorsten-hans.com/grpc-services-in-rust-with-tonic/

use simple_logger::SimpleLogger;
use std::{error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init().unwrap();
    log::info!("Client version: {}", env!("GIT_HASH"));
    log::info!("Client starting.");
    Ok(())
}
