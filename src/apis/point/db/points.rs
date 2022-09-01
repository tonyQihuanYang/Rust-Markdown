use crate::models::point::Point;
use crate::MONGO_DB;
use mongodb::bson::doc;
use mongodb::options::FindOneAndUpdateOptions;

pub async fn get_points(user_id: String) -> Option<Point> {
    let database = MONGO_DB.get().unwrap();
    let collection = database.collection::<Point>("points");
    let filter = doc! {"user_id":bson::oid::ObjectId::parse_str(user_id).unwrap()};

    match collection.find_one(filter, None).await {
        Ok(points) => points,
        Err(_) => None,
    }
}

pub async fn add_point(user_id: String, points: i64) -> Option<Point> {
    let database = MONGO_DB.get().unwrap();
    let collection = database.collection::<Point>("points");
    let filter = doc! {"user_id":bson::oid::ObjectId::parse_str(user_id).unwrap()};

    let options = FindOneAndUpdateOptions::builder()
        .upsert(Some(true))
        .build();

    match collection
        .find_one_and_update(filter, doc! {"$inc": {"points": points}}, options)
        .await
    {
        Ok(result) => match result {
            Some(user_found) => Some(user_found),
            None => None,
        },
        Err(_) => None,
    }
}

pub async fn remove_point(user_id: String, points: i64) -> Option<Point> {
    add_point(user_id, -1 * points).await
}
