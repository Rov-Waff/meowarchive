from fastapi import APIRouter

from app.router.board import board

root = APIRouter(prefix="/api")
root.include_router(board)