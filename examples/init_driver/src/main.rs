use std::{thread, time};
use rust_geck::*;

use simplelog::*;

fn main() {
    TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();
    let mut driver = Driver::new(None).unwrap();
    let page = driver.get("https://yukselyonsel.com");
    let ten_millis = time::Duration::from_millis(5000);
    let now = time::Instant::now();
    println!("Now running in main");
    thread::sleep(ten_millis);

}
