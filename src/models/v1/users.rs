use crate::http::v1::auth_controller::AuthenticateDTO;
use charybdis::macros::charybdis_model;
use charybdis::types::{Int, Text};
use serde::{Deserialize, Serialize};

#[charybdis_model(
    table_name = user_tokens_v1,
    partition_keys = [user_id],
    clustering_keys = [access_token],
    table_options = r"
        default_time_to_live = 604800
    "
)]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct UserToken {
  pub user_id: Int,
  pub access_token: Text,
}

impl UserToken {
  pub fn new(dto: AuthenticateDTO) -> Self {
    Self {
      user_id: dto.user_id,
      access_token: dto.token,
    }
  }
}
