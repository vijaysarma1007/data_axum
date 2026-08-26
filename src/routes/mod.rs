mod create_task;
mod custom_json_extractor;
mod get_task;
mod hello_world;
mod partial_update_task;
mod update_task;

use axum::{
    Extension, Router,
    routing::{get, patch, post, put},
};

use create_task::create_task;
use custom_json_extractor::custom_json_extractor;
use get_task::{get_all_tasks, get_one_task};
use hello_world::hello_world;
use partial_update_task::partial_update;
use sea_orm::DatabaseConnection;
use update_task::atomic_update;

pub fn create_routes(database: DatabaseConnection) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/custom_json_extractor", post(custom_json_extractor))
        .route("/tasks", post(create_task))
        .route("/tasks", get(get_all_tasks))
        .route("/tasks/{task_id}", get(get_one_task))
        .route("/tasks/{task_id}", put(atomic_update))
        .route("/tasks/{task_id}", patch(partial_update))
        .layer(Extension(database))
}
