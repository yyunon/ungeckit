use core::str;
use std::convert::From;
use std::fmt::format;
use std::str::FromStr;
use constants::WebdriverCmd;
use futures::TryFutureExt;
use reqwest::header::HeaderMap;
use serde::de::{self, DeserializeOwned};
use serde::Deserialize;
use tokio::io::AsyncReadExt;
use tokio::process::{Command};
use tracing_subscriber::field::debug;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::io::{self};
use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::task::JoinHandle;
use tokio::runtime::{Handle, Runtime, RuntimeMetrics};

use bytes::Bytes;
use reqwest::{self, Url, Method, Request, RequestBuilder, Response};
use log::*;

pub mod schemas;
pub mod constants;
pub mod service;
pub mod error;

use error::GeckError;
use schemas::session::*;
use service::*;
use constants::Firefox;

pub async fn _request<'a>(client: &reqwest::Client, 
                            method: Method, 
                            url: &str) -> Result<Bytes, GeckError>{
    let mut headers = HeaderMap::new();
    let body = r#"{"capabilities": {"alwaysMatch": {"webSocketUrl": true}}}"#;
    headers.insert("Accept", "application/json".parse().unwrap());
    headers.insert("Content-Type", "application/json;charset=UTF-8".parse().unwrap());
    let request = Request::new(method, Url::parse(&url).expect("Cannot parse url"));
    let response = RequestBuilder::from_parts(client.clone(), request)
                                    .headers(headers)
                                    .body(body)
                                    .send()
                                    .await?;
    if response.status() == 200 {
        Ok(response.bytes().await?)
    } else {
        Err(GeckError::new(error::ErrorKind::Driver, None::<GeckError>, &format!("Response status is {}", response.status())))
    }
}

pub struct Driver<'a> {
    driver_url: String,
    http_client: reqwest::Client,
    sessions: Vec<String>,
    service: Service,
    context: Arc<Mutex<Context>>,
    firefox: Firefox<'a>,
}


impl<'a> Driver<'a> {

    // TODO Create macro
    fn new(remote_url: Option<String>) -> Result<Self, GeckError> {
        let mut driver_url = constants::Driver::HOST.to_owned() + ":" + constants::Driver::PORT;
        if let Some(url) = remote_url {
            driver_url = url;
        }
        let context = Context::new();
        let mut service = Service::new(&context, &String::from("/usr/bin/geckodriver"));
        service.start(Vec::from([constants::Driver::ARGS_PORT, 
                                        constants::Driver::PORT, 
                                        constants::Driver::ARGS_VERBOSITY])).expect("Failed to execute driver");
        Ok(Self {
            http_client: reqwest::Client::new(),
            sessions: Vec::new(),
            driver_url: String::from(driver_url),
            context: context.clone(),
            service: service,
            firefox: constants::Firefox::new(),
        })
    }

    fn command<T>(&mut self, cmd: &str) -> Result<T, GeckError> 
    where T: de::DeserializeOwned
    {
        let client = self.http_client.clone();
        let url = self.driver_url.clone();
        let firefox = &self.firefox;
        let cmd = firefox.command_dict.get(cmd).unwrap();
        let method = Method::from_str(&cmd.verb).unwrap();
        let url = url.to_owned() + &cmd.path;
        // TODO Macro
        let body = self.context.lock().unwrap().handle.block_on(async move {
            _request(&client, method, &url).await
        }).unwrap();
        SchemaParser::try_parse_response(body)
    }
}


pub mod sync {
    use super::*;
    pub struct Driver<'a> {
        driver_url: String,
        http_client: reqwest::Client,
        sessions: Vec<String>,
        service: Service,
        context: Arc<Mutex<Context>>,
        firefox: Firefox<'a>,
    }


    impl<'a> Driver<'a> {

        // TODO Create macro
        pub async fn new(remote_url: Option<String>) -> Result<Self, GeckError> {
            let mut driver_url = constants::Driver::HOST.to_owned() + ":" + constants::Driver::PORT;
            if let Some(url) = remote_url {
                driver_url = url;
            }
            let context = Context::new();
            let mut service = Service::new(&context, &String::from("/home/yyunon/workspace/projects/freelance/scrape_pub/rust_gecko/rust_geck/binary/geckodriver"));
            service.start_async(Vec::from([constants::Driver::ARGS_PORT, 
                                            constants::Driver::PORT, 
                                            constants::Driver::ARGS_VERBOSITY])).await?;
            Ok(Self {
                http_client: reqwest::Client::new(),
                sessions: Vec::new(),
                driver_url: String::from(driver_url),
                context: context.clone(),
                service: service,
                firefox: constants::Firefox::new(),
            })
        }

        pub async fn command<T>(&mut self, cmd: &str) -> Result<T, GeckError> 
        where T: de::DeserializeOwned
        {
            let client = self.http_client.clone();
            let url = self.driver_url.clone();
            let firefox = &self.firefox;
            let cmd = firefox.command_dict.get(cmd).unwrap();
            let method = Method::from_str(&cmd.verb).unwrap();
            let url = url.to_owned() + &cmd.path;
            // TODO Macro
            let body = _request(&client, method, &url).await?;
            SchemaParser::try_parse_response(body)
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};
    use simplelog::*;

    use super::constants;
    #[test]
    fn test_dict() {
        let firefox = constants::Firefox::new();
        assert_eq!(firefox.command_dict.get("get_context").unwrap().verb, "GET");
    }

    fn test_spawn() {
        TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();
        let ten_millis = time::Duration::from_millis(500);
        let now = time::Instant::now();
        println!("Now running in main");
        thread::sleep(ten_millis);
    }
}
