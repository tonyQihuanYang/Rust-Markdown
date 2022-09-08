use crate::{
    apis::friend::db::friends,
    models::friend::{
        friend_detail::FriendDetail, friend_model::Friend, friend_state::FriendState,
    },
};

pub async fn add_friend(current_user_id: String, friend_id: String) -> Result<(), ()> {
    let new_friend_request = Friend::new_friend_request(&current_user_id, &friend_id);

    match friends::add_friend(new_friend_request).await {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

pub async fn update_friend(friendship_id: String, state: FriendState) -> Result<(), ()> {
    match friends::update_friend(friendship_id, state).await {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

pub async fn get_friends(
    current_user_id: String,
    state: Option<FriendState>,
) -> Result<Vec<FriendDetail>, ()> {
    match friends::get_friends(current_user_id, state).await {
        Ok(friendships) => Ok(friendships),
        Err(_) => Err(()),
    }
}
