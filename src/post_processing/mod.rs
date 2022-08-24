use self::helpers::{control_media_helper, render_template, send_event_helper};
use flair_alexa_sdk::response::{speech::Speech, Response as AlexaResponse};
use crate::io::BussinessOutput;
use log::{error, warn};

mod helpers;

pub async fn post_processing(input: BussinessOutput) -> AlexaResponse {
    let mut _resp = AlexaResponse::new("1.0".to_string());
    // prompt speech
    if let Some(prompt_speech) = input.prompt_speech {
        _resp.speak(Speech::ssml(format!("<speak>{}</speak>", prompt_speech)));
    }

    // reprompt speech
    if let Some(reprompt_speech) = input.reprompt_speech {
        _resp.reprompt(Speech::ssml(format!("<speak>{}</speak>", reprompt_speech)));
    }

    // should end session
    if let Some(should_end_session) = input.should_end_session {
        _resp.with_should_end_session(should_end_session);
    }

    if let Some(commands) = input.commands {
        for command in commands {
            match command.command_type.as_str() {
                "SEND_EVENT" => _resp.add_directive(send_event_helper(command)),
                "CONTROL_MEDIA" => _resp.add_directive(control_media_helper(command)),
                "APL_RENDER_TEMPLATE" => match render_template(command, "APL".to_string()).await {
                    Ok(_directive) => _resp.add_directive(_directive),
                    Err(e) => error!("\n {}\n", e),
                },
                "APLA_RENDER_TEMPLATE" => {
                    match render_template(command, "APLA".to_string()).await {
                        Ok(_directive) => _resp.add_directive(_directive),
                        Err(e) => error!("\n {}\n", e),
                    }
                },
                "SET_SESSION_ATTRIBUTES" =>{
                    if let Some(_random)  = command.random{
                        _resp.set_session_attributes(_random);
                    }
                },
                _ => {
                    warn!("\ncommand type miss match: {}\n", command.command_type);
                }
            }
        }
    }
    _resp
}
