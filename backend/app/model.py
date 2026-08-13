from sqlalchemy.ext.asyncio import create_async_engine
from sqlmodel.ext.asyncio.session import AsyncSession
import os
from datetime import datetime

from sqlmodel import Field, Relationship, SQLModel, create_engine


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
    posts: list["Posts"] = Relationship(back_populates="user")


class Board(SQLModel, table=True):
    id: int = Field(primary_key=True)
    name: str
    is_hot: str
    n_post: int
    n_discussions: int
    posts: list["Posts"] = Relationship(back_populates="board")


class Posts(SQLModel, table=True):
    id: int = Field(primary_key=True)
    ask_help_flag: int
    board_id: int = Field(foreign_key="board.id")
    board: Board = Relationship(back_populates="posts")
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
    user: User = Relationship(back_populates="posts")
    replies: list["Replies"] = Relationship(back_populates="post")


class Replies(SQLModel, table=True):
    id: int = Field(primary_key=True)
    content: str
    created_at: datetime
    is_liked: bool
    is_top: bool
    n_comments: int
    n_likes: int
    post_id: int = Field(foreign_key="posts.id")
    post: Posts = Relationship(back_populates="replies")
    update_at: datetime
    user_id: int = Field(foreign_key="user.id")
    comments:list["Comments"] = Relationship(back_populates="reply")

class Comments(SQLModel, table=True):
    id: int = Field(primary_key=True)
    content: str
    created_at: datetime
    is_liked: bool
    n_likes: int
    reply_id: int = Field(foreign_key="replies.id")
    reply:Replies = Relationship(back_populates="comments")
    reply_user_id: int | None = Field(foreign_key="user.id")
    user_id: int = Field(foreign_key="user.id")

engine = create_async_engine(os.getenv("DB_URL",""))

async def get_session():
    async with AsyncSession(engine) as s:
        yield s
    
