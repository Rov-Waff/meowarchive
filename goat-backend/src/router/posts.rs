use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

use crate::{
    AppState,
    dtos::{Pagination, PostCommentDTO},
    entity,
};

async fn get_post_detail_handler(
    post_id: Path<u32>,
    state: State<Arc<AppState>>,
) -> Result<Json<crate::dtos::PostDetail>, (StatusCode, String)> {
    let db = state.db.clone();

    match entity::posts::Entity::find()
        .filter(entity::posts::Column::Id.eq(*post_id as u64))
        .find_also_related(entity::user::Entity)
        .one(&db)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
    {
        Some(res) => {
            let (post, user_opt) = res;
            match user_opt {
                Some(user) => Ok(Json(crate::dtos::PostDetail { post, user })),
                None => Err((StatusCode::NOT_FOUND, "Not Found".to_string())),
            }
        }
        None => Err((StatusCode::NOT_FOUND, "Not Found".to_string())),
    }
}

async fn get_post_replies(
    post_id: Path<u32>,
    state: State<Arc<AppState>>,
    pagination: Query<Pagination>,
) -> Result<Json<Vec<PostCommentDTO>>, (StatusCode, String)> {
    let size = pagination.size;
    let page = pagination.page;
    let db = state.db.clone();
    let data = entity::replies::Entity::find()
        .filter(entity::replies::Column::PostId.eq(*post_id as u64))
        .find_also_related(entity::user::Entity)
        .order_by_asc(entity::replies::Column::Id)
        .paginate(&db, size as u64)
        .fetch_page((page - 1) as u64)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    let mut items = vec![];
    for item in data {
        //TODO: N+1了，日后处理
        let (reply, user) = item;
        let user = match user {
            Some(r) => r,
            None => return Err((StatusCode::NOT_FOUND, "Not found".to_string())),
        };
        let comments = match entity::comments::Entity::find()
            .filter(entity::comments::Column::ReplyId.eq(reply.id))
            .all(&db)
            .await
        {
            Ok(r) => r,
            Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        };
        items.push(PostCommentDTO {
            reply: reply,
            user: user,
            comments: comments,
        });
    }
    Ok(Json(items))
}



pub fn posts_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/{post_id}", get(get_post_detail_handler))
        .route("/{post_id}/replies", get(get_post_replies))
}
