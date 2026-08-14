import math

from fastapi import APIRouter, Depends, Path, Query
from sqlalchemy.orm import selectinload
from sqlmodel import func, select
from sqlmodel.ext.asyncio.session import AsyncSession

from app.dtos import PageResult
from app.dtos.post import PostDetailsDTO
from app.model import Posts, Replies, get_session

post = APIRouter(prefix="/post")


@post.get("/{id}")
async def get_post_by_id(
    id: int = Path(), session: AsyncSession = Depends(get_session)
):
    stmt = select(Posts).where(Posts.id == id).options(selectinload(Posts.user))  # ty: ignore[invalid-argument-type]
    res = (await session.exec(stmt)).one()
    return PostDetailsDTO(
        ask_help_flag=res.ask_help_flag,
        board_id=res.board_id,
        content=res.content,
        is_authorized=res.is_authorized,
        n_replies=res.n_replies,
        title=res.title,
        user=res.user,
        id=res.id,
        board_name=res.board_name,
        created_at=res.created_at,
        is_featured=res.is_featured,
        n_comments=res.n_comments,
        n_views=res.n_comments,
        tutorial_flag=res.tutorial_flag,
        is_pinned=res.is_pinned,
    )


@post.get("/{id}/replies")
async def get_replies_by_post_id(
    id: int = Path(),
    page_size=Query(default=30, le=50),
    page_num=Query(default=1),
    session: AsyncSession = Depends(get_session),
):
    stmt = (
        select(Replies)
        .where(Replies.post_id == id)
        .limit(page_size)
        .offset((page_num - 1) * page_size)
        .options(selectinload(Replies.user))  # ty: ignore[invalid-argument-type]
        .options(selectinload(Replies.comments))  # ty: ignore[invalid-argument-type]
    )
    total_stmt = select(func.count()).select_from(Replies).where(Replies.post_id == id)

    total = (await session.exec(total_stmt)).one()
    total_page = math.ceil(total / page_size)
    has_prev: bool = page_num > 1
    has_next: bool = page_num == total_page
    res = (await session.exec(stmt)).all()
    return PageResult[Replies](
        total_page=total_page,
        current_page=page_num,
        has_prev=has_prev,
        has_next=has_next,
        item=res,
    )
