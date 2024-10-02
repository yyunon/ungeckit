pub mod driver;
pub mod driver_sync;
pub mod options;
pub mod schemas;
pub mod service;
pub mod utils;
pub mod cdp;

use crate::driver::WebDriver;
use crate::options::{capabilities, Capabilities, DriverOptions};
use crate::utils::error::GeckError;
use log::*;

pub struct DriverBuilder {
    pub options: Option<DriverOptions>,
    pub capabilities: Option<Capabilities>,
}

impl DriverBuilder {
    /// Builds the driver from Options, Context (Sync and Async engine), Driver parameters
    // TODO Make macro
    pub fn new(capabilities: Option<Capabilities>) -> Self {
        Self {
            options: None,
            capabilities: capabilities,
        }
    }

    /// Override options
    pub fn options(&mut self, options: DriverOptions) -> Result<(), GeckError> {
        self.options = Some(options);
        Ok(())
    }

    /// Insert a new option
    /// TODO: Make Macro
    pub fn option(&mut self, option_type: &str, option: (&str, &str)) -> Result<(), GeckError> {
        if !DriverOptions::option_types()
            .iter()
            .any(|&i| i == option_type)
        {
            panic!("Invalid option")
        }
        self.options.as_mut().unwrap().insert(option_type, option);
        Ok(())
    }

    /// If no options provided, or some provided
    pub fn setup_default_options(&mut self) -> Result<(), GeckError> {
        //self.options
        //    .as_mut()
        //    .unwrap()
        //    .insert("--arg", ("-headless", ""));
        self.options
            .as_mut()
            .unwrap()
            .insert("--log", ("level", "trace"));
        let cap = self.options.as_ref().unwrap().to_capabilities().unwrap();
        self.capabilities.as_mut().unwrap().extend(cap);
        Ok(())
    }

    /// If no capabilities provided, or some provided
    pub fn setup_default_capabilities(&mut self) -> Result<(), GeckError> {
        self.capabilities = Some(capabilities::DesiredCapabilities::FIREFOX.values());
        self.capabilities
            .as_mut()
            .unwrap()
            .insert("alwaysMatch", "webSocketUrl", true);
        Ok(())
    }

    /// Builds a firefox driver
    pub fn build(&mut self) -> Result<WebDriver, GeckError> {
        // Setup options
        self.options = Some(DriverOptions::new());
        self.setup_default_capabilities();
        self.setup_default_options();
        Ok(WebDriver::new(
            None,
            serde_json::to_string(&self.capabilities.as_ref().unwrap()).unwrap(),
            reqwest::Client::new(),
        ))
    }

    pub async fn build_async(&mut self) -> Result<driver_sync::sync::WebDriver, GeckError> {
        // Setup options
        self.options = Some(DriverOptions::new());
        self.setup_default_capabilities();
        self.setup_default_options();
        Ok(driver_sync::sync::WebDriver::new(
            None,
            serde_json::to_string(&self.capabilities.as_ref().unwrap()).unwrap(),
            reqwest::Client::new(),
        ).await)
    }

    pub fn serialized_capabilities(&mut self) -> Result<String, GeckError> {
        let cap = self.options.as_ref().unwrap().to_capabilities().unwrap();
        self.capabilities.as_mut().unwrap().extend(cap);
        Ok(serde_json::to_string(&self.capabilities).unwrap())
    }
}
