use std::{env, sync::Arc};

use axum::{Router, extract::FromRef};
use dotenvy::var;
use sea_orm::{Database, DatabaseConnection};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

pub mod dtos;
pub mod entity;
pub mod router;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();
    let database = Database::connect(var("DATABASE_URL").expect("请提供数据库URL"))
        .await
        .expect("无法链接到数据库");
    let state = Arc::new(AppState { db: database });
    let app = Router::new()
        .merge(router())
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(state);
    let listener = TcpListener::bind(format!(
        "0.0.0.0:{}",
        env::var("SERVER_PORT").expect("No port provided")
    ))
    .await
    .unwrap();
    axum::serve(listener, app).await.unwrap();
}
