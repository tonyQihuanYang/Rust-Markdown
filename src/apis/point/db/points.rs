use crate::models::point::{Point, PointWithUserInfo};
use crate::MONGO_DB;
use futures::stream::StreamExt;
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
pub async fn get_points_with_user_info(user_id: String) -> Option<Point> {
    let database = MONGO_DB.get().unwrap();
    let collection = database.collection::<Point>("points");

    let pipeline = vec![
        doc! {
           "$match": {
              "user_id": bson::oid::ObjectId::parse_str(user_id).unwrap()
           }
        },
        doc! {
            "$lookup": {
                  "from": "users",
                  "localField": "user_id",
                  "foreignField": "_id",
                  "as": "user_info",
            }
        },
        doc! {
            "$unwind": {
                "path": "$user_info",
                "preserveNullAndEmptyArrays": false
           }
        },
        doc! {
            "$project": {
                "_id": 0,
                "points": 1,
                "user_info": {"_id": 1, "username": 1}
            }
        },
    ];

    match collection.aggregate(pipeline, None).await {
        Ok(mut results) => {
            while let Some(result) = results.next().await {
                let doc: PointWithUserInfo = bson::from_document(result.unwrap()).unwrap();
                println!("* {:?}", doc);
            }
        }
        Err(_) => {}
    };
    // while let Some(result) = results.next().await {
    // let doc: MovieSummary = bson::from_document(result?)?;
    // println!("* {}, comments={:?}", doc, doc.comments);
    // }

    None

    //     let filter = doc! {"user_id":bson::oid::ObjectId::parse_str(user_id).unwrap()};

    //     match collection.find_one(filter, None).await {
    //         Ok(points) => points,
    //         Err(_) => None,
    // }
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
