use actix_web::http::header::LOCATION;
use actix_web::{web, get};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GetRequest {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct GetResponse {
    pub url: String,
}


pub async fn url(web::Path((alias, key)): web::Path<(String, String)>) -> actix_web::HttpResponse {
    println!("{}, {}", alias, key);
    actix_web::HttpResponse::Found()
        .header(LOCATION, "https://www.google.com")
        .finish()
}
