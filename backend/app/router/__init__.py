from fastapi import APIRouter

from app.router.board import board
from app.router.post import post
from app.router.user import user

root = APIRouter(prefix="/api")
root.include_router(board)
root.include_router(post)
root.include_router(user)
