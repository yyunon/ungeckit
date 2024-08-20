use core::panic;
use log::*;
use std::borrow::Borrow;
use std::{collections::HashMap, vec};

use crate::utils::error::GeckError;
use crate::utils::types::DictType;
use crate::utils::types::*;
use serde::Serialize;

pub mod capabilities {
    use Dict;

    use super::*;
    // The idea of DesiredCapabilities is to provide const values to be used during options
    // TODO We only implement Firefox as create is for Firefox
    #[derive(Serialize)]
    pub enum DesiredCapabilities {
        FIREFOX,
    }

    impl DesiredCapabilities {
        pub fn values(&self) -> Capabilities {
            match *self {
                DesiredCapabilities::FIREFOX => {
                    let mut cap = Capabilities::new();
                    cap.insert("alwaysMatch", "browserName", "firefox");
                    cap.insert("alwaysMatch", "acceptInsecureCerts", true);
                    cap.insert("alwaysMatch", "moz:debuggerAddress", true);
                    cap
                }
            }
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Capabilities {
    capabilities: Dict,
}
impl Capabilities {
    pub fn new() -> Self {
        Self {
            capabilities: Dict::new(),
        }
    }
    pub fn insert<T: Into<DictType>>(&mut self, match_type: &str, k: &str, v: T) {
        let mut temp = Dict::new();
        temp.insert(k, v);
        self.capabilities.insert(match_type, temp)
    }
    pub fn extend(&mut self, cap: Self) {
        self.capabilities.extend(cap.capabilities)
    }
}
#[derive(Serialize, Debug)]
pub struct DriverOptions {
    profile: Option<String>,
    args: Option<Vec<String>>,
    prefs: Option<Dict>,
    log: Option<Dict>,
}

// TODO We don't support all the options right now, only firefox
impl DriverOptions {
    /*
    We define Firefox options in this implementations
    */
    pub const KEY: &'static str = "moz:firefoxOptions";

    pub fn new() -> Self {
        Self {
            profile: None,
            args: Some(Vec::new()),
            prefs: Some(Dict::new()),
            log: Some(Dict::new()),
        }
    }

    pub fn profile(&mut self, v: &str) {
        self.profile = Some(v.to_owned())
    }

    pub fn args(&mut self, v: Vec<&str>) {
        self.args = Some(v.iter().map(|&s| s.to_owned()).collect())
    }

    pub fn arg(&mut self, v: &str) {
        self.args.as_mut().unwrap().push(v.to_owned())
    }

    pub fn prefs(&mut self, val: &HashMap<&str, &str>) {
        self.prefs = Some(val.into());
    }

    pub fn pref(&mut self, val: (&str, &str)) {
        self.prefs.as_mut().unwrap().insert(val.0, val.1)
    }

    pub fn logs(&mut self, val: &HashMap<&str, &str>) {
        self.log = Some(val.into());
    }

    pub fn log(&mut self, val: (&str, &str)) {
        self.log.as_mut().unwrap().insert(val.0, val.1)
    }

    pub fn insert(&mut self, option_type: &str, option: (&str, &str)) {
        match option_type {
            "--arg" => self.arg(option.0),
            "--pref" => self.pref(option),
            "--log" => self.log(option),
            _ => panic!("Non existing driver option"),
        }
    }

    pub fn option_types() -> Vec<&'static str> {
        vec!["--arg", "--pref", "--log"]
    }

    pub fn to_capabilities(&self) -> Result<Capabilities, GeckError> {
        // TODO We do a lot of copies here
        let mut cap = Capabilities::new();
        let mut option_dict = Dict::new();
        if let Some(opt) = self.profile.borrow() {
            option_dict.insert("profile", opt.clone());
        }
        if let Some(args) = self.args.borrow() {
            option_dict.insert::<Vec<String>>("args", args.clone().into());
        }
        if let Some(logs) = self.log.borrow() {
            option_dict.insert::<Dict>("log", logs.clone().into());
        }
        if let Some(prefs) = self.prefs.borrow() {
            option_dict.insert::<Dict>("prefs", prefs.clone().into());
        }
        cap.insert::<Dict>("alwaysMatch", "moz:firefoxOptions", option_dict.into());
        Ok(cap)
    }
}

#[cfg(test)]
mod tests {

    use super::Capabilities;
    use super::*;
    use assert_json_diff::assert_json_include;
    use serde_json;

    #[test]
    fn test_options() {
        let mut options = DriverOptions::new();
        options.args(["-headless"].into());
        let mut prefs = HashMap::new();
        prefs.insert("dom.ipc.processCount", "8");
        options.prefs(&prefs);
    }

    #[test]
    fn test_serialize() {
        let capabilities = capabilities::DesiredCapabilities::FIREFOX.values();
        let actual: serde_json::Value = serde_json::from_str(
            r#"{"moz::debuggerAddress":true, "acceptInsecureCerts":true,"browserName":"firefox"}"#,
        )
        .unwrap();
        let expected: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(&capabilities).unwrap()).unwrap();
        //assert_json_include!(expected: expected , actual: actual)
    }
}
