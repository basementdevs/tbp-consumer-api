use crate::http::v1::auth_controller::AuthenticateDTO;
use charybdis::macros::charybdis_model;
use charybdis::types::{Int, Text};
use serde::{Deserialize, Serialize};

#[charybdis_model(
    table_name = user_tokens_v1,
    partition_keys = [access_token],
    clustering_keys = [],
    table_options = r"
        default_time_to_live = 604800
    "
)]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct UserToken {
  pub access_token: Text,
  pub user_id: Option<Int>,
}

impl UserToken {
  pub fn new(dto: AuthenticateDTO) -> Self {
    Self {
      access_token: dto.token,
      user_id: Some(dto.user_id),
    }
  }
}
