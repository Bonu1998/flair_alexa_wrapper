use log::{error, info, debug};

use self::io::{AlexaAuthInput, AlexaAuthOutput};

pub mod io;

pub fn verifier(input: AlexaAuthInput) -> AlexaAuthOutput {
    info!("verifier invoked");
    debug!("\ninput: {:?}", input);
    let verify = alexa_verifier::RequestVerifier::new();
    let output:AlexaAuthOutput;
    match verify.verify(
        &input.signature_cert_chain_url,
        &input.signature,
        &input.request_body_as_bytes,
        &input.request_timestamp,
        None,
    ) {
        Ok(()) => {
            output= AlexaAuthOutput::new(true, String::from("Authentication success"));
        }
        Err(err) => {
            error!("verification failed");
            output = AlexaAuthOutput::new(true, format!("Authentication Failed: {}", err));
        }
    };
    debug!("\noutput: {:?}", output);
    output
}
