from datetime import datetime
from sqlmodel import Field, SQLModel


class User(SQLModel, table=True):
    id: int = Field(primary_key=True)
    nickname: str
    sex: int
    description: str
    doing: str
    level: int
    avatar: str
    collection_times: int
    forked_times: int
    praise_times: int
    view_times: int


class Board(SQLModel, table=True):
    id: int = Field(primary_key=True)
    name: str
    is_hot: str
    n_post: int
    n_discussions: int


class Posts(SQLModel, table=True):
    id: int = Field(primary_key=True)
    ask_help_flag: int
    board_id: int = Field(foreign_key="board.id")
    board_name: str
    content: str
    created_at: datetime
    is_authorized: bool
    is_featured: bool
    is_pinned: bool
    n_comments: int
    n_replies: int
    n_views: int
    title: str
    tutorial_flag: int
    user_id: int = Field(foreign_key="user.id")


class Replies(SQLModel, table=True):
    id: int = Field(primary_key=True)
    content:str
    created_at:datetime
    is_liked:bool
    is_top:bool
    n_comments:int
    n_likes:int
    post_id:int=Field(foreign_key="posts.id")
    update_at:datetime
    user_id:int=Field(foreign_key="user.id")

class Comments(SQLModel, table=True):
    id: int = Field(primary_key=True)
    content: str
    created_at: datetime
    is_liked: bool
    n_likes: int
    reply_id: int = Field(foreign_key="replies.id")
    reply_user_id: int | None = Field(foreign_key="user.id")
    user_id: int = Field(foreign_key="user.id")
