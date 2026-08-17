import math

from fastapi import APIRouter, Depends, Path, Query
from sqlalchemy.orm import selectinload
from sqlmodel import asc, func, select
from sqlmodel.ext.asyncio.session import AsyncSession

from app.dtos import PageResult
from app.dtos.board import BoardPostPageDTO
from app.dtos.post import CommentDTO, PostDetailsDTO, PostRepliesDTO
from app.model import Comments, Posts, Replies, get_session

post = APIRouter(prefix="/post")


@post.get("/{id}")
async def get_post_by_id(
    id: int = Path(),
    session: AsyncSession = Depends(get_session),
):
    stmt = (
        select(Posts)
        .where(Posts.id == id)
        .options(selectinload(Posts.user))  # ty: ignore[invalid-argument-type]
        .order_by(asc(Posts.id))
    )
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
    page_size: int = Query(default=30, lt=50),
    page_num: int = Query(default=1),
    session: AsyncSession = Depends(get_session),
):
    stmt = (
        select(Replies)
        .where(Replies.post_id == id)
        .limit(page_size)
        .offset((page_num - 1) * page_size)
        .options(selectinload(Replies.user))  # ty: ignore[invalid-argument-type]
        .options(selectinload(Replies.comments).selectinload(Comments.user))  # ty: ignore[invalid-argument-type]
        .order_by(asc(Replies.id))
    )
    total_stmt = select(func.count()).select_from(Replies).where(Replies.post_id == id)

    total = (await session.exec(total_stmt)).one()
    total_page = math.ceil(total / page_size)
    has_prev: bool = page_num > 1
    has_next: bool = page_num == total_page
    res = (await session.exec(stmt)).all()
    return PageResult[PostRepliesDTO](
        total_page=total_page,
        current_page=page_num,
        has_prev=has_prev,
        has_next=has_next,
        item=[
            PostRepliesDTO(
                reply=i,
                user=i.user,
                comments=[
                    CommentDTO(**c.model_dump(), user=c.user) for c in i.comments
                ],
            )
            for i in res
        ],
    )


@post.get("/")
async def get_all_paged_posts(
    page_size: int = Query(default=30, lt=50),
    page_num: int = Query(default=1),
    session: AsyncSession = Depends(get_session),
):
    total_stmt = select(func.count()).select_from(Posts)
    total = (await session.exec(total_stmt)).one()
    total_page = math.ceil(total / page_size)
    stmt = (
        select(Posts)
        .limit(page_size)
        .offset((page_num - 1) * page_size)
        .order_by(asc(Posts.id))
        .options(selectinload(Posts.user))  # ty: ignore[invalid-argument-type]
    )
    data = (await session.exec(stmt)).all()
    return PageResult[BoardPostPageDTO](
        total_page=total_page,
        current_page=page_num,
        has_prev=page_num > 1,
        has_next=page_num < total_page,
        item=[
            BoardPostPageDTO(
                id=item.id,
                ask_help_flag=item.ask_help_flag,
                board_id=item.board_id,
                board_name=item.board_name,
                created_at=item.created_at,
                is_authorized=item.is_authorized,
                is_featured=item.is_featured,
                is_pinned=item.is_pinned,
                n_comments=item.n_comments,
                n_replies=item.n_replies,
                n_views=item.n_views,
                title=item.title,
                tutorial_flag=item.tutorial_flag,
                user_id=item.user_id,
                user=item.user,
            )
            for item in data
        ],
    )
