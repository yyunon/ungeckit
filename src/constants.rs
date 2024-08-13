use std::collections::HashMap;
use reqwest::Method;

pub mod Driver {
    pub const HOST :&'static str = "http://127.0.0.1";
    pub const PORT :&'static str = "4444";
    pub const ARGS_PORT :&'static str = "-p";
    pub const ARGS_VERBOSITY :&'static str = "-v";
}

pub struct WebdriverCmd<'a> {
    pub verb: &'a str,
    pub path: &'a str
}

impl<'a, 'b> From<(&'a str, &'a str)> for WebdriverCmd<'b>
where 'a: 'b{
    fn from(inp: (&'a str, &'a str)) -> Self {
        WebdriverCmd {
            verb: inp.0,
            path: inp.1
        }
    }
}

pub struct Firefox<'a> {
  pub command_dict : HashMap<&'a str, WebdriverCmd<'a>> 
}

impl<'a> Firefox<'a>{
    pub fn new() -> Self {
        Self {
            command_dict: HashMap::from([
							("NEW_SESSION", WebdriverCmd::from(("POST", "/session/"))),
							("QUIT", WebdriverCmd::from(("DELETE", "/session/{sessionId}"))),
							("W3C_GET_CURRENT_WINDOW_HANDLE", WebdriverCmd::from(("GET", "/session/{sessionId}/window"))),
							("W3C_GET_WINDOW_HANDLES", WebdriverCmd::from(("GET", "/session/{sessionId}/window/handles"))),
							("GET", WebdriverCmd::from(("POST", "/session/{sessionId}/url"))),
							("GO_FORWARD", WebdriverCmd::from(("POST", "/session/{sessionId}/forward"))),
							("GO_BACK", WebdriverCmd::from(("POST", "/session/{sessionId}/back"))),
							("REFRESH", WebdriverCmd::from(("POST", "/session/{sessionId}/refresh"))),
							("W3C_EXECUTE_SCRIPT", WebdriverCmd::from(("POST", "/session/{sessionId}/execute/sync"))),
							("W3C_EXECUTE_SCRIPT_ASYNC", WebdriverCmd::from(("POST", "/session/{sessionId}/execute/async"))),
							("GET_CURRENT_URL", WebdriverCmd::from(("GET", "/session/{sessionId}/url"))),
							("GET_TITLE", WebdriverCmd::from(("GET", "/session/{sessionId}/title"))),
							("GET_PAGE_SOURCE", WebdriverCmd::from(("GET", "/session/{sessionId}/source"))),
							("SCREENSHOT", WebdriverCmd::from(("GET", "/session/{sessionId}/screenshot"))),
							("ELEMENT_SCREENSHOT", WebdriverCmd::from(("GET", "/session/{sessionId}/element/{id}/screenshot"))),
							("FIND_ELEMENT", WebdriverCmd::from(("POST", "/session/{sessionId}/element"))),
							("FIND_ELEMENTS", WebdriverCmd::from(("POST", "/session/{sessionId}/elements"))),
							("W3C_GET_ACTIVE_ELEMENT", WebdriverCmd::from(("GET", "/session/{sessionId}/element/active"))),
							("FIND_CHILD_ELEMENT", WebdriverCmd::from(("POST", "/session/{sessionId}/element/{id}/element"))),
							("FIND_CHILD_ELEMENTS", WebdriverCmd::from(("POST", "/session/{sessionId}/element/{id}/elements"))),
							("CLICK_ELEMENT", WebdriverCmd::from(("POST", "/session/{sessionId}/element/{id}/click"))),
							("CLEAR_ELEMENT", WebdriverCmd::from(("POST", "/session/{sessionId}/element/{id}/clear"))),
							("GET_ELEMENT_TEXT", WebdriverCmd::from(("GET", "/session/{sessionId}/element/{id}/text"))),
							("SEND_KEYS_TO_ELEMENT", WebdriverCmd::from(("POST", "/session/{sessionId}/element/{id}/value"))),
							("GET_ELEMENT_TAG_NAME", WebdriverCmd::from(("GET", "/session/{sessionId}/element/{id}/name"))),
							("IS_ELEMENT_SELECTED", WebdriverCmd::from(("GET", "/session/{sessionId}/element/{id}/selected"))),
							("IS_ELEMENT_ENABLED", WebdriverCmd::from(("GET", "/session/{sessionId}/element/{id}/enabled"))),
							("GET_ELEMENT_RECT", WebdriverCmd::from(("GET", "/session/{sessionId}/element/{id}/rect"))),
							("GET_ELEMENT_ATTRIBUTE", WebdriverCmd::from(("GET", "/session/{sessionId}/element/{id}/attribute/{name}"))),
							("GET_ELEMENT_PROPERTY", WebdriverCmd::from(("GET", "/session/{sessionId}/element/{id}/property/{name}"))),
							("GET_ELEMENT_ARIA_ROLE", WebdriverCmd::from(("GET", "/session/{sessionId}/element/{id}/computedrole"))),
							("GET_ELEMENT_ARIA_LABEL", WebdriverCmd::from(("GET", "/session/{sessionId}/element/{id}/computedlabel"))),
							("GET_SHADOW_ROOT", WebdriverCmd::from(("GET", "/session/{sessionId}/element/{id}/shadow"))),
							("FIND_ELEMENT_FROM_SHADOW_ROOT", WebdriverCmd::from(("POST", "/session/{sessionId}/shadow/{shadowId}/element"))),
							("FIND_ELEMENTS_FROM_SHADOW_ROOT", WebdriverCmd::from(("POST", "/session/{sessionId}/shadow/{shadowId}/elements"))),
							("GET_ALL_COOKIES", WebdriverCmd::from(("GET", "/session/{sessionId}/cookie"))),
							("ADD_COOKIE", WebdriverCmd::from(("POST", "/session/{sessionId}/cookie"))),
							("GET_COOKIE", WebdriverCmd::from(("GET", "/session/{sessionId}/cookie/{name}"))),
							("DELETE_ALL_COOKIES", WebdriverCmd::from(("DELETE", "/session/{sessionId}/cookie"))),
							("DELETE_COOKIE", WebdriverCmd::from(("DELETE", "/session/{sessionId}/cookie/{name}"))),
							("SWITCH_TO_FRAME", WebdriverCmd::from(("POST", "/session/{sessionId}/frame"))),
							("SWITCH_TO_PARENT_FRAME", WebdriverCmd::from(("POST", "/session/{sessionId}/frame/parent"))),
							("SWITCH_TO_WINDOW", WebdriverCmd::from(("POST", "/session/{sessionId}/window"))),
							("NEW_WINDOW", WebdriverCmd::from(("POST", "/session/{sessionId}/window/new"))),
							("CLOSE", WebdriverCmd::from(("DELETE", "/session/{sessionId}/window"))),
							("GET_ELEMENT_VALUE_OF_CSS_PROPERTY", WebdriverCmd::from(("GET", "/session/{sessionId}/element/{id}/css/{propertyName}"))),
							("EXECUTE_ASYNC_SCRIPT", WebdriverCmd::from(("POST", "/session/{sessionId}/execute_async"))),
							("SET_TIMEOUTS", WebdriverCmd::from(("POST", "/session/{sessionId}/timeouts"))),
							("GET_TIMEOUTS", WebdriverCmd::from(("GET", "/session/{sessionId}/timeouts"))),
							("W3C_DISMISS_ALERT", WebdriverCmd::from(("POST", "/session/{sessionId}/alert/dismiss"))),
							("W3C_ACCEPT_ALERT", WebdriverCmd::from(("POST", "/session/{sessionId}/alert/accept"))),
							("W3C_SET_ALERT_VALUE", WebdriverCmd::from(("POST", "/session/{sessionId}/alert/text"))),
							("W3C_GET_ALERT_TEXT", WebdriverCmd::from(("GET", "/session/{sessionId}/alert/text"))),
							("W3C_ACTIONS", WebdriverCmd::from(("POST", "/session/{sessionId}/actions"))),
							("W3C_CLEAR_ACTIONS", WebdriverCmd::from(("DELETE", "/session/{sessionId}/actions"))),
							("SET_WINDOW_RECT", WebdriverCmd::from(("POST", "/session/{sessionId}/window/rect"))),
							("GET_WINDOW_RECT", WebdriverCmd::from(("GET", "/session/{sessionId}/window/rect"))),
							("W3C_MAXIMIZE_WINDOW", WebdriverCmd::from(("POST", "/session/{sessionId}/window/maximize"))),
							("SET_SCREEN_ORIENTATION", WebdriverCmd::from(("POST", "/session/{sessionId}/orientation"))),
							("GET_SCREEN_ORIENTATION", WebdriverCmd::from(("GET", "/session/{sessionId}/orientation"))),
							("GET_NETWORK_CONNECTION", WebdriverCmd::from(("GET", "/session/{sessionId}/network_connection"))),
							("SET_NETWORK_CONNECTION", WebdriverCmd::from(("POST", "/session/{sessionId}/network_connection"))),
							("GET_LOG", WebdriverCmd::from(("POST", "/session/{sessionId}/se/log"))),
							("GET_AVAILABLE_LOG_TYPES", WebdriverCmd::from(("GET", "/session/{sessionId}/se/log/types"))),
							("CURRENT_CONTEXT_HANDLE", WebdriverCmd::from(("GET", "/session/{sessionId}/context"))),
							("CONTEXT_HANDLES", WebdriverCmd::from(("GET", "/session/{sessionId}/contexts"))),
							("SWITCH_TO_CONTEXT", WebdriverCmd::from(("POST", "/session/{sessionId}/context"))),
							("FULLSCREEN_WINDOW", WebdriverCmd::from(("POST", "/session/{sessionId}/window/fullscreen"))),
							("MINIMIZE_WINDOW", WebdriverCmd::from(("POST", "/session/{sessionId}/window/minimize"))),
							("PRINT_PAGE", WebdriverCmd::from(("POST", "/session/{sessionId}/print"))),
							("ADD_VIRTUAL_AUTHENTICATOR", WebdriverCmd::from(("POST", "/session/{sessionId}/webauthn/authenticator"))),
							("REMOVE_VIRTUAL_AUTHENTICATOR", WebdriverCmd::from(("DELETE", "/session/{sessionId}/webauthn/authenticator/{authenticatorId}"))),
							("ADD_CREDENTIAL", WebdriverCmd::from(("POST", "/session/{sessionId}/webauthn/authenticator/{authenticatorId}/credential"))),
							("GET_CREDENTIALS", WebdriverCmd::from(("GET", "/session/{sessionId}/webauthn/authenticator/{authenticatorId}/credentials"))),
							("REMOVE_CREDENTIAL", WebdriverCmd::from(("DELETE", "/session/{sessionId}/webauthn/authenticator/{authenticatorId}/credentials/{credentialId}"))),
							("REMOVE_ALL_CREDENTIALS", WebdriverCmd::from(("DELETE", "/session/{sessionId}/webauthn/authenticator/{authenticatorId}/credentials"))),
							("SET_USER_VERIFIED", WebdriverCmd::from(("POST", "/session/{sessionId}/webauthn/authenticator/{authenticatorId}/uv"))),
							("UPLOAD_FILE", WebdriverCmd::from(("POST", "/session/{sessionId}/se/file"))),
							("GET_DOWNLOADABLE_FILES", WebdriverCmd::from(("GET", "/session/{sessionId}/se/files"))),
							("DOWNLOAD_FILE", WebdriverCmd::from(("POST", "/session/{sessionId}/se/files"))),
							("DELETE_DOWNLOADABLE_FILES", WebdriverCmd::from(("DELETE", "/session/{sessionId}/se/files"))),
            ])
        }
    }
}