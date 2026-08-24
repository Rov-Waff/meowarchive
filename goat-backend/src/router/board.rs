use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use sea_orm::{DbErr, EntityTrait};

use crate::{AppState, entity};

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

pub fn board_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_board_handler))
        .route("/{board_id}", get(get_board_info))
}
