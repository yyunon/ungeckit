use std::{thread, time};
use serde_json;
use rust_geck::*;
use rust_geck::utils::net::ws;
use simplelog::*;

fn main() {
    TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();

    let context = service::Context::new();
    let session_id = "99e8b287-47e6-4813-9f0b-25eb63d9edbc";
    let ws_url = format!("ws://127.0.0.1:9222/session/{}", session_id);
    //let ws_url = "ws://127.0.0.1:9222";
    let mut ws = ws::WebSocketClient::new(context, session_id, &ws_url);
    ws.connect();
    //let cmd = format!(r#"{{"id": {}, "cmd": "protocol"}}"#, session_id);
    //let url = r#"/json/protocol"#;
    //let cmd = r#"{"method": "/json/protocol", "params": "", "waitingForDebugger":""}"#;
    let mut id = 1;
    let mut cmdList = vec![];
    let mut cmd = r#"{"id": 1, "method": "session.subscribe", "params": {"events":["browsingContext.domContentLoaded"]}}"#;
    cmdList.push(String::from(cmd));
    cmd = r#"{"id": 2, "method": "browser.createUserContext", "params": {}}"#;
    cmdList.push(String::from(cmd));
    ws.send_all(cmdList);
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
