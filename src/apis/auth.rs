use crate::models::User::{LoginCredentials, User};
use crate::MONGO_DB;
use actix_session::Session;
use actix_web::{error::InternalError, web, Error, HttpResponse, Responder};
use mongodb::bson::{doc, Bson, Document};

pub async fn login(
    credentials: web::Json<LoginCredentials>,
    session: Session,
) -> Result<impl Responder, Error> {
    let credentials = credentials.into_inner();

    match User::authenticate(credentials).await {
        Ok(user) => session.insert("user_id", user.id).unwrap(),
        Err(err) => return Err(InternalError::from_response("", err).into()),
    };

    Ok("Welcome!")
}

pub async fn secret(session: Session) -> Result<impl Responder, Error> {
    log::info!("Comes here...");
    // only allow access to this resource if the user has an active session
    // validate_session(&session).map_err(|err| InternalError::from_response("", err))?;

    Ok("secret revealed")
}

pub fn validate_session(session: &Session) -> Result<i64, HttpResponse> {
    let user_id: Option<i64> = session.get("user_id").unwrap_or(None);

    match user_id {
        Some(id) => {
            log::info!("{}", id);
            // keep the user's session alive
            session.renew();
            Ok(id)
        }
        None => Err(HttpResponse::Unauthorized().json("Unauthorized")),
    }
}
