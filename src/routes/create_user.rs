use crate::database::users;
use axum::{Extension, Json, http::StatusCode};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RequestUser {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct ResponseUser {
    username: String,
    id: i32,
    token: String,
}

pub async fn create_user(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let new_user = users::ActiveModel {
        username: Set(request_user.username),
        password: Set(request_user.password),
        token: Set(Some("eyfldkfjsdlkfjsdlkfjlsdkjfsdf3wertj3tie".to_owned())),
        ..Default::default()
    }
    .save(&database)
    .await
    .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseUser {
        username: new_user.username.unwrap(),
        id: new_user.id.unwrap(),
        token: new_user.token.unwrap().unwrap(),
    }))
}
