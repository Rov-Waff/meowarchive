use std::sync::Arc;

use axum::Router;

use crate::{
    AppState,
    router::{
        board::board_router, comment::comment_router, posts::posts_router, reply::reply_router,
        user::user_router,
    },
};

pub mod board;
pub mod comment;
pub mod posts;
pub mod reply;
pub mod user;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/board", board_router())
        .nest("/post", posts_router())
        .nest("/user", user_router())
        .nest("/reply", reply_router())
        .nest("/comment", comment_router())
}
