use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
};
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    sea_query::{BinOper, Condition, Expr, ExprTrait, Func},
};

use crate::{
    AppState,
    dtos::{
        Pagination, PostCommentDTO, PostCommentPage, PostPage, PostSearchPage, SearchDTO,
        SearchParams,
    },
    entity,
};

/// 获取帖子详情
#[utoipa::path(
    get,
    path = "/post/{post_id}",
    tag = "post",
    params(("post_id" = u32, Path, description = "帖子 ID")),
    responses(
        (status = 200, description = "帖子详情", body = crate::dtos::PostDetail),
        (status = 404, description = "未找到"),
    )
)]
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

/// 分页获取帖子回复（含评论）
#[utoipa::path(
    get,
    path = "/post/{post_id}/replies",
    tag = "post",
    params(("post_id" = u32, Path, description = "帖子 ID")),
    responses(
        (status = 200, description = "分页回复列表", body = PostCommentPage),
    )
)]
async fn get_post_replies(
    post_id: Path<u32>,
    state: State<Arc<AppState>>,
    pagination: Query<Pagination>,
) -> Result<Json<PostCommentPage>, (StatusCode, String)> {
    let size = pagination.size;
    let page = pagination.page;
    let db = state.db.clone();
    if size > 50 {
        return Err((StatusCode::BAD_REQUEST, "Too Big size".to_string()));
    }
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
    Ok(Json(PostCommentPage {
        current: page,
        total: total_page as u32,
        has_next: page < total_page as u32,
        has_prev: page > 1,
        item: items,
    }))
}

/// 分页获取全部帖子
#[utoipa::path(
    get,
    path = "/post",
    tag = "post",
    responses(
        (status = 200, description = "分页帖子列表", body = PostPage),
    )
)]
#[axum::debug_handler]
async fn get_all_posts(
    state: State<Arc<AppState>>,
    pagination: Query<Pagination>,
) -> Result<Json<PostPage>, (StatusCode, String)> {
    let db = state.db.clone();
    let size = pagination.size;
    let page = pagination.page;
    if size > 50 {
        return Err((StatusCode::BAD_REQUEST, "Too big size".to_string()));
    } else {
        let total = entity::posts::Entity::find()
            .count(&db)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
            .div_ceil(size as u64);
        let res = entity::posts::Entity::find()
            .paginate(&db, size as u64)
            .fetch_page((page - 1) as u64)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
        return Ok(Json(PostPage {
            current: page,
            total: total as u32,
            has_next: page < total as u32,
            has_prev: page > 1,
            item: res,
        }));
    }
}

async fn search_posts_handler(
    keyword: Query<SearchParams>,
    pagination: Query<Pagination>,
    state: State<Arc<AppState>>,
    column: entity::posts::Column,
) -> Result<Json<PostSearchPage>, (StatusCode, String)> {
    let db = state.db.clone();
    let size = pagination.size;
    let page = pagination.page;
    if size > 30 {
        return Err((StatusCode::BAD_REQUEST, "Too Big".to_string()));
    }
    let tsvector = Expr::expr(
        Func::cust("to_tsvector")
            .arg(Expr::cust("'zh_cn'::regconfig"))
            .arg(Func::cust("strip_html").arg(Expr::col(column))),
    );
    let tsquery = Expr::expr(
        Func::cust("to_tsquery")
            .arg(Expr::cust("'zh_cn'::regconfig"))
            .arg(Expr::val(keyword.keyword.clone())),
    );
    let cond = Condition::all().add(tsvector.binary(BinOper::Custom("@@"), tsquery));
    let data = entity::posts::Entity::find()
        .filter(cond.clone())
        .find_also_related(entity::user::Entity)
        .paginate(&db, size as u64)
        .fetch_page((page - 1) as u64)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    let total_page = entity::posts::Entity::find()
        .filter(cond)
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
        items.push(SearchDTO {
            id: post.id,
            title: post.title,
            user,
            n_replies: post.n_replies,
            n_view: post.n_views,
            n_comments: post.n_comments,
        });
    }
    Ok(Json(PostSearchPage {
        current: page,
        total: total_page as u32,
        has_next: page < total_page as u32,
        has_prev: page > 1,
        item: items,
    }))
}

/// 按内容搜索帖子（全文检索）
#[utoipa::path(
    get,
    path = "/post/search/content",
    tag = "post",
    params(
        ("keyword" = String, Query, description = "搜索关键词"),
    ),
    responses(
        (status = 200, description = "搜索结果", body = PostSearchPage),
    )
)]
async fn search_posts_content_handler(
    keyword: Query<SearchParams>,
    pagination: Query<Pagination>,
    state: State<Arc<AppState>>,
) -> Result<Json<PostSearchPage>, (StatusCode, String)> {
    search_posts_handler(keyword, pagination, state, entity::posts::Column::Content).await
}

/// 按标题搜索帖子（全文检索）
#[utoipa::path(
    get,
    path = "/post/search/title",
    tag = "post",
    params(
        ("keyword" = String, Query, description = "搜索关键词"),
    ),
    responses(
        (status = 200, description = "搜索结果", body = PostSearchPage),
    )
)]
async fn search_posts_title_handler(
    keyword: Query<SearchParams>,
    pagination: Query<Pagination>,
    state: State<Arc<AppState>>,
) -> Result<Json<PostSearchPage>, (StatusCode, String)> {
    search_posts_handler(keyword, pagination, state, entity::posts::Column::Title).await
}

pub fn posts_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_posts))
        .route("/{post_id}", get(get_post_detail_handler))
        .route("/{post_id}/replies", get(get_post_replies))
        .route("/search/content", get(search_posts_content_handler))
        .route("/search/title", get(search_posts_title_handler))
}
