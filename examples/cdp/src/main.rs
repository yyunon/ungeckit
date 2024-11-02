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
    let mut ws = cdp::CDP::new(context, &ws_url);
    //let cmd = format!(r#"{{"id": {}, "cmd": "protocol"}}"#, session_id);
    //let url = r#"/json/protocol"#;
    //let cmd = r#"{"method": "/json/protocol", "params": "", "waitingForDebugger":""}"#;
    let mut resp = ws.send("session.subscribe", r#"{"events":["browsingContext.domContentLoaded"]}"#).unwrap();
    println!("{:?}", resp);
    resp = ws.send("browser.createUserContext", "{}").unwrap();
    println!("{:?}", resp);
}
