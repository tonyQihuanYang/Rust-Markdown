use super::friend_state::FriendState;
use crate::models::user::UserId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Friend {
    pub user_id: UserId,
    pub friend_id: UserId,
    pub state: FriendState,
}

impl Friend {
    pub fn new_friend_request(user_id: &String, friend_id: &String) -> Friend {
        Friend {
            user_id: bson::oid::ObjectId::parse_str(user_id).unwrap(),
            friend_id: bson::oid::ObjectId::parse_str(friend_id).unwrap(),
            state: FriendState::Pending,
        }
    }
}
