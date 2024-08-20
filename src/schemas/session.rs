use log::*;
use bytes::Bytes;
use serde::{self, de, Deserialize, Serialize};

use crate::utils::error::GeckError;

pub struct SchemaParser {}

pub trait TryParse<T> {
    fn try_parse_response(body_bytes: Bytes) -> Result<T, GeckError>
    where
        T: de::DeserializeOwned;
}

impl<T> TryParse<T> for SchemaParser {
    fn try_parse_response(body_bytes: Bytes) -> Result<T, GeckError>
    where
        T: de::DeserializeOwned,
    {
        let b = body_bytes.to_vec();
        let body_text = String::from_utf8_lossy(&b);
        let d: T = serde_json::from_str(&body_text).unwrap();
        Ok(d)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Proxy {
    // TODO implement
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Timeouts {
    pub implicit: i64,
    #[serde(alias = "pageLoad")]
    pub page_load: i64,
    pub script: i64,
}

pub fn none() -> String { return "".to_owned()}

#[derive(Serialize, Deserialize, Debug)]
pub struct Capabilities {
    #[serde(alias = "acceptInsecureCerts")]
    pub accept_insecure_certs: bool,
    #[serde(alias = "browserName")]
    pub browser_name: String,
    #[serde(alias = "browserVersion")]
    pub browser_version: String,
    #[serde(alias = "moz:accessibilityChecks")]
    pub moz_accessibility_checks: bool,
    #[serde(alias = "moz:buildID")]
    pub moz_build_id: String,
    #[serde(alias = "moz:debuggerAddress", default="none")]
    pub moz_debugger_address: String,
    #[serde(alias = "moz:geckodriverVersion")]
    pub moz_geckodriver_version: String,
    #[serde(alias = "moz:headless")]
    pub moz_headless: bool,
    #[serde(alias = "moz:platformVersion")]
    pub moz_platform_version: String,
    #[serde(alias = "moz:processID")]
    pub moz_process_id: i64,
    #[serde(alias = "moz:profile")]
    pub moz_profile: String,
    #[serde(alias = "moz:shutdownTimeout")]
    pub moz_shutdown_timeout: i64,
    #[serde(alias = "moz:webdriverClick")]
    pub moz_webdriver_click: bool,
    #[serde(alias = "moz:windowless")]
    pub moz_windowless: bool,
    #[serde(alias = "pageLoadStrategy")]
    pub page_load_strategy: String,
    #[serde(alias = "platformName")]
    pub platform_name: String,
    pub proxy: Proxy,
    #[serde(alias = "setWindowRect")]
    pub set_window_rect: bool,
    #[serde(alias = "strictFileInteractability")]
    pub strict_file_interactability: bool,
    pub timeouts: Timeouts,
    #[serde(alias = "unhandledPromptBehavior")]
    pub unhandled_prompt_behavior: String,
    #[serde(alias = "userAgent")]
    pub user_agent: String,
    #[serde(alias = "webSocketUrl")]
    pub web_socket_url: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    #[serde(alias = "sessionId")]
    pub session_id: String,
    pub capabilities: Capabilities,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SessionResponse {
    pub value: Session,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    pub value: Option<T>,
}
