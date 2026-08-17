from datetime import datetime

from pydantic import BaseModel

from app.model import User


class BoardWithStatsDTO(BaseModel):
    id: int
    name: str
    is_hot: bool
    n_posts: int
    n_discussions: int


class BoardPostPageDTO(BaseModel):
    id: int
    ask_help_flag: int
    board_id: int
    board_name: str
    created_at: datetime
    is_authorized: bool
    is_featured: bool
    is_pinned: bool
    n_comments: int
    n_replies: int
    n_views: int
    title: str
    tutorial_flag: int
    user_id: int
    user: User
