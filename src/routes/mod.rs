mod create_task;
mod custom_json_extractor;
mod delete_task;
mod gaurd;
mod get_task;
mod hello_world;
mod partial_update_task;
mod update_task;
mod users;

use axum::{
    Extension, Router, middleware,
    routing::{delete, get, patch, post, put},
};

use create_task::create_task;
use custom_json_extractor::custom_json_extractor;
use delete_task::delete_task;
use gaurd::gaurd;
use get_task::{get_all_tasks, get_one_task};
use hello_world::hello_world;
use partial_update_task::partial_update;
use sea_orm::DatabaseConnection;
use update_task::atomic_update;
use users::{create_user, login, logout};

pub fn create_routes(database: DatabaseConnection) -> Router {
    Router::new()
        .route("/users/logout", post(logout))
        .route_layer(middleware::from_fn(gaurd)) // routes above this layer must require authorization
        .route("/", get(hello_world))
        .route("/custom_json_extractor", post(custom_json_extractor))
        .route("/tasks", post(create_task))
        .route("/tasks", get(get_all_tasks))
        .route("/tasks/{task_id}", get(get_one_task))
        .route("/tasks/{task_id}", put(atomic_update))
        .route("/tasks/{task_id}", patch(partial_update))
        .route("/tasks/{task_id}", delete(delete_task))
        .route("/users", post(create_user))
        .route("/users/login", post(login))
        .layer(Extension(database))
}
