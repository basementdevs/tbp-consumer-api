use charybdis::macros::charybdis_model;
use charybdis::types::{Counter, Int, Text};
use serde::{Deserialize, Serialize};

#[charybdis_model(
    table_name = user_metrics_v1,
    partition_keys = [user_id],
    clustering_keys = [],
    global_secondary_indexes = [],
    local_secondary_indexes = [],
    static_columns = [],
)]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct UserMetrics {
  pub user_id: Int,
  pub minutes_watched: Option<Counter>,
  pub messages_count: Option<Counter>,
}

#[charybdis_model(
    table_name = user_metrics_by_channel_v1,
    partition_keys = [user_id, channel_id],
    clustering_keys = [],
    global_secondary_indexes = [],
    local_secondary_indexes = [],
    static_columns = [],
)]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct UserMetricsByStream {
  pub user_id: Int,
  pub channel_id: Text,
  pub minutes_watched: Option<Counter>,
  pub messages_count: Option<Counter>,
}

#[charybdis_model(
    table_name = user_metrics_by_category_v1,
    partition_keys = [user_id, category_id],
    clustering_keys = [],
    global_secondary_indexes = [],
    local_secondary_indexes = [],
    static_columns = [],
)]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct UserMetricsByCategory {
  pub user_id: Int,
  pub category_id: Text,
  pub minutes_watched: Option<Counter>,
  pub messages_count: Option<Counter>,
}

#[charybdis_model(
    table_name = user_most_watched_category_leaderboard_v1,
    partition_keys = [user_id],
    clustering_keys = [minutes_watched, category_id],
    global_secondary_indexes = [],
    local_secondary_indexes = [],
    static_columns = [],
    table_options = "
     CLUSTERING ORDER BY (minutes_watched DESC, category_id ASC)
    "
)]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct UserMostWatchedCategoryLeaderboard {
  pub user_id: Int,
  pub category_id: Text,
  pub minutes_watched: Int,
}

#[charybdis_model(
    table_name = user_most_watched_channels_leaderboard_v1,
    partition_keys = [user_id],
    clustering_keys = [minutes_watched, channel_id],
    global_secondary_indexes = [],
    local_secondary_indexes = [],
    static_columns = [],
    table_options = "
     CLUSTERING ORDER BY (minutes_watched DESC, channel_id ASC)
    "
)]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct UserMostWatchedChannelsLeaderboard {
  pub user_id: Int,
  pub channel_id: Text,
  pub minutes_watched: Int,
}
