use std::vec::Vec;
use std::{thread, time};
use rust_geck::{blocking};

use simplelog::*;

fn main() {
    TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();
    let mut driver = blocking::Driver::new(None);
    let ten_millis = time::Duration::from_millis(5000);
    let now = time::Instant::now();
    println!("Now running in main");
    thread::sleep(ten_millis);

}
