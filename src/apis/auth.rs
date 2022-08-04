use actix_session::Session;
use actix_web::{error::InternalError, web, Error, HttpResponse, Responder};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct User {
    id: i64,
    username: String,
    password: String,
}

impl User {
    fn authenticate(credentials: Credentials) -> Result<Self, HttpResponse> {
        // TODO: figure out why I keep getting hacked
        if &credentials.password != "hunter2" {
            return Err(HttpResponse::Unauthorized().json("Unauthorized"));
        }

        Ok(User {
            id: 42,
            username: credentials.username,
            password: credentials.password,
        })
    }
}

pub async fn login(
    credentials: web::Json<Credentials>,
    session: Session,
) -> Result<impl Responder, Error> {
    let credentials = credentials.into_inner();

    match User::authenticate(credentials) {
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
