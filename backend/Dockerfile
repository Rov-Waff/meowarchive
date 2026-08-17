# ---------- 构建阶段：安装依赖 ----------
FROM astral/uv:python3.13-bookworm-slim AS builder
WORKDIR /app
ENV UV_COMPILE_BYTECODE=1 \
    UV_LINK_MODE=copy

ENV UV_INDEX_URL="https://pypi.tuna.tsinghua.edu.cn/simple" \
    UV_EXTRA_INDEX_URL="https://mirrors.aliyun.com/pypi/simple"
# 先复制依赖清单，利用 Docker 层缓存
COPY pyproject.toml uv.lock ./
RUN uv sync --frozen --no-dev --no-install-project

# 复制应用代码
COPY app ./app

# ---------- 运行阶段 ----------
FROM python:3.13-slim

WORKDIR /app
ENV PATH="/app/.venv/bin:$PATH"

COPY --from=builder /app/.venv /app/.venv
COPY --from=builder /app/app /app/app

EXPOSE 8000
CMD ["uvicorn", "app:app", "--host", "0.0.0.0", "--port", "8000"]
