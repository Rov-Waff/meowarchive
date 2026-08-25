use std::sync::Arc;

use axum::Router;

use crate::{
    AppState,
    router::{board::board_router, posts::posts_router, user::user_router},
};

mod board;
mod posts;
mod user;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/board", board_router())
        .nest("/post", posts_router())
        .nest("/user", user_router())
}
