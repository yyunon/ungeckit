use std::{thread, time};
use serde_json;
use rust_geck::*;
use rust_geck::utils::net::ws;
use simplelog::*;

fn main() {
    TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();

    let context = service::Context::new();
    let mut options = options::Options::new();
    let page = driver.get("https://nowsecure.nl");
    //let page = driver.get("https://bot.sannysoft.com/");
    //let page = driver.get("https://intoli.com/blog/not-possible-to-block-chrome-headless/chrome-headless-test.html");
    driver.save_screenshot("test.png");
    //driver.execute_script();

    //println!("{}", screenshot.unwrap());

    //let cap = driver.options.to_capabilities();
    //println!("{:?}", serde_json::to_string(&desired_capabilities).unwrap());
    //let ten_millis = time::Duration::from_millis(5000);
    //let now = time::Instant::now();
    //println!("Now running in main");
    //thread::sleep(ten_millis);

}
