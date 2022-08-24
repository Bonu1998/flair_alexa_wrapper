use flair_alexa_sdk::request::Request as AlexaRequest;

pub fn get_device_size(input: &AlexaRequest) -> String {
    let mut device_size = String::from("horizontal-large");
    if let Some(viewport) = input.context.viewport.clone() {
        let width = viewport.pixel_width.unwrap_or_default();
        let height = viewport.pixel_height.unwrap_or_default();
        let shape = viewport.shape.unwrap_or_default();
        let orientation: String = if shape.as_str() == "ROUND" {
            String::from("round")
        } else if width < height {
            String::from("vertical")
        } else {
            String::from("horizontal")
        };
        let size;
        if width < 600 {
            size = String::from("extra-small")
        } else if width >= 600 && width < 960 {
            size = String::from("small")
        } else if width >= 960 && width < 1280 {
            size = String::from("medium")
        } else if width >= 1280 && width < 1920 {
            size = String::from("large")
        } else if width >= 1920 {
            size = String::from("extra-large")
        } else {
            size = String::from("large")
        }
        device_size = format!("{}-{}", orientation, size);
    }
    device_size
}
