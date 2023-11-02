use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SearchApiResponse {
    pub error: u16,
    pub message: String,
    pub parameters: SearchApiResponseParameters,
}

#[derive(Deserialize, Debug)]
pub struct SearchApiResponseParameters {
    pub services: Vec<Service>,
}

#[derive(Deserialize, Debug)]
pub struct Service {
    pub id: u32,
    pub name: String,
    pub slug: String,
    pub is_comprehensively_reviewed: bool,
    pub urls: Vec<String>,
    pub wikipedia: String,
    pub rating: ServiceRating,
    pub links: ServiceLinks,
}

#[derive(Deserialize, Debug)]
pub struct ServiceRating {
    pub hex: u8,
    pub human: String,
    pub letter: String,
}

#[derive(Deserialize, Debug)]
pub struct ServiceLinks {
    pub crisp: ServiceCrispLinks,
    pub phoenix: ServicePhoenixLinks,
}

#[derive(Deserialize, Debug)]
pub struct ServiceCrispLinks {
    pub api: String,
    pub service: String,
    pub badge: ServiceBadge,
}

#[derive(Deserialize, Debug)]
pub struct ServiceBadge {
    pub svg: String,
    pub png: String,
}

#[derive(Deserialize, Debug)]
pub struct ServicePhoenixLinks {
    pub service: String,
    pub documents: String,
    pub new_comment: String,
    pub edit: String,
}
