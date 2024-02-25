use axum::{
  extract::{Path, State},
  Json,
  http::StatusCode
};
use serde_json::json;
use sqlx::PgPool;

use crate::models::task;

pub async fn get_tasks(
  State(db_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
  let rows = sqlx::query_as!(task::TaskRow, "SELECT * FROM tasks ORDER BY id")
      .fetch_all(&db_pool)
      .await
      .map_err(|e| {
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          json!({"success": false, "message": e.to_string()}).to_string(),
        )
      })?;

  Ok((
    StatusCode::OK,
    json!({"success": true, "data": rows}).to_string(),
  ))
}


pub async fn create_task(
  State(db_pool): State<PgPool>,
  Json(task): Json<task::CreateTaskReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
  let row = sqlx::query_as!(
    task::CreateTaskRow,
    "INSERT INTO tasks (name, priority) VALUES ($1, $2) RETURNING id",
    task.name,
    task.priority
  )
      .fetch_one(&db_pool)
      .await
      .map_err(|e| {
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          json!({"success": false, "message": e.to_string()}).to_string(),
        )
      })?;

  Ok((
    StatusCode::CREATED,
    json!({"success": true, "data": row}).to_string(),
  ))
}

pub async fn update_task(
  State(db_pool): State<PgPool>,
  Path(id): Path<i32>,
  Json(task): Json<task::UpdateTaskReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
  let mut query = "UPDATE tasks SET id = $1".to_owned();

  let mut i = 2;

  if task.name.is_some() {
    query.push_str(&format!(", name = ${i}"));
    i = i + 1;
  };

  if task.priority.is_some() {
    query.push_str(&format!(", priority = ${i}"));
  };

  query.push_str(&" WHERE id = $1".to_string());

  let mut s = sqlx::query(&query).bind(id);

  if task.name.is_some() {
    s = s.bind(task.name);
  }

  if task.priority.is_some() {
    s = s.bind(task.priority);
  }

  s.execute(&db_pool).await.map_err(|e| {
    (
      StatusCode::INTERNAL_SERVER_ERROR,
      json!({"success": false, "message": e.to_string()}).to_string(),
    )
  })?;

  Ok((StatusCode::OK, json!({"success":true}).to_string()))
}

pub async fn delete_task(
  State(db_pool): State<PgPool>,
  Path(id): Path<i32>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
  sqlx::query!("DELETE FROM tasks WHERE id = $1", id,)
      .execute(&db_pool)
      .await
      .map_err(|e| {
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          json!({"success": false, "message": e.to_string()}).to_string(),
        )
      })?;

  Ok((StatusCode::OK, json!({"success":true}).to_string()))
}