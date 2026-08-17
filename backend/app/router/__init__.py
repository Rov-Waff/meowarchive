from fastapi import APIRouter

from app.router.board import board
from app.router.post import post

root = APIRouter(prefix="/api")
root.include_router(board)
root.include_router(post)
