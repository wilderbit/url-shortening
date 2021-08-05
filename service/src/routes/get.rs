use actix_web::http::header::LOCATION;
use actix_web::{get, web};
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
pub struct GetRequest {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct GetResponse {
    pub url: String,
}
// web::Path((alias, key)): web::Path<(String, String)>

pub async fn url() -> actix_web::HttpResponse {
    // println!("{}, {}", alias, key);
    actix_web::HttpResponse::Found()
        .header(LOCATION, "https://www.google.com")
        .status(actix_web::http::StatusCode::PERMANENT_REDIRECT)
        .finish()
}
