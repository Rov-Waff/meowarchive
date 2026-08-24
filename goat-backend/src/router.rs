use std::sync::Arc;

use axum::Router;

use crate::{AppState, router::board::board_router};

mod board;

pub fn router()->Router<Arc<AppState>>{
    Router::new().nest("/board", board_router())
}