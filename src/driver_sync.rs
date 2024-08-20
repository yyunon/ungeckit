use core::{panic, str};
use serde::de::{self};
use serde_json::json;
use std::borrow::Borrow;
use std::convert::From;
use std::sync::{Arc, Mutex};

use handlebars::Handlebars;

use crate::options::{capabilities, Capabilities, DriverOptions};
use crate::schemas::session::{self, *};
use crate::service::*;
use crate::utils::error::GeckError;
use crate::utils::*;

pub mod sync {
    // TODO implement
    use super::*;
    pub struct Driver {}

    impl Driver {}
}

#[cfg(test)]
mod tests {

    use simplelog::*;
    use std::{thread, time};

    use super::webdriver_commands;
    #[test]
    fn test_dict() {
        let firefox = webdriver_commands::Firefox::new();
        assert_eq!(firefox.command_dict.get("get_context").unwrap().verb, "GET");
    }
}
