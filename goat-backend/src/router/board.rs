use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use sea_orm::{DbErr, EntityTrait};

use crate::{AppState, entity};

#[axum::debug_handler]
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

pub fn board_router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_all_board_handler))
}
