use crate::database::users::{self, Entity as Users};
use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use headers::{Authorization, HeaderMapExt, authorization::Bearer};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn gaurd(mut request: Request, next: Next) -> Result<Response, StatusCode> {
    let token = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or(StatusCode::BAD_REQUEST)?
        .token()
        .to_owned();

    let database = request
        .extensions()
        .get::<DatabaseConnection>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = Users::find()
        .filter(users::Column::Token.eq(token))
        .one(database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    let Some(user) = user else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    request.extensions_mut().insert(user);
    Ok(next.run(request).await)
}
