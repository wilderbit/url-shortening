use actix_web::web::Json;
use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    pub api_key: Option<String>,
    pub url: String,
    pub alias: Option<String>,
    pub expiry_time: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateRequestResponse {
    pub api_key: Option<String>,
    pub url: String,
    pub alias: Option<String>,
    pub expiry_time: Option<String>,
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
        expiry_time: create_req.expiry_time.clone(),
    };

    CreateRequestResponse::success(&t)
}
