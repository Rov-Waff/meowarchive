use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
};
use sea_orm::{EntityTrait, PaginatorTrait};

use crate::{
    AppState,
    dtos::{PageResult, Pagination},
    entity,
};

#[axum::debug_handler]
async fn get_all_user_handler(
    pagination: Query<Pagination>,
    state: State<Arc<AppState>>,
) -> Result<Json<PageResult<entity::user::Model>>, (StatusCode, String)> {
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
        Ok(Json(PageResult {
            current: page,
            total: total_page,
            has_next: page > 1,
            has_prev: page < total_page,
            item: data,
        }))
    }
}

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

pub fn user_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_user_handler))
        .route("/{user_id}", get(get_user_info_handler))
}
