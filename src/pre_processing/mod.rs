use std::collections::HashMap;
use serde_json::Value as JsonValue;

pub mod helpers;

use flair_alexa_sdk::request::{Request as AlexaRequest, RequestType};
use log::{debug, error, warn};

use crate::io::{ActionType, BussinessInput};

use self::helpers::get_device_size;

pub fn pre_processing(
    input: AlexaRequest,
    mut skill_info: HashMap<String, String>,
) -> BussinessInput {
    let mut args: Vec<HashMap<String, String>> = Vec::new();

    let is_display_enabled = input.is_apl_supported();
    let is_new_session = input.is_new_session();
    let locale = input.get_locale();

    // Additional Skill Info
    skill_info.insert("id".to_string(), "SKILL_INFO".to_string());
    if let Some(system) = input.context.system.clone() {
        skill_info.insert("api_end_point".to_string(), system.api_endpoint);
        skill_info.insert("api_access_token".to_string(), system.api_access_token);
    }
    skill_info.insert(
        "request_id".to_string(),
        input.request_body.request_id.clone(),
    );
    if let Some(apl_context) = input.context.apl_context.clone() {
        skill_info.insert("last_apl_token".to_string(), apl_context.token);
    }
    args.push(skill_info);

    let user_id = input.get_user_id().unwrap_or_default();
    let session_id = input.get_session_id().unwrap_or_default();
    let device_size = get_device_size(&input);

    // Slot Values
    let request_type = RequestType::from_str(input.request_body.req_type.as_str().clone());
    let slot_values = input.get_slot_values();
    if slot_values.len() > 0 {
        args.extend(slot_values)
    };

    // AudioPlayer details
    if let Some(mut audio_player_details) = input.context.audio_player_details.clone() {
        audio_player_details.insert("id".to_string(), "AUDIO_PLAYER_DETAILS".to_string());
        if !audio_player_details.is_empty() {
            args.push(audio_player_details)
        }
    }

    // User Event Args
    if let Some(user_event_args) = input.request_body.arguments.clone() {
        args.extend(user_event_args);
    }

    // Access token
    let access_token = input.get_user_access_token();
    match access_token {
        Ok(token) => {
            let mut token_args: HashMap<String, String> = HashMap::new();
            token_args.insert("id".to_string(), "ACCESS_TOKEN".to_string());
            token_args.insert("token".to_string(), token);
            args.push(token_args);
        }
        Err(e) => debug!("\n{}\n", e),
    }

    // Action type
    let mut action = ActionType::Unknown.to_string();
    match request_type {
        RequestType::AplUserEvent => {
            if let Some(user_args) = args
                .iter()
                .find(|x| x.contains_key("USER_EVENT") && x.contains_key("INTENT_NAME"))
            {
                if let Some(intent_name) = user_args.get("INTENT_NAME") {
                    action = intent_name.clone();
                };
            };
        }
        RequestType::IntentRequest => match input.clone().get_intent_name() {
            Ok(intent_name) => {
                action = intent_name;
            }
            Err(e) => {
                error!("\n {}", e);
            }
        },
        RequestType::LaunchRequest => {
            if let Some(cause) = input.request_body.cause.clone() {
                action = ActionType::CustomTask.to_string();
                let mut task_args: HashMap<String, String> = cause.to_hash_map();
                task_args.insert("id".to_string(), "CUSTOM_TASK_ARGS".to_string());
                args.push(task_args);
            } else {
                action = ActionType::Launch.to_string()
            }
        }
        RequestType::AudioPlayerPlaybackStarted => todo!(),
        RequestType::AudioPlayerPlaybackFinished => todo!(),
        RequestType::AudioPlayerPlaybackNearlyFinished => todo!(),
        RequestType::AudioPlayerPlaybackStopped => todo!(),
        RequestType::PlaybackControllerNextCommandIssued => todo!(),
        RequestType::PlaybackControllerPreviousCommandIssued => todo!(),
        RequestType::PlaybackControllerPlayCommandIssued => todo!(),
        RequestType::PlaybackControllerPauseCommandIssued => todo!(),
        RequestType::AudioPlayerPlaybackFailed => todo!(),
        RequestType::SessionEndedRequest => action = ActionType::SessionEnd.to_string(),
        RequestType::ConnectionsResponse => {
            action = ActionType::ConnectionsResponse.to_string()
            // purchase entities
        }
        RequestType::CanFulfillIntentRequest => todo!(),
        RequestType::DisplayElementSelected => todo!(),
        RequestType::SessionResumedRequest => todo!(),
    }
    let mut extras:HashMap<String, JsonValue> = HashMap::new();
    match input.get_session_attributes() {
        Ok(data) => {
            extras.insert("session_data".to_string(), data);
        },
        Err(e) => warn!("\n{}", e),
    }
    BussinessInput {
        request_type: "ALEXA".to_string(),
        action,
        user_id,
        session_id,
        device_size,
        is_display_enabled,
        is_new_session,
        locale,
        args,
        extras: Some(extras)
    }
}
