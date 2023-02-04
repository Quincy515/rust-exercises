use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Extension, Json,
};
use chrono::{DateTime, FixedOffset};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::databases::{prelude::*, tasks};

#[derive(Serialize)]
pub struct ResponseTask {
    id: i32,
    title: String,
    priority: Option<String>,
    description: Option<String>,
    deleted_at: Option<DateTime<FixedOffset>>,
    user_id: Option<i32>,
}

pub async fn get_one_task(
    Path(task_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<ResponseTask>, StatusCode> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::DeletedAt.is_null())
        .one(&database)
        .await
        .unwrap();

    if let Some(task) = task {
        return Ok(Json(ResponseTask {
            id: task.id,
            title: task.title,
            priority: task.priority,
            description: task.description,
            deleted_at: task.deleted_at,
            user_id: task.user_id,
        }));
    } else {
        return Err(StatusCode::NOT_FOUND);
    };
}

#[derive(Deserialize)]
pub struct GetTasksQueryParams {
    priority: Option<String>,
}

pub async fn get_all_tasks(
    Query(query_params): Query<GetTasksQueryParams>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
    let mut priority_filter = Condition::all();

    if let Some(priority) = query_params.priority {
        priority_filter = if priority.is_empty() {
            priority_filter.add(tasks::Column::Priority.is_null())
        } else {
            priority_filter.add(tasks::Column::Priority.eq(priority))
        };
    }

    let tasks = Tasks::find()
        .filter(tasks::Column::DeletedAt.is_null())
        .filter(priority_filter)
        .all(&database)
        .await
        .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_task| ResponseTask {
            id: db_task.id,
            title: db_task.title,
            priority: db_task.priority,
            description: db_task.description,
            deleted_at: db_task.deleted_at,
            user_id: db_task.user_id,
        })
        .collect();
    Ok(Json(tasks))
}
