import math

from fastapi import APIRouter, Depends, Query
from sqlalchemy.orm import load_only, selectinload
from sqlmodel import asc, func, select
from sqlmodel.ext.asyncio.session import AsyncSession

from app.dtos import PageResult
from app.dtos.post import ReplySearchDTO
from app.model import Replies, get_session

reply = APIRouter(prefix="/reply")


@reply.get("/search/content", response_model=PageResult[ReplySearchDTO])
async def search_content(
    keyword: str = Query(),
    page_size: int = Query(default=30),
    page_num: int = Query(),
    session: AsyncSession = Depends(get_session),
):
    stmt = (
        select(Replies)
        .where(
            func.to_tsvector("zh_cn", Replies.content).op("@@")(
                func.to_tsquery("zh_cn", keyword)
            )
        )
        .options(
            load_only(
                Replies.id, Replies.content, Replies.created_at, Replies.n_likes, Replies.n_comments  # ty: ignore[invalid-argument-type]
            ),
            selectinload(Replies.user),  # ty: ignore[invalid-argument-type]
        )
        .order_by(asc(Replies.id))
        .limit(page_size)
        .offset((page_num - 1) * page_size)
    )
    total_stmt = (
        select(func.count())
        .select_from(Replies)
        .where(
            func.to_tsvector("zh_cn", Replies.content).op("@@")(
                func.to_tsquery("zh_cn", keyword)
            )
        )
    )
    total = (await session.exec(total_stmt)).one()
    data = (await session.exec(stmt)).all()
    total_page = math.ceil(total / page_size)
    return PageResult[ReplySearchDTO](
        total_page=total_page,
        current_page=page_num,
        has_prev=page_num > 1,
        has_next=page_num < total_page,
        item=[
            ReplySearchDTO(
                id=i.id,
                content=i.content,
                created_at=i.created_at,
                n_likes=i.n_likes,
                n_comments=i.n_comments,
                user=i.user,
            )
            for i in data
        ],
    )
