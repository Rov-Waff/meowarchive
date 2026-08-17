import math

from fastapi import APIRouter, Depends, Path, Query
from sqlalchemy.orm import selectinload
from sqlmodel import asc, func, select
from sqlmodel.ext.asyncio.session import AsyncSession

from app.dtos import PageResult
from app.dtos.board import BoardPostPageDTO, BoardWithStatsDTO
from app.model import Board, Posts, Replies, get_session

board: APIRouter = APIRouter(prefix="/board", dependencies=[Depends(get_session)])


@board.get("/all")
async def get_all_board(sess: AsyncSession = Depends(get_session)):
    # 帖子量：该板块的帖子数（SQL 实时统计，而非 board 表冗余字段）
    posts_count = (
        select(func.count())
        .select_from(Posts)
        .where(Posts.board_id == Board.id)
        .correlate(Board)
        .scalar_subquery()
    )
    # 讨论量：该板块所有帖子的回复总数
    replies_count = (
        select(func.count())
        .select_from(Replies)
        .join(Posts, Replies.post_id == Posts.id)
        .where(Posts.board_id == Board.id)
        .correlate(Board)
        .scalar_subquery()
    )
    stmt = select(Board, posts_count, replies_count).order_by(asc(Board.id))
    rows = (await sess.exec(stmt)).all()
    return [
        BoardWithStatsDTO(
            id=item.id,
            name=item.name,
            is_hot=item.is_hot,
            n_posts=posts,
            n_discussions=replies,
        )
        for item, posts, replies in rows
    ]


# TODO: 获取单个板块的信息
@board.get("/{id}")
async def get_board_info(id: int = Path(), sess: AsyncSession = Depends(get_session)):
    stmt = select(Board).where(Board.id == id)
    return (await sess.exec(stmt)).one()


# TODO: 获取一个板块下的帖子，分页
@board.get("/{id}/page")
async def get_paged_posts_in_board(
    id: int = Path(),
    page_size: int = Query(lt=50),
    page_num: int = Query(),
    sess: AsyncSession = Depends(get_session),
):
    total_stmt = select(func.count()).select_from(Posts).where(Posts.board_id == id)
    total = (await sess.exec(total_stmt)).one()
    total_page = math.ceil(total / page_size)
    has_prev: bool = page_num > 1
    has_next: bool = page_num < total_page
    stmt = (
        select(Posts)
        .where(Posts.board_id == id)
        .limit(page_size)
        .offset((page_num - 1) * page_size)
        .options(selectinload(Posts.user))  # ty: ignore[invalid-argument-type]
    )
    items = (await sess.exec(stmt)).all()
    res = [
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
        for item in items
    ]

    return PageResult[BoardPostPageDTO](
        total_page=total_page,
        current_page=page_num,
        has_prev=has_prev,
        has_next=has_next,
        item=res,
    )
