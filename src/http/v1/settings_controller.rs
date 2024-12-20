use actix_web::{get, put, web, HttpResponse, Responder};
use charybdis::operations::{Find, Insert};
use charybdis::options::Consistency;
use log::info;
use serde::Deserialize;
use serde_json::json;
use web::Json;

use crate::config::app::AppState;
use crate::http::SomeError;
use crate::models::v1::settings::{Settings, SettingsByUsername};

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
  info!(
    "[PUT Settings] -> Channel/User -> {} / {}",
    settings.channel_id, settings.username
  );

  settings.insert().execute(&data.database).await?;

  Ok(HttpResponse::Ok().json(json!(settings)))
}

#[derive(Deserialize)]
struct SettingsQuery {
  channel_id: Option<String>,
}

#[get("/api/v1/settings/{username}")]
pub async fn get_settings(
  data: web::Data<AppState>,
  username: web::Path<String>,
  channel_id: web::Query<SettingsQuery>,
) -> Result<impl Responder, SomeError> {
  let username = username.into_inner();
  let channel_id = channel_id
    .into_inner()
    .channel_id
    .unwrap_or("global".to_string());

  info!(
    "[GET Settings] -> Channel/User -> {} / {}",
    channel_id, username
  );

  let mut settings = SettingsByUsername {
    username: username.clone(),
    enabled: true,
    channel_id,
    ..Default::default()
  };

  // Query the user settings with the given username and channel_id
  let settings_model = settings
    .find_by_partition_key()
    .consistency(Consistency::LocalOne)
    .execute(&data.database)
    .await?
    .try_collect()
    .await
    .unwrap();

  if !settings_model.is_empty() {
    return Ok(HttpResponse::Ok().json(json!(settings_model.first())));
  }

  settings.channel_id = "global".to_string();

  let settings_model = settings
    .find_by_partition_key()
    .consistency(Consistency::LocalOne)
    .execute(&data.database)
    .await?
    .try_collect()
    .await
    .unwrap();

  let result = match settings_model.is_empty() {
    true => HttpResponse::NotFound().finish(),
    false => HttpResponse::Ok().json(json!(settings_model.first())),
  };

  Ok(result)
}
