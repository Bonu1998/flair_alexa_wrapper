use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlexaAuthInput {
    pub signature: String,
    pub signature_cert_chain_url: String,
    pub request_timestamp: String,
    pub request_body_as_bytes: Vec<u8>,
}

impl AlexaAuthInput {
    pub fn new(
        signature: String,
        signature_cert_chain_url: String,
        request_timestamp: String,
        request_body_as_bytes: Vec<u8>,
    ) -> AlexaAuthInput {
        AlexaAuthInput {
            signature,
            signature_cert_chain_url,
            request_timestamp,
            request_body_as_bytes,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlexaAuthOutput {
    pub success: bool,
    pub message: String
}

impl AlexaAuthOutput {
    pub fn new(success: bool, message: String) -> AlexaAuthOutput {
        AlexaAuthOutput {
            success,
            message
        }
    }
}
