use crate::models::user::{LoginCredentials, User};
use base64::encode;

use crate::apis::auth::db::users::{find_user_by_credentials, find_user_by_username, insert_user};

pub async fn login_user(credentials: LoginCredentials) -> Result<User, ()> {
    let hashed_password = encode(&credentials.password);
    match find_user_by_credentials(credentials.username, hashed_password).await {
        Some(user_found) => Ok(user_found),
        None => Err(()),
    }
}

pub async fn register_user(credentials: LoginCredentials) -> Result<(), ()> {
    let user_in_db = find_user_by_username(credentials.username.clone()).await;

    if user_in_db.is_some() {
        return Err(());
    }

    let hashed_password = encode(credentials.password);
    let new_user = User::new(credentials.username, hashed_password);

    match insert_user(new_user).await {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}
