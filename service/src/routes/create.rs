use actix_web::web::Json;
use actix_web::{web, HttpResponse, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    pub api_key: Option<String>,
    pub url: String,
    pub alias: Option<String>,
    pub expiry_time: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct CreateRequestResponse {
    pub api_key: Option<String>,
    pub url: String,
    pub alias: Option<String>,
    pub expiry_time: Option<usize>,
}

impl CreateRequestResponse {
    pub fn success(data: &impl serde::Serialize) -> actix_web::HttpResponse {
        actix_web::HttpResponse::Ok().json(data)
    }
}

pub async fn url(create_req: Json<CreateRequest>) -> HttpResponse {
    let t = CreateRequestResponse {
        api_key: create_req.api_key.clone(),
        url: create_req.url.clone(),
        alias: create_req.alias.clone(),
        expiry_time: create_req.expiry_time,
    };
    crate::models::url::create(
        &crate::models::db::connection().unwrap(),
        "12345",
        &create_req.alias,
        &create_req.url,
        &Utc::now(),
    );
    CreateRequestResponse::success(&t)
}
