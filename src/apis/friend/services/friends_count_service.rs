use crate::apis::friend::db::friends_count;
use crate::models::friend::friend_state::FriendState;

pub async fn get_count(current_user_id: String, state: Option<FriendState>) -> Result<u64, ()> {
    match friends_count::get_count(current_user_id, state).await {
        Ok(friendships) => Ok(friendships),
        Err(_) => Err(()),
    }
}
