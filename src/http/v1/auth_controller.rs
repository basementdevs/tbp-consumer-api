use crate::config::app::AppState;
use crate::http::v1::validate_token;
use crate::http::SomeError;
use crate::models::v1::users::UserToken;
use actix_web::{post, web, HttpResponse, Responder};
use charybdis::operations::Insert;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthenticateDTO {
  pub user_id: i32,
  pub token: String,
}
#[post("/api/v1/authenticate")]
pub async fn post_user_authentication(
  data: web::Data<AppState>,
  payload: web::Json<AuthenticateDTO>,
  req: actix_web::HttpRequest,
) -> Result<impl Responder, SomeError> {
  if let Some(value) = validate_token(req, data.config.app.platform_secret.clone()) {
    return Ok(HttpResponse::Unauthorized().json(json!({
      "error": "Unauthorized",
      "message": value
    })));
  }

  UserToken::new(payload.into_inner())
    .insert()
    .execute(&data.database)
    .await?;

  Ok(HttpResponse::Created().finish())
}
