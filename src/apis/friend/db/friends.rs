use crate::models::friend::{
    friend_detail::FriendDetail, friend_model::Friend, friend_state::FriendState,
};
use crate::MONGO_DB;
use futures::stream::StreamExt;
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::results::{InsertOneResult, UpdateResult};

pub async fn add_friend(
    new_friend_request: Friend,
) -> Result<InsertOneResult, mongodb::error::Error> {
    let database = MONGO_DB.get().unwrap();
    let collection = database.collection::<Friend>("friends");
    collection.insert_one(new_friend_request, None).await
}

pub async fn update_friend(
    friendship_id: String,
    state: FriendState,
) -> Result<UpdateResult, mongodb::error::Error> {
    let database = MONGO_DB.get().unwrap();
    let collection = database.collection::<Friend>("friends");
    let friendship_old = bson::oid::ObjectId::parse_str(friendship_id).unwrap();

    let query = doc! {"_id": friendship_old};
    let update = doc! {
        "$set":{
            "state": state.to_string()
        }
    };
    let result = collection.update_one(query, update, None).await;

    log::info!("Something here {:?}", result);

    return result;
}

// pub async fn get_friends(user_id: String) -> Result<Vec<Friend>, mongodb::error::Error> {
//     let database = MONGO_DB.get().unwrap();
//     let collection = database.collection::<Friend>("friends");
//     let filter = doc! {"user_id":bson::oid::ObjectId::parse_str(user_id).unwrap()};

//     let cursor = collection.find(filter, None).await?;
//     let results = cursor.try_collect().await.unwrap_or_else(|_| vec![]);
//     Ok(results)
// }

pub async fn get_friends(
    user_id: String,
    state: Option<FriendState>,
) -> Result<Vec<FriendDetail>, mongodb::error::Error> {
    let database = MONGO_DB.get().unwrap();
    let collection = database.collection::<Friend>("friends");
    let user_oid = bson::oid::ObjectId::parse_str(user_id).unwrap();

    let match_doc = match state {
        Some(search_state) => doc! {
           "$match": {
            "user_id": user_oid,
            "state": search_state.to_string()
           }
        },
        None => doc! {
           "$match": {
              "user_id": user_oid
           }
        },
    };

    let pipeline = vec![
        match_doc,
        doc! {
            "$lookup": doc! {
                "from": "users",
                "localField": "friend_id",
                "foreignField": "_id",
                "as": "friend_detail"
            }
        },
        doc! {
            "$unwind": doc! {
                "path": "$friend_detail",
                "preserveNullAndEmptyArrays": false
            }
        },
        doc! {
            "$addFields": doc! {
                "friend_name": "$friend_detail.username"
            }
        },
        doc! {
            "$project": doc! {
                "_id": doc! {
                    "$toString": "$_id"
                },
                "user_id": doc! {
                    "$toString": "$user_id"
                },
                "friend_id": doc! {
                    "$toString": "$friend_id"
                },
                "friend_name": 1,
                "state": 1
            }
        },
    ];

    let mut results: Vec<FriendDetail> = Vec::new();
    match collection.aggregate(pipeline, None).await {
        Ok(mut aggregate_results) => {
            while let Some(result) = aggregate_results.next().await {
                let doc: FriendDetail = bson::from_document(result.unwrap()).unwrap();
                results.push(doc);
            }
        }
        Err(_) => {}
    };
    Ok(results)
}
