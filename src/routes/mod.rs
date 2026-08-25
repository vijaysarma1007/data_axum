mod create_task;
mod custom_json_extractor;
mod get_one_task;
mod hello_world;

use axum::{
    Extension, Router,
    http::Method,
    middleware,
    routing::{get, post},
};

use create_task::create_task;
use custom_json_extractor::custom_json_extractor;
use get_one_task::get_one_task;
use hello_world::hello_world;
use sea_orm::DatabaseConnection;

pub fn create_routes(database: DatabaseConnection) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/custom_json_extractor", post(custom_json_extractor))
        .route("/tasks", post(create_task))
        .route("/tasks/{task_id}", get(get_one_task))
        .layer(Extension(database))
}
