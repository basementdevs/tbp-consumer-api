use charybdis::macros::{charybdis_model, charybdis_udt_model, charybdis_view_model};
use charybdis::types::{Boolean, Frozen, Int, Text, Timestamp};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[charybdis_udt_model(type_name = settingoptions)]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct SettingOptions {
  pub name: Text,
  pub slug: Text,
  pub translation_key: Text,
}

#[charybdis_udt_model(type_name = coloroption)]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct ColorOption {
  pub name: Text,
  pub slug: Text,
  pub translation_key: Text,
  pub hex: Option<Text>,
}

#[charybdis_udt_model(type_name = effectoption)]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct EffectOption {
  pub name: Text,
  pub slug: Text,
  pub translation_key: Text,
  pub class_name: Text,
  pub hex: Option<Text>,
}

#[charybdis_model(
    table_name = settings_v1,
    partition_keys = [user_id, channel_id],
    clustering_keys = [],
    global_secondary_indexes = [],
    local_secondary_indexes = [],
    static_columns = [],
)]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Settings {
  pub user_id: Int,
  pub username: Text,
  pub channel_id: Text,
  pub enabled: Option<Boolean>,
  pub locale: Option<Text>,
  pub timezone: Option<Text>,
  pub occupation: Option<Frozen<SettingOptions>>,
  pub pronouns: Option<Frozen<SettingOptions>>,
  pub color: Option<Frozen<ColorOption>>,
  pub effect: Option<Frozen<EffectOption>>,
  pub is_developer: Option<Boolean>,
  #[serde(default = "default_updated_at")]
  pub updated_at: Timestamp,
}

#[charybdis_view_model(
    table_name = settings_by_username_v1,
    base_table = settings_v1,
    partition_keys = [username, channel_id],
    clustering_keys = [user_id],
)]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SettingsByUsername {
  pub user_id: Int,
  pub username: Text,
  pub channel_id: Text,
  pub enabled: Boolean,
  pub locale: Option<Text>,
  pub timezone: Option<Text>,
  pub occupation: Frozen<SettingOptions>,
  pub pronouns: Frozen<SettingOptions>,
  pub color: Frozen<ColorOption>,
  pub effect: Frozen<EffectOption>,
  pub is_developer: Option<Boolean>,
  pub updated_at: Timestamp,
}

pub fn default_updated_at() -> DateTime<Utc> {
  Utc::now()
}
