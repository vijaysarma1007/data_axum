use std::{dbg, format, println};

use axum::{
    Json,
    extract::{FromRequest, Request},
    http::{self, StatusCode},
};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct RequestUser {
    #[validate(email(message = "must be a valid email."))]
    pub username: String,
    #[validate(length(min = 8, message = "must have at least 8 characters!"))]
    pub password: String,
}

impl<S> FromRequest<S> for RequestUser
where
    S: Sync + Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = Json::<RequestUser>::from_request(req, state)
            .await
            .map_err(|error| (StatusCode::BAD_REQUEST, format!("{}", error)))?;
        println!("cusomt");

        if let Err(erros) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", erros)));
        }

        Ok(user)
    }
}

pub async fn custom_json_extractor(user: RequestUser) {
    dbg!(user);
}
