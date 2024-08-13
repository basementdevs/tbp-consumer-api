use charybdis::macros::charybdis_view_model;
use charybdis::types::{Frozen, Int, Text, Timestamp};
use serde::{Deserialize, Serialize};
use crate::models::v1::settings::SettingOptions;

#[charybdis_view_model(
    table_name = settings_by_username_v1,
    base_table = settings_v1,
    partition_keys = [username],
    clustering_keys = [user_id],
)]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SettingsByUsername {
    pub user_id: Int,
    pub username: Text,
    pub locale: Option<Text>,
    pub timezone: Option<Text>,
    pub occupation: Frozen<SettingOptions>,
    pub pronouns: Frozen<SettingOptions>,
    pub updated_at: Timestamp,
}
