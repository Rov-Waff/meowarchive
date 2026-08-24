use serde::{Deserialize, Serialize};

use crate::entity;

#[derive(Deserialize, Serialize)]
pub struct Pagination {
    pub page: u32,
    pub size: u32,
}

#[derive(Deserialize, Serialize)]
pub struct PageResult<T> {
    pub current: u32,
    pub total: u32,
    pub has_next: bool,
    pub has_prev: bool,
    pub item: Vec<T>,
}

#[derive(Deserialize, Serialize)]
pub struct PostDetail {
    pub post: entity::posts::Model,
    pub user: entity::user::Model,
}
