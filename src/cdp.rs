/*
Holds the logic to communicate with dev tools via CDP
 */
use crate::utils::error::GeckError;
use crate::utils::webdriver_commands::WebdriverCmd;
use crate::utils::net::ws::WebSocketClient;
use crate::service::Context;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::Deserialize;
use tokio_tungstenite::tungstenite::protocol::Message;
use tracing_subscriber::fmt::format;


#[derive(Debug, Deserialize)]
pub struct CDPResponse {
    #[serde(alias = "result")]
    pub result: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct CDPMessage {
    #[serde(alias = "type")]
    pub m_type: String,
    #[serde(alias = "id")]
    pub id: i32,
    #[serde(alias = "result")]
    pub result: CDPResponse,
}
pub struct CDP {
    pub command_dict: HashMap<&'static str, WebdriverCmd<'static>>,
    pub ws_client: WebSocketClient,
    pub id: i32,
}

impl CDP {
    pub fn new(context_t: Arc<Mutex<Context>>, ws_url: &str) -> Self {
        let mut ws_client = WebSocketClient::new(context_t, ws_url);
        ws_client.connect();
        Self {
            command_dict: HashMap::from([
							("json", WebdriverCmd::from(("POST", "/json"))),
							("protocol", WebdriverCmd::from(("POST", "/json/protocol"))),
							("list", WebdriverCmd::from(("POST", "/json/list"))),
							("new", WebdriverCmd::from(("POST", "/json/new?{{url}}"))),
							("activate", WebdriverCmd::from(("POST", "/json/activate/{{id}}"))),
							("close", WebdriverCmd::from(("POST", "/json/close/{{id}}"))),
            ]),
            ws_client,
            id: 0,
        }
    }
    pub fn send(&mut self, command: &str, params: &str) -> Result<CDPMessage, GeckError> {
        let message = format!(r#"{{"id": {}, "method":"{}", "params":{} }}"#, self.id, command, params);
        let data = self.ws_client.send(&message)?.into_text()?;
        let parsed_msg: CDPMessage = serde_json::from_str(&data).unwrap();
        self.id +=1;
        Ok(parsed_msg)
    }
}