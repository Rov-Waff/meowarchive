import math

from fastapi import APIRouter, Depends, Path, Query
from sqlalchemy.orm import selectinload
from sqlmodel import asc, func, select
from sqlmodel.ext.asyncio.session import AsyncSession

from app.dtos import PageResult
from app.dtos.board import BoardPostPageDTO
from app.dtos.user import UserCommentDTO, UserReplyDTO
from app.model import Comments, Posts, Replies, User, get_session

user = APIRouter(prefix="/user")


@user.get("/")
async def get_all_paged_user(
    page_size: int = Query(default=30),
    page_num: int = Query(default=1),
    session: AsyncSession = Depends(get_session),
):
    total_stmt = select(func.count()).select_from(User)
    total = (await session.exec(total_stmt)).one()
    total_page = math.ceil(total / page_size)
    stmt = (
        select(User)
        .limit(page_size)
        .offset((page_num - 1) * page_size)
        .order_by(asc(User.id))
    )
    data = (await session.exec(stmt)).all()
    return PageResult[User](
        total_page=total_page,
        current_page=page_num,
        has_prev=page_num > 1,
        has_next=page_num < total_page,
        item=data,
    )


@user.get("/{id}")
async def get_user_info(
    id: int = Path(),
    session: AsyncSession = Depends(get_session),
):
    stmt = select(User).where(User.id == id)
    user = (await session.exec(stmt)).one()
    return user


@user.get("/{id}/reply")
async def get_user_reply(
    id: int = Path(),
    page_size: int = Query(default=30),
    page_num: int = Query(default=1),
    session: AsyncSession = Depends(get_session),
):
    total_stmt = select(func.count()).select_from(Replies).where(Replies.user_id == id)
    total = (await session.exec(total_stmt)).one()
    total_page = math.ceil(total / page_size)
    stmt = (
        select(Replies)
        .where(Replies.user_id == id)
        .limit(page_size)
        .offset((page_num - 1) * page_size)
        .options(selectinload(Replies.user))  # ty: ignore[invalid-argument-type]
        .options(selectinload(Replies.post))  # ty: ignore[invalid-argument-type]
        .order_by(asc(Replies.id))
    )
    data = (await session.exec(stmt)).all()
    return PageResult[UserReplyDTO](
        total_page=total_page,
        current_page=page_num,
        has_prev=page_num > 1,
        has_next=page_num < total_page,
        item=[UserReplyDTO(reply=i, post=i.post, user=i.user) for i in data],
    )


@user.get("/{id}/posts")
async def get_user_post(
    id: int = Path(),
    page_size: int = Query(default=30),
    page_num: int = Query(default=1),
    session: AsyncSession = Depends(get_session),
):
    total_stmt = select(func.count()).select_from(Posts).where(Posts.user_id == id)
    total = (await session.exec(total_stmt)).one()
    total_page = math.ceil(total / page_size)
    stmt = (
        select(Posts)
        .where(Posts.user_id == id)
        .limit(page_size)
        .offset((page_num - 1) * page_size)
        .options(selectinload(Posts.user))  # ty: ignore[invalid-argument-type]
        .order_by(asc(Posts.id))
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


@user.get("/{id}/comments")
async def get_user_comment(
    id: int = Path(),
    page_size: int = Query(default=30),
    page_num: int = Query(default=1),
    session: AsyncSession = Depends(get_session),
):
    total_stmt = (
        select(func.count()).select_from(Comments).where(Comments.user_id == id)
    )
    total = (await session.exec(total_stmt)).one()
    total_page = math.ceil(total / page_size)
    stmt = (
        select(Comments)
        .where(Comments.user_id == id)
        .limit(page_size)
        .offset((page_num - 1) * page_size)
        .options(selectinload(Comments.user))  # ty: ignore[invalid-argument-type]
        .options(selectinload(Comments.reply).selectinload(Replies.post))  # ty: ignore[invalid-argument-type]
        .order_by(asc(Comments.id))
    )
    data = (await session.exec(stmt)).all()
    return PageResult[UserCommentDTO](
        total_page=total_page,
        current_page=page_num,
        has_prev=page_num > 1,
        has_next=page_num < total_page,
        item=[
            UserCommentDTO(comment=c, reply=c.reply, post=c.reply.post, user=c.user)
            for c in data
        ],
    )
