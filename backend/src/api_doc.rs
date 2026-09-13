use utoipa::OpenApi;

use crate::{
    dtos::{
        CommentSearchDTO, CommentSearchPage, Pagination, PostCommentDTO, PostCommentPage,
        PostDetail, PostPage, PostSearchPage, ReplySearchDTO, ReplySearchPage, SearchDTO,
        UserCommentDTO, UserCommentPage, UserPage, UserPostDTO, UserPostPage, UserReplyDTO,
        UserReplyPage,
    },
    entity,
    router::{board, comment, posts, reply, user},
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "goat-backend",
        version = "0.1.0",
        description = "Meowarchive Rust 后端（axum + sea-orm）"
    ),
    paths(
        board::get_all_board_handler,
        board::get_board_info,
        board::get_board_posts_handler,
        posts::get_all_posts,
        posts::get_post_detail_handler,
        posts::get_post_replies,
        posts::search_posts_content_handler,
        posts::search_posts_title_handler,
        user::get_all_user_handler,
        user::get_user_info_handler,
        user::get_user_reply_handler,
        user::get_user_post_handler,
        user::get_user_comment_handler,
        reply::search_content_handler,
        comment::search_content_handler,
    ),
    components(
        schemas(
            entity::board::Model,
            entity::posts::Model,
            entity::user::Model,
            entity::replies::Model,
            entity::comments::Model,
            Pagination,
            PostDetail,
            PostCommentDTO,
            UserReplyDTO,
            UserPostDTO,
            UserCommentDTO,
            SearchDTO,
            ReplySearchDTO,
            CommentSearchDTO,
            PostPage,
            PostCommentPage,
            UserPage,
            UserReplyPage,
            UserPostPage,
            UserCommentPage,
            PostSearchPage,
            ReplySearchPage,
            CommentSearchPage,
        )
    ),
    tags(
        (name = "board", description = "板块相关"),
        (name = "post", description = "帖子相关"),
        (name = "user", description = "用户相关"),
        (name = "reply", description = "回复相关"),
        (name = "comment", description = "评论相关"),
    )
)]
pub struct ApiDoc;
