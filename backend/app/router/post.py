from sqlalchemy.orm import selectinload
from fastapi import APIRouter, Depends, Path
from sqlmodel import select
from sqlmodel.ext.asyncio.session import AsyncSession

from app.dtos.post import PostDetailsDTO
from app.model import Posts, get_session

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
