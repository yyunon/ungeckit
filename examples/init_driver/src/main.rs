use std::{thread, time};
use serde_json;
use rust_geck::*;
use rust_geck::utils::net::ws;
use simplelog::*;

fn main() {
    TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();

    let context = service::Context::new();
    let session_id = "ee185c4b-ca3d-49f3-b72c-a7a40e81eff1";
    let ws_url = format!("ws://127.0.0.1:9222/session/{}", session_id);
    let mut ws = ws::WebSocketClient::new(context, session_id, &ws_url);
    //let cmd = format!(r#"{{"id": {}, "cmd": "protocol"}}"#, session_id);
    let cmd = r#"{"cmd": "protocol"}"#;
    ws.send(&cmd);
    //let mut options = options::Options::new();
    //let page = driver.get("https://nowsecure.nl");
    //let page = driver.get("https://bot.sannysoft.com/");
    //let page = driver.get("https://intoli.com/blog/not-possible-to-block-chrome-headless/chrome-headless-test.html");
    //let screenshot = driver.save_screenshot("test.png");
    //driver.execute_script();

    //println!("{}", screenshot.unwrap());

    //let cap = driver.options.to_capabilities();
    //println!("{:?}", serde_json::to_string(&desired_capabilities).unwrap());
    //let ten_millis = time::Duration::from_millis(5000);
    //let now = time::Instant::now();
    //println!("Now running in main");
    //thread::sleep(ten_millis);

}
