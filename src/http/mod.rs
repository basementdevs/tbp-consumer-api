pub mod v0;
pub mod v1;

use actix_web::{get, HttpResponse, ResponseError};
use charybdis::errors::CharybdisError;
use serde_json::json;
use thiserror::Error;

#[get("/")]
pub async fn welcome() -> anyhow::Result<String, SomeError> {
  Ok("Welcome!".to_string())
}

#[derive(Error, Debug)]
pub enum SomeError {
  #[error("Charybdis error: {0}")]
  CharybdisError(#[from] CharybdisError),
}


impl ResponseError for SomeError {
  fn error_response(&self) -> HttpResponse {
    match self {
      SomeError::CharybdisError(e) => match e {
        CharybdisError::NotFoundError(e) => HttpResponse::NotFound().json(json!({
            "status": 404,
            "message": e.to_string()
        })),
        _ => HttpResponse::InternalServerError().json(json!({
            "status": 500,
            "message": e.to_string()
        })),
      },
    }
  }
}
