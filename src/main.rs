use axum::{
  routing::{ get, patch },
  Router
};

use sqlx::postgres::PgPoolOptions;

use tokio::net::TcpListener;

mod models {
  pub mod task;
}

mod controller {
  pub mod task;
}

#[tokio::main]
async fn main() {
  dotenvy::dotenv().expect("Unable to access environment file");

//    set environment variables
  let server_address: String = std::env::var("SERVER_URL").unwrap_or("localhost:3333".to_string()).to_owned();
  let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in the environment file");

//   database connection poll
  let database_poll = PgPoolOptions::new()
      .max_connections(5)
      .connect(&database_url)
      .await.expect("Can't connect to database");

//   tcp listen
  let listener = TcpListener::bind(&server_address)
      .await.expect("Could not create tcp listener");

  println!("Listen on {}", listener.local_addr().unwrap());

  let app = Router::new()
      .route("/", get(|| async {"Hello World"}))
      .route("/tasks", get(controller::task::get_tasks).post(controller::task::create_task))
      .route("/task/:id", patch(controller::task::update_task).delete(controller::task::delete_task))
      .with_state(database_poll);

  axum::serve(listener, app)
      .await
      .expect("Error serving app");

}