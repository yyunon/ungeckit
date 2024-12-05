use base64::prelude::*;
use core::{panic, str};
use log::info;
use serde::de::{self};
use std::convert::From;
use std::fs::{self, OpenOptions};
use std::path::Path;
use std::io::Write;
use std::sync::{Arc, Mutex};

use crate::options::{capabilities, Capabilities, DriverOptions};
use crate::schemas::session::{self, *};
use crate::{cdp::*, service::*};
use crate::utils::error::GeckError;
use crate::utils::*;

pub struct WebDriver {
    pub service: Service,
    pub open_page: Option<String>,
    pub context: Arc<Mutex<Context>>,
    pub cdp: Option<CDP>,
    pub session: Option<Session>,
    pub capabilities: String,
    pub driver_url: String,
    pub http_client: reqwest::Client,
    pub firefox: webdriver_commands::WebDriver<'static>,
}
impl WebDriver {
    pub fn new(
        remote_url: Option<String>,
        capabilities: String,
        http_client: reqwest::Client,
    ) -> Self {
        let context = Context::new();
        let mut service = Service::new(&context, &String::from("geckodriver"));

        service
            .start(Vec::from([
                webdriver_commands::Driver::ARGS_PORT,
                webdriver_commands::Driver::PORT,
                webdriver_commands::Driver::ARGS_VERBOSITY,
            ]))
            .expect("Failed to execute driver");

        let mut driver_url =
            webdriver_commands::Driver::HOST.to_owned() + ":" + webdriver_commands::Driver::PORT;
        if let Some(url) = remote_url {
            driver_url = url;
        }
        while !service.session_is_up().unwrap() {
            info!("Session is booting up, give it more seconds...");
        }

        let mut cmds = webdriver_commands::WebDriver::new();
        cmds.insert("GET_CONTEXT", "GET", "/session/{{sessionId}}/moz/context");
        cmds.insert("SET_CONTEXT", "POST", "/session/{{sessionId}}/moz/context");
        cmds.insert(
            "INSTALL_ADDON",
            "POST",
            "/session/{{sessionId}}/moz/addon/install",
        );
        cmds.insert(
            "UNINSTALL_ADDON",
            "POST",
            "/session/{{sessionId}}/moz/addon/uninstall",
        );
        cmds.insert(
            "FULL_PAGE_SCREENSHOT",
            "GET",
            "/session/{{sessionId}}/moz/screenshot/full",
        );

        Self {
            service: service,
            open_page: None,
            context: context,
            cdp: None,
            session: None,
            capabilities: capabilities,
            driver_url: driver_url,
            http_client: http_client,
            firefox: cmds,
        }
    }

