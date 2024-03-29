use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

pub type UserId = bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: UserId,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new(username: String, password: String) -> Self {
        Self {
            id: bson::oid::ObjectId::new(),
            username,
            password,
        }
    }

    pub fn get_id_string(self) -> String {
        bson::oid::ObjectId::to_hex(self.id.clone())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneralUserInformation {
    pub _id: Option<bson::oid::ObjectId>,
    pub id: Option<String>,
    pub username: Option<String>,
}
