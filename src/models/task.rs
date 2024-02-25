use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct TaskRow {
  pub id: i32,
  pub name: String,
  pub priority: Option<String>
}

#[derive(Deserialize)]
pub struct CreateTaskReq {
  pub name: String,
  pub priority: Option<String>,
}

#[derive(Serialize)]
pub struct CreateTaskRow {
  pub id: i32,
}

#[derive(Deserialize)]
pub struct UpdateTaskReq {
  pub name: Option<String>,
  pub priority: Option<String>,
}