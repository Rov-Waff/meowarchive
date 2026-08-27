use std::collections::HashMap;
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
    dtos::{
        Pagination, UserCommentDTO, UserCommentPage, UserPage, UserPostDTO, UserPostPage,
        UserReplyDTO, UserReplyPage,
    },
    entity,
};

/// 分页获取全部用户
#[utoipa::path(
    get,
    path = "/user",
    tag = "user",
    responses(
        (status = 200, description = "分页用户列表", body = UserPage),
    )
)]
#[axum::debug_handler]
async fn get_all_user_handler(
    pagination: Query<Pagination>,
    state: State<Arc<AppState>>,
) -> Result<Json<UserPage>, (StatusCode, String)> {
    let db = state.db.clone();
    let page = pagination.page;
    let size = pagination.size;
    if size > 30 {
        return Err((StatusCode::BAD_REQUEST, "Too big".to_string()));
    } else {
        let data = match entity::user::Entity::find()
            .order_by_id_asc()
            .paginate(&db, size as u64)
            .fetch_page((page - 1) as u64)
            .await
        {
            Ok(res) => res,
            Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "DbERR".to_string())),
        };
        let total_page = entity::user::Entity::find()
            .count(&db)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
            .div_ceil(size as u64) as u32;
        Ok(Json(UserPage {
            current: page,
            total: total_page,
            has_next: page < total_page,
            has_prev: page > 1,
            item: data,
        }))
    }
}

/// 获取单个用户信息
#[utoipa::path(
    get,
    path = "/user/{user_id}",
    tag = "user",
    params(("user_id" = i64, Path, description = "用户 ID")),
    responses(
        (status = 200, description = "用户信息", body = entity::user::Model),
        (status = 404, description = "未找到"),
    )
)]
async fn get_user_info_handler(
    user_id: Path<i64>,
    state: State<Arc<AppState>>,
) -> Result<Json<entity::user::Model>, (StatusCode, String)> {
    let db = state.db.clone();

    match entity::user::Entity::find_by_id(*user_id).one(&db).await {
        Ok(r) => match r {
            Some(r) => Ok(Json(r)),
            None => Err((StatusCode::NOT_FOUND, "Not Found".to_string())),
        },
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// 分页获取用户回复
#[utoipa::path(
    get,
    path = "/user/{user_id}/reply",
    tag = "user",
    params(("user_id" = i64, Path, description = "用户 ID")),
    responses(
        (status = 200, description = "分页用户回复", body = UserReplyPage),
    )
)]
async fn get_user_reply_handler(
    user_id: Path<i64>,
    pagination: Query<Pagination>,
    state: State<Arc<AppState>>,
) -> Result<Json<UserReplyPage>, (StatusCode, String)> {
    let db = state.db.clone();
    let size = pagination.size;
    let page = pagination.page;
    if size > 30 {
        return Err((StatusCode::BAD_REQUEST, "Too Big".to_string()));
    }
    let data = entity::replies::Entity::find()
        .filter(entity::replies::Column::UserId.eq(*user_id))
        .find_also_related(entity::user::Entity)
        .order_by_asc(entity::replies::Column::Id)
        .paginate(&db, size as u64)
        .fetch_page((page - 1) as u64)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    let total_page = entity::replies::Entity::find()
        .filter(entity::replies::Column::UserId.eq(*user_id))
        .count(&db)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
        .div_ceil(size as u64);
    let post_ids: Vec<i64> = data.iter().filter_map(|(r, _)| r.post_id).collect();
    let posts = if post_ids.is_empty() {
        vec![]
    } else {
        entity::posts::Entity::find()
            .filter(entity::posts::Column::Id.is_in(post_ids))
            .all(&db)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
    };
    let posts_by_id: HashMap<i64, entity::posts::Model> =
        posts.into_iter().map(|p| (p.id, p)).collect();
    let mut items = vec![];
    for (reply, user) in data {
        let user = match user {
            Some(u) => u,
            None => return Err((StatusCode::NOT_FOUND, "Not Found".to_string())),
        };
        let post = match reply.post_id.and_then(|id| posts_by_id.get(&id)) {
            Some(p) => p.clone(),
            None => return Err((StatusCode::NOT_FOUND, "Not Found".to_string())),
        };
        items.push(UserReplyDTO { reply, user, post });
    }
    Ok(Json(UserReplyPage {
        current: page,
        total: total_page as u32,
        has_next: page < total_page as u32,
        has_prev: page > 1,
        item: items,
    }))
}