    /// Generates a session per driver, we maintain a single session per driver at this point
    pub fn new_session(&mut self) -> Result<(), GeckError> {
        info!("{:?}", self.capabilities);
        let session = self
            .command::<SessionResponse>("NEW_SESSION", r#"{}"#, self.capabilities.clone())
            .unwrap();
        self.session = Some(session.value);

        let ws_url = &self.session.as_ref().unwrap().capabilities.web_socket_url;
        let mut cdp = CDP::new(self.context.clone(), &ws_url);

        // TODO CAN BE BETTER DONE? 
        // Everytime we create a session we pass some commands to websocket
        // Credits to: https://github.com/ultrafunkamsterdam/undetected-chromedriver/blob/master/undetected_chromedriver/__init__.py
        let mut resp = cdp.send("session.subscribe", r#"{"events":["browsingContext.domContentLoaded"]}"#).unwrap();
        resp = cdp.send("browser.createUserContext", "{}").unwrap();
        let mut userContext = &resp["result"]["userContext"].as_str().unwrap();
        //cdp.send("Page.addScriptToEvaluateOnNewDocument", "").unwrap();
        self.cdp = Some(cdp);

        Ok(())
    }

    pub fn get(&mut self, url: &str) -> Result<String, GeckError> {
        match &self.session {
            Some(_) => (),
            None => self.new_session().unwrap(),
        };

        self.command::<Response<String>>(
            "GET",
            &format!(
                r#"{{"sessionId": "{}"}}"#,
                self.session.as_ref().unwrap().session_id
            ),
            format!(r#"{{"url": "{}"}}"#, url),
        )
        .unwrap();
        let page_source = self
            .command::<Response<String>>(
                "GET_PAGE_SOURCE",
                &format!(
                    r#"{{"sessionId": "{}"}}"#,
                    self.session.as_ref().unwrap().session_id
                ),
                format!(r#"{{"url": "{}"}}"#, url),
            )
            .unwrap();
        self.open_page = Some(url.to_owned());
        Ok(page_source.value.unwrap())
    }

    /// Execute a W3C script
    pub fn execute_script(&mut self, script: &str, args: &str) -> Result<String, GeckError>{
        match &self.session {
            Some(_) => (),
            None => self.new_session().unwrap(),
        };
        let data = format!(r#"{{"script": "{script}", args: {args}}}"#);
        let result = self
            .command::<Response<String>>(
                "W3C_EXECUTE_SCRIPT",
                &format!(
                    r#"{{"sessionId": "{}"}}"#,
                    self.session.as_ref().unwrap().session_id
                ),
                data
            )
            .unwrap();
        Ok(result.value.unwrap())
    }

    /// Save the screenshot of the webpage, uses moz capabilities full screenshot option.
    /// Creates the file if not exists.
    pub fn save_screenshot(&mut self, path: &str) -> Result<(), GeckError> {
        match &self.session {
            Some(_) => (),
            None => self.new_session().unwrap(),
        };

        let screenshot = self
            .command::<Response<String>>(
                "FULL_PAGE_SCREENSHOT",
                &format!(
                    r#"{{"sessionId": "{}"}}"#,
                    self.session.as_ref().unwrap().session_id
                ),
                "".to_owned(),
            )
            .unwrap()
            .value
            .unwrap();

        let img_bytes = BASE64_STANDARD.decode(screenshot.as_bytes()).unwrap();
        if Path::new(path).exists() {
            let mut file = fs::OpenOptions::new().write(true).open(path).unwrap();
            file.write_all(&img_bytes).unwrap();
        } else {
            let mut file = fs::File::create(path).unwrap();
            file.write_all(&img_bytes).unwrap();
        }

        Ok(())
    }

    /// A private implementation to communicate with geckodriver
    ///
    /// An example usage is as such:
    /// let session = self
    ///     .command::<SessionResponse>(
    ///         "NEW_SESSION",
    ///         r#"{}"#,
    ///         r#"{"capabilities": {"alwaysMatch": {"webSocketUrl": true}}}"#.to_owned(),
    ///     )
    ///     .unwrap();
    ///
    /// This returns a Session object
    /// TODO A better way to pass args
    fn command<T>(&mut self, cmd: &str, args: &str, data: String) -> Result<T, GeckError>
    where
        T: de::DeserializeOwned,
    {
        let client = self.http_client.clone();
        let url = self.driver_url.clone();
        let firefox = &self.firefox;
        let cmd = firefox.command_dict.get(cmd).unwrap();
        let url = url.to_owned() + &webdriver_commands::template_str(&cmd.path, args).unwrap();
        // TODO Macro
        let body =
            self.context.lock().unwrap().handle.block_on(async move {
                net::http::request(&client, &cmd.verb, &url, data).await.unwrap()
            });
        SchemaParser::try_parse_response(body)
    }
}

#[cfg(test)]
mod tests {

    use simplelog::*;
    use std::{thread, time};

    use super::webdriver_commands;
    #[test]
    fn test_dict() {
        let firefox = webdriver_commands::WebDriver::new();
        assert_eq!(firefox.command_dict.get("NEW_SESSION").unwrap().verb, "POST");
    }
}
