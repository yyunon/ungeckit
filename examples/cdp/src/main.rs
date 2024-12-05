use std::{thread, time};
use serde_json;
use serde_json::{Value};
use ungeckit::*;
use ungeckit::utils::net::ws;
use simplelog::*;
use fake_user_agent::get_rua;

fn main() {
    TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();

    let context = service::Context::new();
    let session_id = "c8165f2e-f79f-4af6-b0f4-03227570212e";
    let ws_url = format!("ws://127.0.0.1:9222/session/{}", session_id);
    //let ws_url = "ws://127.0.0.1:9222";
    let mut ws = cdp::CDP::new(context, &ws_url);
    //let cmd = format!(r#"{{"id": {}, "cmd": "protocol"}}"#, session_id);
    //let url = r#"/json/protocol"#;
    //let cmd = r#"{"method": "/json/protocol", "params": "", "waitingForDebugger":""}"#;
    let mut resp = ws.send("session.subscribe", r#"{"events":["browsingContext.domContentLoaded"]}"#).unwrap();
    resp = ws.send("browser.createUserContext", "{}").unwrap();
    println!("{:?}", resp);
    let mut userContext = &resp["result"]["userContext"].as_str().unwrap();

    let mut scriptParams = format!(r#"{{ "type":"tab", "userContext": "{}" }}"#, userContext);
    let mut browsingContext = ws.send("browsingContext.create", &scriptParams).unwrap();
    let rua = get_rua();

    //resp = ws.send("Network.setUserAgentOverride", &format!(r#"{{"userAgent": "{}"}}"#, rua)).unwrap();
    scriptParams = format!(r#"{{"expression": "return navigator.webdriver", "awaitPromise":true, "userContext": "{}" }}"#, userContext);
    println!("{}", scriptParams);
    resp = ws.send("script.evaluate", &scriptParams).unwrap();
    println!("{:?}", resp);
}
