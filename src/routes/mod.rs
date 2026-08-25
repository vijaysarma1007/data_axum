mod create_task;
mod custom_json_extractor;
mod hello_world;

use axum::{
    Extension, Router,
    http::Method,
    middleware,
    routing::{get, post},
};

use create_task::create_task;
use custom_json_extractor::custom_json_extractor;
use hello_world::hello_world;
use sea_orm::DatabaseConnection;

pub fn create_routes(database: DatabaseConnection) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/custom_json_extractor", post(custom_json_extractor))
        .route("/tasks", post(create_task))
        .layer(Extension(database))
}
