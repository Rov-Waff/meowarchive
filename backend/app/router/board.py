from fastapi import APIRouter, Depends
from sqlmodel import select
from sqlmodel.ext.asyncio.session import AsyncSession

from app.model import Board, get_session

board:APIRouter = APIRouter(prefix="/board", dependencies=[Depends(get_session)])

@board.get("/all")
async def get_all_board(sess: AsyncSession = Depends(get_session)):
    stmt = select(Board)
    return (await sess.exec(stmt)).all()

#TODO: 获取单个板块的信息

#TODO: 获取一个板块下的帖子