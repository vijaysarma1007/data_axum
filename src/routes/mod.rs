mod hello_world;
mod custom_json_extractor;
mod create_task;

use axum::{
    Extension, Router,
    http::Method,
    middleware,
    routing::{get, post},
};

use hello_world::hello_world;
use custom_json_extractor::custom_json_extractor;
use sea_orm::{DatabaseConnection};
use create_task::create_task;

pub fn create_routes(database: DatabaseConnection) -> Router {
    Router::new()
    .route("/", get(hello_world))
    .route("/custom_json_extractor", post(custom_json_extractor))
    .route("/tasks", post(create_task))
    .layer(Extension(database))
}
