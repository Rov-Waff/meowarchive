use std::{env, sync::Arc};

use axum::{Router, extract::FromRef};
use dotenvy::var;
use sea_orm::{Database, DatabaseConnection};
use tokio::net::TcpListener;

use crate::router::router;

#[derive(Clone, Debug)]
pub struct AppState {
    db: DatabaseConnection,
}
impl FromRef<AppState> for DatabaseConnection {
    fn from_ref(input: &AppState) -> DatabaseConnection {
        input.db.clone()
    }
}

pub mod entity;
pub mod router;
pub mod dtos;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database = Database::connect(var("DATABASE_URL").expect("请提供数据库URL"))
        .await
        .expect("无法链接到数据库");
    let state = Arc::new(AppState { db: database });
    let app = Router::new().merge(router()).with_state(state);
    let listener = TcpListener::bind(format!(
        "0.0.0.0:{}",
        env::var("SERVER_PORT").expect("No port provided")
    ))
    .await
    .unwrap();
    axum::serve(listener, app).await.unwrap();
}
