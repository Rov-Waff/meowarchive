use std::env;

use axum::{Router, routing::get};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let app = Router::new().route("/", get(||async{"Fuck you"}));
    let listener = TcpListener::bind(format!(
        "0.0.0.0:{}",
        env::var("SERVER_PORT").expect("No port provided")
    ))
    .await
    .unwrap();
    axum::serve(listener, app).await.unwrap();
}
