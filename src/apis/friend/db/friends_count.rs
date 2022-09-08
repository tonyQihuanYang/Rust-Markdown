use crate::models::friend::{friend_model::Friend, friend_state::FriendState};
use crate::MONGO_DB;
use mongodb::bson::doc;

pub async fn get_count(
    user_id: String,
    state: Option<FriendState>,
) -> Result<u64, mongodb::error::Error> {
    let database = MONGO_DB.get().unwrap();
    let collection = database.collection::<Friend>("friends");
    let user_oid = bson::oid::ObjectId::parse_str(user_id).unwrap();
    let filter = match state {
        Some(search_state) => {
            doc! {"user_id": user_oid, "state": search_state.to_string()}
        }
        None => doc! {"user_id": user_oid},
    };

    let count = collection.count_documents(filter, None).await?;
    Ok(count)
}
