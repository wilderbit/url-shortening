use actix_web::http::header::LOCATION;
use actix_web::web::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GetRequest {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct GetResponse {
    pub url: String,
}

pub async fn url() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Found()
        .header(LOCATION, "https://www.google.com")
        .finish()
}
