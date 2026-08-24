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
    dtos::{PageResult, Pagination, PostCommentDTO},
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
) -> Result<Json<PageResult<PostCommentDTO>>, (StatusCode, String)> {
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
    let reply_ids: Vec<i64> = data.iter().map(|(reply, _)| reply.id).collect();
    let total_page = entity::replies::Entity::find()
        .filter(entity::replies::Column::PostId.eq(*post_id as u64))
        .count(&db)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
        .div_ceil(size as u64);
    let comments = if reply_ids.is_empty() {
        vec![]
    } else {
        entity::comments::Entity::find()
            .filter(entity::comments::Column::ReplyId.is_in(reply_ids))
            .order_by_asc(entity::comments::Column::Id)
            .all(&db)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
    };
    let mut comments_by_reply: std::collections::HashMap<i64, Vec<entity::comments::Model>> =
        std::collections::HashMap::new();
    for comment in comments {
        if let Some(reply_id) = comment.reply_id {
            comments_by_reply.entry(reply_id).or_default().push(comment);
        }
    }
    let mut items = vec![];
    for (reply, user) in data {
        let user = match user {
            Some(r) => r,
            None => return Err((StatusCode::NOT_FOUND, "Not found".to_string())),
        };
        let reply_id = reply.id;
        items.push(PostCommentDTO {
            reply,
            user,
            comments: comments_by_reply.remove(&reply_id).unwrap_or_default(),
        });
    }
    Ok(Json(PageResult {
        current: page,
        total: total_page as u32,
        has_next: page > 1,
        has_prev: page < total_page as u32,
        item: items,
    }))
}

pub fn posts_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/{post_id}", get(get_post_detail_handler))
        .route("/{post_id}/replies", get(get_post_replies))
}
