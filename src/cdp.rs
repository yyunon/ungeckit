/*
Holds the logic to communicate with dev tools via CDP
 */
use crate::utils::webdriver_commands::WebdriverCmd;
use std::collections::HashMap;

pub struct CDP {
    pub command_dict: HashMap<&'static str, WebdriverCmd<'static>>,
}

impl CDP {
    pub fn new() -> Self {
        Self {
            command_dict: HashMap::from([
							("json", WebdriverCmd::from(("POST", "/json"))),
							("protocol", WebdriverCmd::from(("POST", "/json/protocol"))),
							("list", WebdriverCmd::from(("POST", "/json/list"))),
							("new", WebdriverCmd::from(("POST", "/json/new?{{url}}"))),
							("activate", WebdriverCmd::from(("POST", "/json/activate/{{id}}"))),
							("close", WebdriverCmd::from(("POST", "/json/close/{{id}}"))),
            ])
        }
    }
}