/// 分页获取用户帖子
#[utoipa::path(
    get,
    path = "/user/{user_id}/posts",
    tag = "user",
    params(("user_id" = i64, Path, description = "用户 ID")),
    responses(
        (status = 200, description = "分页用户帖子", body = UserPostPage),
    )
)]
async fn get_user_post_handler(
    user_id: Path<i64>,
    pagination: Query<Pagination>,
    state: State<Arc<AppState>>,
) -> Result<Json<UserPostPage>, (StatusCode, String)> {
    let db = state.db.clone();
    let size = pagination.size;
    let page = pagination.page;
    if size > 30 {
        return Err((StatusCode::BAD_REQUEST, "Too Big".to_string()));
    }
    let data = entity::posts::Entity::find()
        .filter(entity::posts::Column::UserId.eq(*user_id))
        .find_also_related(entity::user::Entity)
        .order_by_asc(entity::posts::Column::Id)
        .paginate(&db, size as u64)
        .fetch_page((page - 1) as u64)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    let total_page = entity::posts::Entity::find()
        .filter(entity::posts::Column::UserId.eq(*user_id))
        .count(&db)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
        .div_ceil(size as u64);
    let mut items = vec![];
    for (post, user) in data {
        let user = match user {
            Some(u) => u,
            None => return Err((StatusCode::NOT_FOUND, "Not Found".to_string())),
        };
        items.push(UserPostDTO { post, user });
    }
    Ok(Json(UserPostPage {
        current: page,
        total: total_page as u32,
        has_next: page < total_page as u32,
        has_prev: page > 1,
        item: items,
    }))
}

/// 分页获取用户评论
#[utoipa::path(
    get,
    path = "/user/{user_id}/comments",
    tag = "user",
    params(("user_id" = i64, Path, description = "用户 ID")),
    responses(
        (status = 200, description = "分页用户评论", body = UserCommentPage),
    )
)]
async fn get_user_comment_handler(
    user_id: Path<i64>,
    pagination: Query<Pagination>,
    state: State<Arc<AppState>>,
) -> Result<Json<UserCommentPage>, (StatusCode, String)> {
    let db = state.db.clone();
    let size = pagination.size;
    let page = pagination.page;
    if size > 30 {
        return Err((StatusCode::BAD_REQUEST, "Too Big".to_string()));
    }
    let data = entity::comments::Entity::find()
        .filter(entity::comments::Column::UserId.eq(*user_id))
        .order_by_asc(entity::comments::Column::Id)
        .paginate(&db, size as u64)
        .fetch_page((page - 1) as u64)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    let total_page = entity::comments::Entity::find()
        .filter(entity::comments::Column::UserId.eq(*user_id))
        .count(&db)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
        .div_ceil(size as u64);
    let user_ids: Vec<i64> = data.iter().filter_map(|c| c.user_id).collect();
    let reply_ids: Vec<i64> = data.iter().filter_map(|c| c.reply_id).collect();
    let users = if user_ids.is_empty() {
        vec![]
    } else {
        entity::user::Entity::find()
            .filter(entity::user::Column::Id.is_in(user_ids))
            .all(&db)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
    };
    let users_by_id: HashMap<i64, entity::user::Model> =
        users.into_iter().map(|u| (u.id, u)).collect();
    let replies = if reply_ids.is_empty() {
        vec![]
    } else {
        entity::replies::Entity::find()
            .filter(entity::replies::Column::Id.is_in(reply_ids))
            .all(&db)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
    };
    let replies_by_id: HashMap<i64, entity::replies::Model> =
        replies.into_iter().map(|r| (r.id, r)).collect();
    let post_ids: Vec<i64> = replies_by_id.values().filter_map(|r| r.post_id).collect();
    let posts = if post_ids.is_empty() {
        vec![]
    } else {
        entity::posts::Entity::find()
            .filter(entity::posts::Column::Id.is_in(post_ids))
            .all(&db)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
    };
    let posts_by_id: HashMap<i64, entity::posts::Model> =
        posts.into_iter().map(|p| (p.id, p)).collect();
    let mut items = vec![];
    for comment in data {
        let user = match comment.user_id.and_then(|id| users_by_id.get(&id)) {
            Some(u) => u.clone(),
            None => return Err((StatusCode::NOT_FOUND, "Not Found".to_string())),
        };
        let reply = match comment.reply_id.and_then(|id| replies_by_id.get(&id)) {
            Some(r) => r.clone(),
            None => return Err((StatusCode::NOT_FOUND, "Not Found".to_string())),
        };
        let post = match reply.post_id.and_then(|id| posts_by_id.get(&id)) {
            Some(p) => p.clone(),
            None => return Err((StatusCode::NOT_FOUND, "Not Found".to_string())),
        };
        items.push(UserCommentDTO {
            comment,
            user,
            reply,
            post,
        });
    }
    Ok(Json(UserCommentPage {
        current: page,
        total: total_page as u32,
        has_next: page < total_page as u32,
        has_prev: page > 1,
        item: items,
    }))
}

pub fn user_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_user_handler))
        .route("/{user_id}", get(get_user_info_handler))
        .route("/{user_id}/reply", get(get_user_reply_handler))
        .route("/{user_id}/posts", get(get_user_post_handler))
        .route("/{user_id}/comments", get(get_user_comment_handler))
}
