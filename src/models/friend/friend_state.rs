use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum FriendState {
    Pending,
    Friend,
    UnFriend,
    Declined,
}

impl fmt::Display for FriendState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FriendState::Pending => write!(f, "Pending"),
            FriendState::Friend => write!(f, "Friend"),
            FriendState::UnFriend => write!(f, "UnFriend"),
            FriendState::Declined => write!(f, "Declined"),
        }
    }
}
