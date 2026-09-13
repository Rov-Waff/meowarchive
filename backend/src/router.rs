use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use sea_orm::{EntityTrait, PaginatorTrait};

use crate::{
    AppState,
    dtos::CountDTO,
    entity,
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

async fn count_handler(
    state: State<Arc<AppState>>,
) -> Result<Json<CountDTO>, (StatusCode, String)> {
    let db = state.db.clone();

    let post_count = entity::posts::Entity::find()
        .count(&db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DbErr".to_string()))?;
    let replies_count = entity::replies::Entity::find()
        .count(&db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DbErr".to_string()))?;
    let comment_count = entity::comments::Entity::find()
        .count(&db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DbErr".to_string()))?;
    let user_count = entity::user::Entity::find()
        .count(&db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DbErr".to_string()))?;
    Ok(Json(CountDTO {
        posts: post_count,
        replies: replies_count,
        comments: comment_count,
        user: user_count,
    }))
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/count",get(count_handler))
        .nest("/board", board_router())
        .nest("/post", posts_router())
        .nest("/user", user_router())
        .nest("/reply", reply_router())
        .nest("/comment", comment_router())
}
