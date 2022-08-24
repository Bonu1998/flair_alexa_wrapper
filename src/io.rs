use std::{collections::HashMap, fmt};

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActionType {
    CustomTask,
    ConnectionsResponse,
    SessionEnd,
    Launch,
    Next,
    Previous,
    Stop,
    Help,
    More,
    FallBack,
    Unknown,
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            ActionType::Unknown => "UNKNOWN",
            ActionType::Launch => "LAUNCH",
            ActionType::Next => "NEXT",
            ActionType::Previous => "PREVIOUS",
            ActionType::Stop => "STOP",
            ActionType::Help => "HELP",
            ActionType::More => "MORE",
            ActionType::SessionEnd => "SESSION_END",
            ActionType::ConnectionsResponse => "CONNECTION_RESPONSE",
            ActionType::CustomTask => "CUSTOM_TASK",
            ActionType::FallBack => "FALLBACK",
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BussinessInput {
    pub request_type:String,
    pub action: String,
    pub user_id: String,
    pub session_id: String,
    pub device_size: String,
    pub is_display_enabled: bool,
    pub is_new_session: bool,
    pub locale: String,
    pub args: Vec<HashMap<String, String>>,
    pub extras: Option<HashMap<String, JsonValue>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BussinessOutput {
    pub should_end_session: Option<bool>,
    pub prompt_speech: Option<String>,
    pub reprompt_speech: Option<String>,
    pub commands: Option<Vec<ResponseCommand>>
}

impl BussinessOutput {
    pub fn new() -> BussinessOutput {
        BussinessOutput {
            should_end_session: None,
            prompt_speech: None,
            reprompt_speech: None,
            commands: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseCommand {
    pub command_type: String,
    pub keys: Option<HashMap<String, String>>,
    pub data: Option<HashMap<String, HashMap<String, String>>>,
    pub list_data: Option<Vec<HashMap<String, HashMap<String, String>>>>,
    pub random: Option<HashMap<String, JsonValue>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ResponseCommandType {
    SendEvent,
    ControlMedia,
    AplRenderTemplate,
    AplaRenderTemplate,
    SetSessionAttribute
}

impl fmt::Display for ResponseCommandType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            ResponseCommandType::SendEvent => "SEND_EVENT",
            ResponseCommandType::ControlMedia => "CONTROL_MEDIA",
            ResponseCommandType::AplRenderTemplate => "APL_RENDER_TEMPLATE",
            ResponseCommandType::AplaRenderTemplate => "APLA_RENDER_TEMPLATE",
            ResponseCommandType::SetSessionAttribute => "SET_SESSION_ATTRIBUTES"
        };
        write!(f, "{}", s)
    }
}

impl ResponseCommand {
    pub fn new(command_type: String) -> ResponseCommand {
        ResponseCommand {
            command_type,
            keys: None,
            data: None,
            list_data: None,
            random: None
        }
    }
}
