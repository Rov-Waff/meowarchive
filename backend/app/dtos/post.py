from datetime import datetime

from pydantic import BaseModel

from app.model import Replies, User


class PostDetailsDTO(BaseModel):
    ask_help_flag: int
    board_id: int
    content: str
    is_authorized: bool
    is_pinned: bool
    n_replies: int
    title: str
    user: User
    id: int
    board_name: str
    created_at: datetime
    is_featured: bool
    n_comments: int
    n_views: int
    tutorial_flag: int


class CommentDTO(BaseModel):
    id: int
    content: str
    created_at: datetime
    is_liked: bool
    n_likes: int
    reply_id: int
    reply_user_id: int | None
    user_id: int
    user: User


class PostRepliesDTO(BaseModel):
    reply: Replies
    user: User
    comments: list[CommentDTO]

class SearchDTO(BaseModel):
    title:str
    id:int
    user:User
    n_replies:int
    n_view:int
    n_comments:int


class CommentSearchDTO(BaseModel):
    id: int
    content: str
    created_at: datetime
    n_likes: int
    user: User


class ReplySearchDTO(BaseModel):
    id: int
    content: str
    created_at: datetime
    n_likes: int
    n_comments: int
    user: User
