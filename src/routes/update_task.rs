use crate::database::tasks::{Column, Entity as Tasks};
use axum::http::StatusCode;
use axum::{Extension, Json, extract::Path};
use sea_orm::{
    ActiveValue::Set, ColumnTrait, DatabaseConnection, entity::prelude::DateTimeWithTimeZone,
};
use sea_orm::{EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::database::tasks;

#[derive(Deserialize)]
pub struct RequestTask {
    pub priority: Option<String>,
    pub title: String,
    pub completed_at: Option<DateTimeWithTimeZone>,
    pub description: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub user_id: Option<i32>,
    pub is_default: Option<bool>,
}

pub async fn atomic_update(
    Path(task_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(reuqest_task): Json<RequestTask>,
) -> Result<(), StatusCode> {
    let update_task = tasks::ActiveModel {
        id: Set(task_id),
        completed_at: Set(reuqest_task.completed_at),
        deleted_at: Set(reuqest_task.deleted_at),
        description: Set(reuqest_task.description),
        priority: Set(reuqest_task.priority),
        title: Set(reuqest_task.title),
        user_id: Set(reuqest_task.user_id),
        is_default: Set(reuqest_task.is_default),
    };

    Tasks::update(update_task)
        .filter(Column::Id.eq(task_id))
        .exec(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
