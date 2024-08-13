use bytes::Bytes;
use serde::{self, de, Deserialize, Serialize};

use crate::error::GeckError;

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
    pub pageLoad: i64,
    pub script: i64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Capabilities {
    pub acceptInsecureCerts: bool,
    pub browserName: String,
    pub browserVersion: String,
    #[serde(alias = "moz:accessibilityChecks")]
    pub moz_accessibilityChecks: bool,
    #[serde(alias = "moz:buildID")]
    pub moz_buildID: String,
    #[serde(alias = "moz:debuggerAddress")]
    pub moz_debuggerAddress: String,
    #[serde(alias = "moz:geckodriverVersion")]
    pub moz_geckodriverVersion: String,
    #[serde(alias = "moz:headless")]
    pub moz_headless: bool,
    #[serde(alias = "moz:platformVersion")]
    pub moz_platformVersion: String,
    #[serde(alias = "moz:processID")]
    pub moz_processID: i64,
    #[serde(alias = "moz:profile")]
    pub moz_profile: String,
    #[serde(alias = "moz:shutdownTimeout")]
    pub moz_shutdownTimeout: i64,
    #[serde(alias = "moz:webdriverClick")]
    pub moz_webdriverClick: bool,
    #[serde(alias = "moz:windowless")]
    pub moz_windowless: bool,
    pub pageLoadStrategy: String,
    pub platformName: String,
    pub proxy: Proxy,
    pub setWindowRect: bool,
    pub strictFileInteractability: bool,
    pub timeouts: Timeouts,
    pub unhandledPromptBehavior: String,
    pub userAgent: String,
    pub webSocketUrl: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub sessionId: String,
    pub capabilities: Capabilities,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SessionResponse {
    pub value: Session,
}
