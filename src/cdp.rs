/*
Holds the logic to communicate with dev tools via CDP
 */
use crate::utils::error::{ErrorKind, GeckError};
use crate::utils::webdriver_commands::WebdriverCmd;
use crate::utils::net::ws::WebSocketClient;
use crate::service::Context;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::Deserialize;
use serde::de::Visitor;
use serde_json::{Value};
use tokio_tungstenite::tungstenite::protocol::Message;
use tracing_subscriber::fmt::format;


#[derive(Debug)]
pub struct CDPResponseSuccess {
    pub m_type: String,
    pub id: i32,
    pub result: Value,
}
#[derive(Debug)]
pub struct CDPResponseFailure {
    pub m_type: String,
    pub id: i32,
    pub error: Option<String>,
    pub stacktrace: Option<String>,
    pub message: Option<String>,
}
#[derive(Debug)]
pub enum CDPMessage {
    CDPResponseSuccess(CDPResponseSuccess),
    CDPResponseFailure(CDPResponseFailure),
}
impl<'de> Deserialize<'de> for CDPMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
                struct CDPMessageVisitor;
                impl<'de> Visitor<'de> for CDPMessageVisitor {
                    type Value = CDPMessage;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        write!(formatter, "a string, but got something else ungeck")
                    }
                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                        where
                            A: serde::de::MapAccess<'de> {
                        let mut m_type = String::new();
                        let mut id = -1;
                        let mut result = String::new();
                        let mut error = String::new();
                        let mut stacktrace = String::new();
                        let mut message = String::new();
                        while let Some(key) = map.next_key::<&str>()? {
                            match key {
                                "type" => m_type = map.next_value()?,
                                "id" => id = map.next_value::<i32>()?,
                                "result" => result = map.next_value()?,
                                "error" => error = map.next_value()?,
                                "stacktrace" => stacktrace = map.next_value()?,
                                "message" => message = map.next_value()?,
                                &_ => todo!()
                            }
                        }
                        if m_type == "success" {
                            Ok(CDPMessage::CDPResponseSuccess(
                                CDPResponseSuccess{
                                    m_type: m_type,
                                    id: id,
                                    result: Value::Null // Change later
                                }
                            ))
                        } else {
                            Ok(CDPMessage::CDPResponseFailure(
                                CDPResponseFailure {
                                    m_type,
                                    id,
                                    error: Some(error),
                                    stacktrace: Some(stacktrace),
                                    message: Some(message)
                                }
                            ))

                        }
                    }
                }
                deserializer.deserialize_map(CDPMessageVisitor)
    }

}
pub struct CDP {
    /// Command dictionary with specific webdriver commands for http endpoints
    pub command_dict: HashMap<&'static str, WebdriverCmd<'static>>,
    /// WebSocket Client
    pub ws_client: WebSocketClient,
    /// Id of the sent message, to be autoincremented on each sent message
    pub id: i32,
}

impl CDP {
    /// Create CDP wrapper
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

    /// Send CDP message on websocket
    /// Works with serde_json::Value type as a result.
    pub fn send(&mut self, command: &str, params: &str) -> Result<Value, GeckError> {
        let message = format!(r#"{{"id": {}, "method":"{}", "params":{} }}"#, self.id, command, params);
        let data = self.ws_client.send(&message)?.into_text()?;
        let parsed_msg: Value = serde_json::from_str(&data).unwrap();
        match parsed_msg["type"].as_str().unwrap() {
            "error" => {
                Err(GeckError::new(ErrorKind::Driver, Some("None"), &format!("Failed to parse successful message for {:?} \nError: {:?} \nStacktrace {:?}", data, parsed_msg["error"], parsed_msg["stacktrace"])))
            },
            &_ => {
                self.id +=1;
                Ok(parsed_msg)
            }
        }
    }
}
