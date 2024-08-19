use crate::config::app::AppState;
use crate::http::v1::is_authenticated;
use crate::http::SomeError;
use crate::models::v1::metrics::{
  delete_user_most_watched_category_leaderboard, delete_user_most_watched_channels_leaderboard,
  UserMetrics, UserMetricsByCategory, UserMetricsByStream, UserMostWatchedCategoryLeaderboard,
  UserMostWatchedChannelsLeaderboard,
};
use actix_web::{get, post, web, HttpResponse, Responder};
use charybdis::operations::{Find, Insert};
use charybdis::types::Text;
use scylla::statement::Consistency;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Debug)]
struct UserMetricsDTO {
  pub channel_id: Text,
  pub category_id: Text,
}

#[get("/api/v1/metrics/by-user")]
pub async fn get_user_metrics(
  data: web::Data<AppState>,
  req: actix_web::HttpRequest,
) -> Result<impl Responder, SomeError> {
  let authenticated_user = is_authenticated(&data.database, req).await;

  if authenticated_user.is_none() {
    return Ok(HttpResponse::Unauthorized().finish());
  }

  let user_id = authenticated_user.unwrap().user_id.unwrap();
  let main_metrics = UserMetrics {
    user_id,
    ..Default::default()
  }
  .maybe_find_by_primary_key()
  .execute(&data.database)
  .await?;

  if main_metrics.is_none() {
    return Ok(HttpResponse::NotFound().json(json!({
        "error": "Not Found",
        "message": "User metrics not found"
    })));
  }

  let user_metrics_by_channel = UserMostWatchedCategoryLeaderboard {
    user_id,
    ..Default::default()
  };
  let user_metrics_by_category = UserMostWatchedChannelsLeaderboard {
    user_id,
    ..Default::default()
  };

  let user_metrics_by_channel = user_metrics_by_channel
    .find_by_partition_key()
    .consistency(Consistency::LocalOne)
    .page_size(5)
    .execute(&data.database)
    .await?
    .try_collect()
    .await
    .unwrap();

  let user_metrics_by_category = user_metrics_by_category
    .find_by_partition_key()
    .consistency(Consistency::LocalOne)
    .execute(&data.database)
    .await?
    .try_collect()
    .await
    .unwrap();

  Ok(HttpResponse::Ok().json(json!({
      "main_metrics": main_metrics,
      "user_metrics_by_channel": user_metrics_by_channel,
      "user_metrics_by_category": user_metrics_by_category,
  })))
}

#[post("/api/v1/metrics/heartbeat")]
pub async fn post_heartbeat(
  data: web::Data<AppState>,
  payload: web::Json<UserMetricsDTO>,
  req: actix_web::HttpRequest,
) -> Result<impl Responder, SomeError> {
  let payload = payload.into_inner();
  let is_authenticated = is_authenticated(&data.database, req).await;

  if is_authenticated.is_none() {
    return Ok(HttpResponse::Unauthorized().finish());
  }
  let user_id = is_authenticated.unwrap().user_id.unwrap();

  let main_metrics = UserMetrics {
    user_id,
    ..Default::default()
  };
  let user_metrics_by_channel = UserMetricsByStream {
    user_id,
    channel_id: payload.channel_id.clone(),
    ..Default::default()
  };
  let user_metrics_by_category = UserMetricsByCategory {
    user_id,
    category_id: payload.category_id.clone(),
    ..Default::default()
  };

  main_metrics
    .increment_minutes_watched(1)
    .consistency(Consistency::LocalOne)
    .execute(&data.database)
    .await?;
  user_metrics_by_channel
    .increment_minutes_watched(1)
    .consistency(Consistency::LocalOne)
    .execute(&data.database)
    .await?;
  user_metrics_by_category
    .increment_minutes_watched(1)
    .consistency(Consistency::LocalOne)
    .execute(&data.database)
    .await?;

  let user_metrics_by_category = user_metrics_by_category
    .find_by_primary_key()
    .consistency(Consistency::LocalOne)
    .execute(&data.database)
    .await?;
  let user_metrics_by_channel = user_metrics_by_channel
    .find_by_primary_key()
    .consistency(Consistency::LocalOne)
    .execute(&data.database)
    .await?;

  let current_minutes_by_category = user_metrics_by_category.minutes_watched.unwrap().0 as i32;
  let current_minutes_by_channel = user_metrics_by_channel.minutes_watched.unwrap().0 as i32;

  let user_category_leaderboard = UserMostWatchedCategoryLeaderboard {
    user_id,
    minutes_watched: current_minutes_by_category,
    category_id: payload.category_id.clone(),
  };

  let user_channels_leaderboard = UserMostWatchedChannelsLeaderboard {
    user_id,
    minutes_watched: current_minutes_by_channel,
    channel_id: payload.channel_id.clone(),
  };

  delete_user_most_watched_category_leaderboard!(
    "user_id = ? AND category_id = ? AND minutes_watched = ?",
    (
      user_id,
      payload.category_id.clone(),
      current_minutes_by_category - 1
    )
  )
  .execute(&data.database)
  .await?;

  delete_user_most_watched_channels_leaderboard!(
    "user_id = ? AND channel_id = ? AND minutes_watched = ?",
    (user_id, payload.channel_id, current_minutes_by_channel - 1)
  )
  .consistency(Consistency::LocalOne)
  .execute(&data.database)
  .await?;

  user_category_leaderboard
    .insert()
    .consistency(Consistency::LocalOne)
    .execute(&data.database)
    .await?;
  user_channels_leaderboard
    .insert()
    .consistency(Consistency::LocalOne)
    .execute(&data.database)
    .await?;

  Ok(HttpResponse::Ok().finish())
}
