use actix_web::{post, web, HttpResponse, Responder};
use charybdis::operations::{Find, Insert};
use charybdis::options::Consistency::One;
use charybdis::types::Text;
use serde::{Deserialize, Serialize};

use crate::config::app::AppState;
use crate::http::SomeError;
use crate::models::v1::metrics::{
  delete_user_most_watched_category_leaderboard, delete_user_most_watched_channels_leaderboard,
  UserMetrics, UserMetricsByCategory, UserMetricsByStream, UserMostWatchedCategoryLeaderboard,
  UserMostWatchedChannelsLeaderboard,
};
use crate::models::v1::users::UserToken;

#[derive(Deserialize, Serialize, Debug)]
struct UserMetricsDTO {
  pub user_id: i32,
  pub channel_id: Text,
  pub category_id: Text,
}

#[post("/api/v1/heartbeat")]
pub async fn post_heartbeat(
  data: web::Data<AppState>,
  payload: web::Json<UserMetricsDTO>,
  req: actix_web::HttpRequest,
) -> Result<impl Responder, SomeError> {
  let header = req.headers().get("Authorization");

  if header.is_none() {
    return Ok(HttpResponse::Unauthorized().finish());
  }

  let header = header.unwrap().to_str();

  if header.is_err() {
    return Ok(HttpResponse::Unauthorized().finish());
  }
  let payload = payload.into_inner();

  let response = UserToken {
    user_id: payload.user_id,
    access_token: header.unwrap().to_string(),
  }
  .maybe_find_by_primary_key()
  .execute(&data.database)
  .await?;

  if response.is_none() {
    return Ok(HttpResponse::Unauthorized().finish());
  }

  let main_metrics = UserMetrics {
    user_id: payload.user_id,
    ..Default::default()
  };
  let user_metrics_by_channel = UserMetricsByStream {
    user_id: payload.user_id,
    channel_id: payload.channel_id.clone(),
    ..Default::default()
  };
  let user_metrics_by_category = UserMetricsByCategory {
    user_id: payload.user_id,
    category_id: payload.category_id.clone(),
    ..Default::default()
  };

  main_metrics
    .increment_watch_time_in_minutes(1)
    .consistency(One)
    .execute(&data.database)
    .await?;
  user_metrics_by_channel
    .increment_minutes_watched(1)
    .consistency(One)
    .execute(&data.database)
    .await?;
  user_metrics_by_category
    .increment_minutes_watched(1)
    .consistency(One)
    .execute(&data.database)
    .await?;

  let user_metrics_by_category = user_metrics_by_category
    .find_by_primary_key()
    .execute(&data.database)
    .await?;
  let user_metrics_by_channel = user_metrics_by_channel
    .find_by_primary_key()
    .execute(&data.database)
    .await?;

  let current_minutes_by_category = user_metrics_by_category.minutes_watched.unwrap().0 as i32;
  let current_minutes_by_channel = user_metrics_by_channel.minutes_watched.unwrap().0 as i32;

  let user_category_leaderboard = UserMostWatchedCategoryLeaderboard {
    user_id: payload.user_id,
    minutes_watched: current_minutes_by_category,
    category_id: payload.category_id.clone(),
  };

  let user_channels_leaderboard = UserMostWatchedChannelsLeaderboard {
    user_id: payload.user_id,
    minutes_watched: current_minutes_by_channel,
    channel_id: payload.channel_id.clone(),
  };

  delete_user_most_watched_category_leaderboard!(
    "user_id = ? AND category_id = ? AND minutes_watched = ?",
    (
      payload.user_id,
      payload.category_id.clone(),
      current_minutes_by_category - 1
    )
  )
  .execute(&data.database)
  .await?;

  delete_user_most_watched_channels_leaderboard!(
    "user_id = ? AND channel_id = ? AND minutes_watched = ?",
    (
      payload.user_id,
      payload.channel_id,
      current_minutes_by_channel - 1
    )
  )
  .execute(&data.database)
  .await?;

  user_category_leaderboard
    .insert()
    .execute(&data.database)
    .await?;
  user_channels_leaderboard
    .insert()
    .execute(&data.database)
    .await?;

  Ok(HttpResponse::Ok().finish())
}
