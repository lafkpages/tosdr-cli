use serde::Deserialize;

pub type SearchApiResponse = ApiResponse<SearchApiResponseParameters>;
pub type ServiceApiResponse = ApiResponse<Service<u8>>;

#[derive(Deserialize, Debug)]
pub struct ApiResponse<Parameters> {
    pub error: u16,
    pub message: String,
    pub parameters: Parameters,
}

#[derive(Deserialize, Debug)]
pub struct SearchApiResponseParameters {
    pub services: Vec<Service>,
}

#[derive(Deserialize, Debug)]
pub struct Service<Rating: ServiceRatingString = ServiceRating> {
    pub id: u32,
    pub name: String,
    pub slug: String,
    pub is_comprehensively_reviewed: bool,
    pub urls: Vec<String>,
    pub wikipedia: String,
    pub rating: Rating,
    pub links: Option<ServiceLinks>,
    pub points: Option<Vec<ServicePoint>>,
}

pub trait ServiceRatingString {
    fn to_rating_string(self) -> String;
}

impl ServiceRatingString for ServiceRating {
    fn to_rating_string(self) -> String {
        self.human
    }
}

impl ServiceRatingString for u8 {
    fn to_rating_string(self) -> String {
        self.to_string()
    }
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

#[derive(Deserialize, Debug)]
pub struct ServicePoint {
    pub id: u32,
    pub title: String,
    pub source: Option<String>,
    pub status: String, // TODO: 'declined' | 'approved' | 'pending'?
    pub case_id: u32,
}
