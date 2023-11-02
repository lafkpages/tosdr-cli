use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Resp {
    pub error: u16,
    pub message: String,
    pub parameters: RespParameters,
}

#[derive(Deserialize, Debug)]
pub struct RespParameters {
    pub services: Vec<RespService>,
}

#[derive(Deserialize, Debug)]
pub struct RespService {
    pub id: u32,
    pub name: String,
    pub slug: String,
    pub is_comprehensively_reviewed: bool,
    pub urls: Vec<String>,
    pub wikipedia: String,
    pub rating: RespServiceRating,
    pub links: RespServiceLinks,
}

#[derive(Deserialize, Debug)]
pub struct RespServiceRating {
    pub hex: u8,
    pub human: String,
    pub letter: String,
}

#[derive(Deserialize, Debug)]
pub struct RespServiceLinks {
    pub crisp: RespServiceCrispLinks,
    pub phoenix: RespServicePhoenixLinks,
}

#[derive(Deserialize, Debug)]
pub struct RespServiceCrispLinks {
    pub api: String,
    pub service: String,
    pub badge: RespServiceBadge,
}

#[derive(Deserialize, Debug)]
pub struct RespServiceBadge {
    pub svg: String,
    pub png: String,
}

#[derive(Deserialize, Debug)]
pub struct RespServicePhoenixLinks {
    pub service: String,
    pub documents: String,
    pub new_comment: String,
    pub edit: String,
}
