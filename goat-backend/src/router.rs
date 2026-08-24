use std::sync::Arc;

use axum::Router;

use crate::{
    AppState,
    router::{board::board_router, posts::posts_router},
};

mod board;
mod posts;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/board", board_router())
        .nest("/post", posts_router())
}
