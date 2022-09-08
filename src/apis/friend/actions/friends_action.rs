use crate::apis::friend::services::friends_service;
use crate::models::friend::friend_state::FriendState;
use actix_session::Session;
use actix_web::{get, post, put, web, HttpResponse};
use serde::Deserialize;

#[post("/{friend_id}")]
pub async fn add_friend(session: Session, path: web::Path<String>) -> HttpResponse {
    let user_id: Option<String> = session.get("user_id").unwrap_or(None);

    let friend_id = path.into_inner();

    match user_id {
        Some(current_user_id) => {
            match friends_service::add_friend(current_user_id, friend_id).await {
                Ok(_) => HttpResponse::Ok().json("Ok"),
                Err(_) => HttpResponse::Unauthorized().json("Unauthorized"),
            }
        }
        None => HttpResponse::Unauthorized().json("Unauthorized"),
    }
}

#[put("/{friendship_id}/{state}")]
pub async fn update_friend(
    session: Session,
    path: web::Path<(String, FriendState)>,
) -> HttpResponse {
    let user_id: Option<String> = session.get("user_id").unwrap_or(None);
    let (friendship_id, state) = path.into_inner();

    match user_id {
        Some(_) => match friends_service::update_friend(friendship_id, state).await {
            Ok(_) => HttpResponse::Ok().json("Ok"),
            Err(_) => HttpResponse::BadRequest().json("Unauthorized"),
        },
        None => HttpResponse::Unauthorized().json("Unauthorized"),
    }
}

#[derive(Deserialize)]
pub struct GetFriendsSearch {
    state: Option<FriendState>,
}
// ?state={Pending}
#[get("")]
pub async fn get_friends(
    session: Session,
    web::Query(search): web::Query<GetFriendsSearch>,
) -> HttpResponse {
    let user_id: Option<String> = session.get("user_id").unwrap_or(None);

    match user_id {
        Some(current_user_id) => {
            match friends_service::get_friends(current_user_id, search.state).await {
                Ok(friends) => HttpResponse::Ok().json(friends),
                Err(_) => HttpResponse::Unauthorized().json("Unauthorized"),
            }
        }
        None => HttpResponse::Unauthorized().json("Unauthorized"),
    }
}
