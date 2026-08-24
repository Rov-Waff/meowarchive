use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
};
use sea_orm::{ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter};

use crate::{
    AppState,
    dtos::{PageResult, Pagination},
    entity,
};

async fn get_all_board_handler(
    state: State<Arc<AppState>>,
) -> Result<Json<Vec<entity::board::Model>>, (StatusCode, String)> {
    let db = state.db.clone();
    let res = entity::board::Entity::find()
        .all(&db)
        .await
        .map_err(|err: DbErr| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    Ok(Json(res))
}

async fn get_board_info(
    board_id: Path<i64>,
    state: State<Arc<AppState>>,
) -> Result<Json<entity::board::Model>, (StatusCode, String)> {
    let db = state.db.clone();
    match entity::board::Entity::find_by_id(*board_id)
        .one(&db)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
    {
        Some(res) => Ok(Json(res)),
        None => Err((StatusCode::NOT_FOUND, "Not Found".to_string())),
    }
}

async fn get_board_posts_handler(
    board_id: Path<i64>,
    pagination: Query<Pagination>,
    state: State<Arc<AppState>>,
) -> Result<Json<PageResult<entity::posts::Model>>, (StatusCode, String)> {
    // 参数校验
    let size = pagination.size;
    let page = pagination.page;
    let db = state.db.clone();

    let total = entity::posts::Entity::find()
        .count(&db)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    let total_page = total.div_ceil(size as u64);
    if size > 50 {
        Err((StatusCode::BAD_REQUEST, "Bad Request".to_string()))
    } else {
        Ok(Json(PageResult {
            current: page,
            total: total_page as u32,
            has_next: page > 1,
            has_prev: page < (total_page as u32),
            item: entity::posts::Entity::find()
                .filter(entity::posts::Column::BoardId.eq(*board_id))
                .order_by_id_asc()
                .paginate(&db, size as u64)
                .fetch_page((page - 1) as u64)
                .await
                .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?,
        }))
    }
}

pub fn board_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_board_handler))
        .route("/{board_id}", get(get_board_info))
        .route("/{board_id}/post", get(get_board_posts_handler))
}
