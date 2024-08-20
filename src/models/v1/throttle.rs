use charybdis::macros::charybdis_model;
use charybdis::types::{Int, Text, Timestamp};
use scylla::query::Query;
use scylla::CachingSession;
use serde::{Deserialize, Serialize};

static INSERT_THROTTLE_WITH_TTL: &str =
  "INSERT INTO throttle_v1 (uri, user_id, content, updated_at) VALUES (?, ?, ?, ?) USING TTL ?";

#[charybdis_model(
    table_name = throttle_v1,
    partition_keys = [uri,user_id,content],
    clustering_keys = [updated_at],
    global_secondary_indexes = [],
    local_secondary_indexes = [],
    static_columns = [],
)]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Throttle {
  pub uri: Text,
  pub user_id: Int,
  pub content: Text,
  pub updated_at: Timestamp,
}

impl Throttle {
  pub async fn insert_throttle(&self, connection: &CachingSession, ttl: i32) -> anyhow::Result<()> {
    let query = Query::new(INSERT_THROTTLE_WITH_TTL);

    connection
      .execute(
        query,
        (
          &self.uri,
          &self.user_id,
          &self.content,
          &self.updated_at,
          ttl,
        ),
      )
      .await?;

    Ok(())
  }
}
