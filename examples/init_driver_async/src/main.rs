use std::vec::Vec;
use std::{thread, time};
use rust_geck::sync;

use simplelog::*;
use tokio::signal;

#[tokio::main]
async fn main() {
    TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();
    let mut driver = sync::Driver::new(None).await;
    signal::ctrl_c().await.expect("failed to listen sigint");
}
