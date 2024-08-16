use actix_web::{get, put, web, HttpResponse, Responder};
use charybdis::operations::{Find, Insert};
use serde_json::json;
use web::Json;

use crate::config::app::AppState;
use crate::http::SomeError;
use crate::models::v1::settings::Settings;
use crate::models::v1::settings_by_username::SettingsByUsername;

#[put("/api/v1/settings")]
pub async fn put_settings(
  data: web::Data<AppState>,
  message: Json<Settings>,
  req: actix_web::HttpRequest,
) -> anyhow::Result<impl Responder, SomeError> {
  let authorization_token = req.headers().get("X-Authorization");

  if authorization_token.is_none() {
    return Ok(HttpResponse::Unauthorized().json(json!({
      "error": "Unauthorized",
      "message": "You must be logged in to update your settings"
    })));
  }

  let authorization_token = authorization_token.unwrap().to_str().unwrap();
  if authorization_token != data.config.app.platform_secret {
    return Ok(HttpResponse::Unauthorized().json(json!({
      "error": "Unauthorized",
      "message": "Invalid token"
    })));
  }

  let settings = message.into_inner();

  settings.insert().execute(&data.database).await?;

  Ok(HttpResponse::Ok().json(json!(settings)))
}

#[get("/api/v1/settings/{username}")]
pub async fn get_settings(
  data: web::Data<AppState>,
  username: web::Path<String>,
) -> Result<impl Responder, SomeError> {
  let username = username.into_inner();

  let settings = SettingsByUsername {
    username,
    ..Default::default()
  };

  let settings = settings
    .find_by_partition_key()
    .execute(&data.database)
    .await?;

  let settings = settings.try_collect().await?;
  let response = match settings.is_empty() {
    true => HttpResponse::NotFound().json(json!({})),
    false => HttpResponse::Ok().json(json!(settings[0].clone())),
  };

  Ok(response)
}
