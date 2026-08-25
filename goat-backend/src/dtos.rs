use serde::{Deserialize, Serialize};

use crate::entity;
#[derive(Deserialize, Serialize, utoipa::ToSchema)]
pub struct Pagination {
    pub page: u32,
    pub size: u32,
}

#[derive(Deserialize, Serialize, utoipa::ToSchema)]
pub struct PageResult<T> {
    pub current: u32,
    pub total: u32,
    pub has_next: bool,
    pub has_prev: bool,
    pub item: Vec<T>,
}

/// 为 `PageResult<T>` 生成命名具体类型，便于 utoipa 生成 schema。
macro_rules! page_result {
    ($name:ident, $item:ty) => {
        #[derive(Deserialize, Serialize, utoipa::ToSchema)]
        pub struct $name {
            pub current: u32,
            pub total: u32,
            pub has_next: bool,
            pub has_prev: bool,
            pub item: Vec<$item>,
        }
    };
}
page_result!(PostPage, crate::entity::posts::Model);
page_result!(PostCommentPage, PostCommentDTO);
page_result!(UserPage, crate::entity::user::Model);
page_result!(UserReplyPage, UserReplyDTO);
page_result!(UserPostPage, UserPostDTO);
page_result!(UserCommentPage, UserCommentDTO);
page_result!(PostSearchPage, SearchDTO);
page_result!(ReplySearchPage, ReplySearchDTO);
page_result!(CommentSearchPage, CommentSearchDTO);

#[derive(Deserialize, Serialize, utoipa::ToSchema)]
pub struct PostDetail {
    pub post: entity::posts::Model,
    pub user: entity::user::Model,
}

#[derive(Deserialize,Serialize, utoipa::ToSchema)]
pub struct PostCommentDTO {
    pub reply: entity::replies::Model,
    pub user: entity::user::Model,
    pub comments: Vec<entity::comments::Model>,
}

#[derive(Deserialize, Serialize, utoipa::ToSchema)]
pub struct UserReplyDTO {
    pub reply: entity::replies::Model,
    pub user: entity::user::Model,
    pub post: entity::posts::Model,
}

#[derive(Deserialize, Serialize, utoipa::ToSchema)]
pub struct UserPostDTO {
    pub post: entity::posts::Model,
    pub user: entity::user::Model,
}

#[derive(Deserialize, Serialize, utoipa::ToSchema)]
pub struct UserCommentDTO {
    pub comment: entity::comments::Model,
    pub user: entity::user::Model,
    pub reply: entity::replies::Model,
    pub post: entity::posts::Model,
}

#[derive(Deserialize, Serialize, utoipa::ToSchema)]
pub struct SearchDTO {
    pub id: i64,
    pub title: Option<String>,
    pub user: entity::user::Model,
    pub n_replies: Option<i32>,
    pub n_view: Option<i32>,
    pub n_comments: Option<i32>,
}

#[derive(Deserialize, Serialize, utoipa::ToSchema)]
pub struct ReplySearchDTO {
    pub id: i64,
    pub content: Option<String>,
    pub created_at: Option<sea_orm::entity::prelude::DateTime>,
    pub n_likes: Option<i32>,
    pub n_comments: Option<i32>,
    pub user: entity::user::Model,
}

#[derive(Deserialize, Serialize, utoipa::ToSchema)]
pub struct CommentSearchDTO {
    pub id: i64,
    pub content: Option<String>,
    pub created_at: Option<sea_orm::entity::prelude::DateTime>,
    pub n_likes: Option<i32>,
    pub user: entity::user::Model,
}
