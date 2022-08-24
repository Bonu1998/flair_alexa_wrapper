use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceMap {
    pub alexa_wrappers: Vec<WrapperElement>,
    pub bussiness: Vec<BussinessElement>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WrapperElement {
    pub name: String,
    pub url: String,
    pub is_active: bool,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BussinessElement {
    pub name: String,
    pub url: String,
    pub is_active: bool,
    pub content_token: String,
    pub description: String,
}
