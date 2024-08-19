use std::sync::Arc;

use actix_web::HttpRequest;
use charybdis::operations::Find;
use scylla::CachingSession;

use crate::models::v1::users::UserToken;

pub mod auth_controller;
pub mod metrics_controller;
pub mod settings_controller;

pub fn validate_token(req: HttpRequest, platform_secret: String) -> Option<String> {
  let authorization_token = req.headers().get("X-Authorization");

  if authorization_token.is_none() {
    return Some(String::from(
      "You must be logged in to update your settings",
    ));
  }

  let authorization_token = authorization_token.unwrap().to_str().unwrap();
  if authorization_token != platform_secret {
    return Some(String::from("Invalid token"));
  }
  None
}

pub async fn is_authenticated(
  session: &Arc<CachingSession>,
  req: HttpRequest,
) -> Option<UserToken> {
  let header = req.headers().get("Authorization");

  let header = header?.to_str();

  if header.is_err() {
    return None;
  }

  let response = UserToken {
    access_token: header.unwrap().to_string(),
    ..Default::default()
  }
  .maybe_find_by_primary_key()
  .execute(session)
  .await
  .unwrap();

  response.as_ref()?;
  Some(response.unwrap())
}
