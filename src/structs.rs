use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Resp {
    error: u16,
    message: String,
    parameters: RespParameters,
}

#[derive(Deserialize, Debug)]
pub struct RespParameters {
    services: Vec<RespService>,
}

#[derive(Deserialize, Debug)]
pub struct RespService {
    id: u32,
    name: String,
    slug: String,
    is_comprehensively_reviewed: bool,
    urls: Vec<String>,
    wikipedia: String,
}
