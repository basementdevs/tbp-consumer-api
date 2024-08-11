use charybdis::macros::{charybdis_model, charybdis_udt_model};
use charybdis::types::{Frozen, Int, Text, Timestamp};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[charybdis_udt_model(type_name = settingoptions)]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct SettingOptions {
  pub name: Text,
  pub slug: Text,
  pub translation_key: Text,
}

#[charybdis_model(
    table_name = settings,
    partition_keys = [user_id],
    clustering_keys = [updated_at],
    global_secondary_indexes = [],
    local_secondary_indexes = [],
    static_columns = [],
    table_options = "
     CLUSTERING ORDER BY (updated_at DESC)
    "
)]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Settings {
  pub user_id: Int,
  pub username: Option<Text>,
  pub locale: Option<Text>,
  pub timezone: Option<Text>,
  pub occupation: Frozen<SettingOptions>,
  pub pronouns: Frozen<SettingOptions>,
  #[serde(default = "default_updated_at")]
  pub updated_at: Timestamp,
}

pub fn default_updated_at() -> DateTime<Utc> {
  Utc::now()
}
