use crate::{apis::point::db::points, models::point::Point};

pub async fn get_points(user_id: String) -> Result<Point, ()> {
    match points::get_points(user_id).await {
        Some(points) => Ok(points),
        None => Err(()),
    }
}
