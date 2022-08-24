use log::{error, info};

use self::io::{AlexaAuthInput, AlexaAuthOutput};

pub mod io;

pub fn verifier(input: AlexaAuthInput) -> AlexaAuthOutput {
    info!("verifier invoked");
    let verify = alexa_verifier::RequestVerifier::new();
    match verify.verify(
        &input.signature_cert_chain_url,
        &input.signature,
        &input.request_body_as_bytes,
        &input.request_timestamp,
        None,
    ) {
        Ok(()) => {
            return AlexaAuthOutput::new(true, String::from("Authentication success"));
        }
        Err(err) => {
            error!("verification failed");
            return AlexaAuthOutput::new(true, format!("Authentication Failed: {}", err));
        }
    }
}
