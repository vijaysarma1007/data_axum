use crate::{
    database::users::{self, Entity as Users},
    utils::{app_error::AppError, jwt::is_valid},
};
use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use headers::{Authorization, HeaderMapExt, authorization::Bearer};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn gaurd(mut request: Request, next: Next) -> Result<Response, AppError> {
    let token = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or_else(|| AppError::new(StatusCode::BAD_GATEWAY, "Missing Bearer Token"))?
        .token()
        .to_owned();

    let database = request
        .extensions()
        .get::<DatabaseConnection>()
        .ok_or_else(|| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error.")
        })?;

    let user = Users::find()
        .filter(users::Column::Token.eq(&token))
        .one(database)
        .await
        .map_err(|_error| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
        })?;

    //validating the token after the database query and user query will not give hacker a chance to guess due to timing attack

    is_valid(&token)?;

    let Some(user) = user else {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "You are not authorized, please log in or create account.",
        ));
    };

    request.extensions_mut().insert(user);
    Ok(next.run(request).await)
}
