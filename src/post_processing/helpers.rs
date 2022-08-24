use crate::io::ResponseCommand;
use flair_alexa_sdk::response::directive::Directive;
use flair_general_utils::file_fetch::get_data;
use log::{error, warn};
use serde_json::{json, Value as JsonValue};
use std::collections::HashMap;

pub fn send_event_helper(command: ResponseCommand) -> Directive {
    let mut _args: Vec<HashMap<String, String>> = Vec::new();
    let mut token = String::from("");
    if let Some(data) = command.data {
        if let Some(mut user_args) = data.get("send_event_args").cloned() {
            if !user_args.contains_key("id") {
                user_args.insert("id".to_string(), "USER_EVENT".to_string());
            }
            _args.push(user_args);
        }
    }
    if let Some(keys) = command.keys {
        if let Some(_token) = keys.get("token").cloned() {
            token = _token
        }
    }
    Directive::apl_execute_commands(
        token,
        Some(vec![json!({
            "type": "SendEvent",
            "arguments": _args,
        })]),
    )
}

pub fn control_media_helper(command: ResponseCommand) -> Directive {
    let mut value: i32 = 0;
    let mut component_id: String = String::from("");
    let mut commmand_name: String = String::from("");
    let mut token = String::from("");

    if let Some(keys) = command.keys {
        if let Some(val) = keys.get("value").cloned() {
            match val.parse::<i32>() {
                Ok(int_val) => value = int_val,
                Err(e) => {
                    warn!("\n Not a number: {}\n", e);
                }
            }
        }
        if let Some(comp_id) = keys.get("component_id").cloned() {
            component_id = comp_id;
        }
        if let Some(comp_name) = keys.get("commmand_name").cloned() {
            commmand_name = comp_name;
        }
        if let Some(_token) = keys.get("token").cloned() {
            token = _token
        }
    }
    Directive::apl_execute_commands(
        token,
        Some(vec![json!({
            "type": "ControlMedia",
            "componentId": component_id,
            "command": commmand_name,
            "value":value
        })]),
    )
}

pub async fn render_template(command: ResponseCommand, _type: String) -> Result<Directive, String> {
    let mut template_url = String::from("");
    let mut token = String::from("");
    let mut document: Option<JsonValue> = None;
    if let Some(keys) = command.keys.clone() {
        if let Some(temp_url) = keys.get("template_url").cloned() {
            template_url = temp_url
        }

        if let Some(_token) = keys.get("token").cloned() {
            token = _token
        }
    }

    match get_data::<JsonValue>(template_url).await {
        Ok(file) => document = Some(file),
        Err(e) => {
            error!("\n Fetching apl template: {}\n", e);
        }
    }
    let mut data: Option<JsonValue> = None;
    match serde_json::to_value(command) {
        Ok(_data) => data = Some(_data),
        Err(err) => {
            error!("\n {}\n", err);
        }
    }
    if _type.as_str() == "APLA" {
        Ok(Directive::apla_render_document(token, document, data, None))
    } else if _type.as_str() == "APL" {
        Ok(Directive::apl_render_document(token, document, data, None))
    } else {
        Err("Type not define".to_string())
    }
}
