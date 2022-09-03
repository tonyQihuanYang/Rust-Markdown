use super::user::{GeneralUserInformation, UserId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Point {
    pub user_id: UserId,
    pub points: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PointWithUserInfo {
    pub points: i64,
    pub user_info: GeneralUserInformation,
}
