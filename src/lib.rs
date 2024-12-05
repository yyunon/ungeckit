pub mod driver;
pub mod driver_sync;
pub mod options;
pub mod schemas;
pub mod service;
pub mod utils;
pub mod cdp;
pub mod package;

//use crate::package::PackageManager;
use crate::driver::WebDriver;
use crate::options::{capabilities, Capabilities, DriverOptions};
use crate::utils::error::GeckError;
use log::*;

#[macro_export]
macro_rules! driver{
    ($x: expr) => {
        {
            let mut driver = DriverBuilder::new(None)
                                .build()
                                .unwrap();
            if let Some(s) = $x {
                let driver = DriverBuilder::new(Some(s)).build().unwrap();
                return driver;
            } else {
                let driver = DriverBuilder::new(None).build().unwrap();
                return driver;
            }
        }
    };
}
#[macro_export]
macro_rules! get_page{
    ($x: expr) => {
        {
            let mut driver = DriverBuilder::new(None)
                                .build()
                                .unwrap();
            let page = driver.get($x);
            page
        }
    };
}

#[macro_export]
macro_rules! save_screenshot{
    ($x: expr, $y: expr) => {
        {
            let mut driver = DriverBuilder::new(None)
                                .build()
                                .unwrap();
            let page = driver.get($x);
            driver.save_screenshot($y);
        }
    };
}
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
    pub fn options(&mut self, options: DriverOptions) -> Result<&mut Self, GeckError> {
        self.options = Some(options);
        Ok(self)
    }

    /// Insert a new option
    /// TODO: Make Macro
    pub fn option(&mut self, option_type: &str, option: (&str, &str)) -> Result<&mut Self, GeckError> {
        if !DriverOptions::option_types()
            .iter()
            .any(|&i| i == option_type)
        {
            panic!("Invalid option")
        }
        self.options.as_mut().unwrap().insert(option_type, option);
        Ok(self)
    }

    /// If no options provided, or some provided
    pub fn setup_default_options(&mut self) -> Result<&mut Self, GeckError> {
        self.options
            .as_mut()
            .unwrap()
            .insert("--arg", ("-headless", ""));
        self.options
            .as_mut()
            .unwrap()
            .insert("--log", ("level", "trace"));
        let cap = self.options.as_ref().unwrap().to_capabilities().unwrap();
        self.capabilities.as_mut().unwrap().extend(cap);
        Ok(self)
    }

    /// If no capabilities provided, or some provided
    pub fn setup_default_capabilities(&mut self) -> Result<&mut Self, GeckError> {
        self.capabilities = Some(capabilities::DesiredCapabilities::FIREFOX.values());
        self.capabilities
            .as_mut()
            .unwrap()
            .insert("alwaysMatch", "webSocketUrl", true);
        Ok(self)
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
