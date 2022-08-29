use crate::models::user::User;
use crate::MONGO_DB;
use mongodb::bson::doc;
use mongodb::results::InsertOneResult;

pub async fn find_user_by_id(id: String) -> Option<User> {
    let database = MONGO_DB.get().unwrap();
    let collection = database.collection::<User>("users");
    let filter = doc! {"_id":bson::oid::ObjectId::parse_str(id).unwrap()};
    match collection.find_one(filter, None).await {
        Ok(result) => match result {
            Some(user_found) => Some(user_found),
            None => None,
        },
        Err(_) => None,
    }
}

pub async fn find_user_by_username(username: String) -> Option<User> {
    let database = MONGO_DB.get().unwrap();
    let collection = database.collection::<User>("users");
    let filter = doc! {"username": username};
    match collection.find_one(filter, None).await {
        Ok(result) => match result {
            Some(user_found) => Some(user_found),
            None => None,
        },
        Err(_) => None,
    }
}

pub async fn find_user_by_credentials(username: String, hashed_password: String) -> Option<User> {
    let database = MONGO_DB.get().unwrap();
    let collection = database.collection::<User>("users");
    let filter = doc! {"username": username, "password": hashed_password};
    match collection.find_one(filter, None).await {
        Ok(result) => match result {
            Some(user_found) => Some(user_found),
            None => None,
        },
        Err(_) => None,
    }
}

pub async fn insert_user(new_user: User) -> Result<InsertOneResult, mongodb::error::Error> {
    let database = MONGO_DB.get().unwrap();
    let collection = database.collection::<User>("users");
    collection.insert_one(new_user, None).await
}
