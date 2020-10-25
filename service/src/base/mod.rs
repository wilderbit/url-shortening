use chrono::Utc;
pub use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ApiResponse {
    pub data: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
    pub success: bool,
    pub timestamp: chrono::DateTime<Utc>,
}

impl Default for ApiResponse {
    fn default() -> Self {
        ApiResponse {
            data: None,
            error: None,
            success: false,
            timestamp: Utc::now(),
        }
    }
}

impl ApiResponse {
    fn with_data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self.success = true;
        return self;
    }

    fn with_error(mut self, error: serde_json::Value) -> Self {
        self.data = Some(error);
        self.success = false;
        self
    }

    fn from_success(data: &impl serde::Serialize) -> Self {
        ApiResponse::default().with_data(json!(data))
    }

    fn from_error(data: &impl serde::Serialize) -> Self {
        ApiResponse::default().with_error(json!(data))
    }

    fn success(data: &impl serde::Serialize) -> actix_web::HttpResponse {
        actix_web::HttpResponse::Ok().json(Self::from_success(data))
    }

    fn error(error: &impl serde::Serialize) -> actix_web::HttpResponse {
        actix_web::HttpResponse::Ok().json(Self::from_error(error))
    }
}
