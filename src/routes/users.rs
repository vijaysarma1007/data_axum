use crate::database::users::Entity as Users;
use crate::database::users::{self, Model};
use axum::{Extension, Json, http::StatusCode};
use axum_extra::TypedHeader;
use headers::{Authorization, authorization::Bearer};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter,
};
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

pub async fn login(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let db_user = Users::find()
        .filter(users::Column::Username.eq(&request_user.username))
        .one(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(db_user) = db_user {
        let new_token = "1233wd4f45f67yy78u9i0o".to_owned();
        let mut user = db_user.into_active_model();

        user.token = Set(Some(new_token));
        let saved_user = user
            .save(&database)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
        return Ok(Json(ResponseUser {
            username: saved_user.username.unwrap(),
            id: saved_user.id.unwrap(),
            token: saved_user.token.unwrap().unwrap(),
        }));
    } else {
        return Err(StatusCode::NOT_FOUND);
    }
}

pub async fn logout(
    Extension(database): Extension<DatabaseConnection>,
    Extension(user): Extension<Model>,
) -> Result<(), StatusCode> {
    let mut user = user.into_active_model();

    user.token = Set(None);
    user.save(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
