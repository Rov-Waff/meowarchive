use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    routing::get,
};
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    sea_query::{Alias, BinOper, Condition, Expr, ExprTrait, Func},
};

use crate::{
    AppState,
    dtos::{CommentSearchDTO, CommentSearchPage, Pagination, SearchParams},
    entity,
};

/// 按内容搜索评论（全文检索）
#[utoipa::path(
    get,
    path = "/comment/search/content",
    tag = "comment",
    params(
        ("keyword" = String, Query, description = "搜索关键词"),
    ),
    responses(
        (status = 200, description = "搜索结果", body = CommentSearchPage),
    )
)]
async fn search_content_handler(
    keyword: Query<SearchParams>,
    pagination: Query<Pagination>,
    state: State<Arc<AppState>>,
) -> Result<Json<CommentSearchPage>, (StatusCode, String)> {
    let db = state.db.clone();
    let size = pagination.size;
    let page = pagination.page;
    if size > 30 {
        return Err((StatusCode::BAD_REQUEST, "Too Big".to_string()));
    }
    let tsvector = Expr::expr(
        Func::cust("to_tsvector")
            .arg(Expr::val("zh_cn").cast_as(Alias::new("regconfig")))
            .arg(Expr::col(entity::comments::Column::Content)),
    );
    let tsquery = Expr::expr(
        Func::cust("to_tsquery")
            .arg(Expr::val("zh_cn").cast_as(Alias::new("regconfig")))
            .arg(Expr::val(keyword.keyword.clone())),
    );
    let cond = Condition::all().add(tsvector.binary(BinOper::Custom("@@"), tsquery));
    let data = entity::comments::Entity::find()
        .filter(cond.clone())
        .order_by_asc(entity::comments::Column::Id)
        .paginate(&db, size as u64)
        .fetch_page((page - 1) as u64)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    let total_page = entity::comments::Entity::find()
        .filter(cond)
        .count(&db)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
        .div_ceil(size as u64);
    let user_ids: Vec<i64> = data.iter().filter_map(|c| c.user_id).collect();
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
    let mut items = vec![];
    for comment in data {
        let user = match comment.user_id.and_then(|id| users_by_id.get(&id)) {
            Some(u) => u.clone(),
            None => return Err((StatusCode::NOT_FOUND, "Not Found".to_string())),
        };
        items.push(CommentSearchDTO {
            id: comment.id,
            content: comment.content,
            created_at: comment.created_at,
            n_likes: comment.n_likes,
            user,
        });
    }
    Ok(Json(CommentSearchPage {
        current: page,
        total: total_page as u32,
        has_next: page < total_page as u32,
        has_prev: page > 1,
        item: items,
    }))
}

pub fn comment_router() -> Router<Arc<AppState>> {
    Router::new().route("/search/content", get(search_content_handler))
}
