use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{AppState, entity};

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

pub fn posts_router() -> Router<Arc<AppState>> {
    Router::new().route("/{post_id}", get(get_post_detail_handler))
}
