use super::friend_state::FriendState;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FriendDetail {
    pub _id: String,
    pub state: FriendState,
    pub user_id: String,
    pub friend_id: String,
    pub friend_name: String,
}
