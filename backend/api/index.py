"""Vercel Python Function 入口：暴露 FastAPI ASGI 应用。

部署时 Vercel 项目 Root Directory 设为 backend/，所有 /api/* 请求都会
进入这个函数，由 FastAPI 的 root router（prefix="/api"）接管。
依赖安装读取 requirements.txt（Vercel 不识别 pyproject.toml）。
"""

from app import app
