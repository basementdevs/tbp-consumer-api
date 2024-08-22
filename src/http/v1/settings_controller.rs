use actix_web::{get, put, web, HttpResponse, Responder};
use charybdis::operations::{Find, Insert};
use charybdis::options::Consistency;
use log::debug;
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
  let channel_id = channel_id.into_inner().channel_id.unwrap_or("global".to_string());

  debug!("username: {}, channel_id: {}", username, channel_id);

  let mut settings = SettingsByUsername {
    username: username.clone(),
    channel_id: channel_id.clone(),
    ..Default::default()
  };

  // Query the user settings with the given username and channel_id
  let mut settings_model = settings
    .find_by_partition_key()
    .consistency(Consistency::LocalOne)
    .execute(&data.database)
    .await?
      .try_collect()
      .await
      .unwrap();

  if channel_id == "global" {
    return Ok(HttpResponse::Ok().json(json!(settings_model.first())));
  }
  
  let mut settings_model = settings_model.pop();
  debug!("settings_model q tem: {:?}", settings_model);
  
  let should_query_global =  settings_model
      .clone()
      .is_some_and(|s| !s.enabled);
  
    let result = if should_query_global {
        let mut settings = SettingsByUsername {
            username,
            channel_id: "global".to_string(),
            ..Default::default()
        };
    
        let mut settings_model = settings
            .find_by_partition_key()
            .consistency(Consistency::LocalOne)
            .execute(&data.database)
            .await?
            .try_collect()
            .await
            .unwrap();
      debug!("settings_model q tem: {:?}", settings_model);
        settings_model.pop()
    } else {
        settings_model
    };
  
  let response = match result.is_some() {
    true => HttpResponse::Ok().json(json!(result.unwrap())),
    false => HttpResponse::NotFound().json(json!({})),
  };

  // Ok(HttpResponse::Ok())
  Ok(response)
}
