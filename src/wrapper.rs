use bincode;
use flair_alexa_sdk::{request::Request as AlexaRequest, response::Response as AlexaResponse};
use flair_general_utils::file_fetch::post_data;
use flair_types::skill::io::BussinessOutput;
use log::{error, debug};
use std::collections::HashMap;

use crate::{auth::{io::AlexaAuthInput, verifier}, pre_processing::pre_processing, post_processing::post_processing};

pub async fn alexa_wrapper(
    skill_info: HashMap<String, String>,
    alexa_request: AlexaRequest,
    signature: String,
    signature_cert_chain_url: String,
    bussiness_path: String,
) -> AlexaResponse {
    let mut _resp = AlexaResponse::default_session_close();
    let request_body_as_bytes = bincode::serialize(&alexa_request).unwrap_or_default();
    let auth_input = AlexaAuthInput::new(
        signature,
        signature_cert_chain_url,
        alexa_request.request_body.timestamp.clone(),
        request_body_as_bytes,
    );
    let auth_output = verifier(auth_input);
    debug!("after auth");
    if auth_output.success == true {
        let bussiness_input = pre_processing(alexa_request, skill_info);
        match serde_json::to_value(bussiness_input) {
            Ok(_bussiness_input) => {
                match post_data::<BussinessOutput>(bussiness_path, _bussiness_input, HashMap::new()).await {
                    Ok(bussiness_output) => {
                        _resp = post_processing(bussiness_output).await
                    },
                    Err(e) => {error!("\nBussiness {}", e);},
                }
            },
            Err(e) => {error!("\n{}", e);},
        }
    }else {
        error!("\n auth failed");
    }
    _resp
}
