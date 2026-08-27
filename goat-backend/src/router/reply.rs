use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    routing::get,
};
use sea_orm::{
    EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    sea_query::{Alias, BinOper, Condition, Expr, ExprTrait, Func},
};

use crate::{
    AppState,
    dtos::{Pagination, ReplySearchDTO, ReplySearchPage, SearchParams},
    entity,
};

/// 按内容搜索回复（全文检索）
#[utoipa::path(
    get,
    path = "/reply/search/content",
    tag = "reply",
    params(
        ("keyword" = String, Query, description = "搜索关键词"),
    ),
    responses(
        (status = 200, description = "搜索结果", body = ReplySearchPage),
    )
)]
async fn search_content_handler(
    keyword: Query<SearchParams>,
    pagination: Query<Pagination>,
    state: State<Arc<AppState>>,
) -> Result<Json<ReplySearchPage>, (StatusCode, String)> {
    let db = state.db.clone();
    let size = pagination.size;
    let page = pagination.page;
    if size > 30 {
        return Err((StatusCode::BAD_REQUEST, "Too Big".to_string()));
    }
    let tsvector = Expr::expr(
        Func::cust("to_tsvector")
            .arg(Expr::val("zh_cn").cast_as(Alias::new("regconfig")))
            .arg(Expr::col(entity::replies::Column::Content)),
    );
    let tsquery = Expr::expr(
        Func::cust("to_tsquery")
            .arg(Expr::val("zh_cn").cast_as(Alias::new("regconfig")))
            .arg(Expr::val(keyword.keyword.clone())),
    );
    let cond = Condition::all().add(tsvector.binary(BinOper::Custom("@@"), tsquery));
    let data = entity::replies::Entity::find()
        .filter(cond.clone())
        .find_also_related(entity::user::Entity)
        .order_by_asc(entity::replies::Column::Id)
        .paginate(&db, size as u64)
        .fetch_page((page - 1) as u64)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    let total_page = entity::replies::Entity::find()
        .filter(cond)
        .count(&db)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
        .div_ceil(size as u64);
    let mut items = vec![];
    for (reply, user) in data {
        let user = match user {
            Some(u) => u,
            None => return Err((StatusCode::NOT_FOUND, "Not Found".to_string())),
        };
        items.push(ReplySearchDTO {
            id: reply.id,
            content: reply.content,
            created_at: reply.created_at,
            n_likes: reply.n_likes,
            n_comments: reply.n_comments,
            user,
        });
    }
    Ok(Json(ReplySearchPage {
        current: page,
        total: total_page as u32,
        has_next: page < total_page as u32,
        has_prev: page > 1,
        item: items,
    }))
}

pub fn reply_router() -> Router<Arc<AppState>> {
    Router::new().route("/search/content", get(search_content_handler))
}
