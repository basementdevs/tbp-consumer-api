use actix_web::HttpRequest;

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
