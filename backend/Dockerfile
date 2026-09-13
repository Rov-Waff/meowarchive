FROM docker.1ms.run/library/rust:1.98-slim-bookworm AS builder
WORKDIR /app
COPY Cargo.toml ./
COPY Cargo.lock ./
COPY src ./src

# 配置 crates.io 走 rsproxy（写文件比环境变量更稳）
RUN mkdir -p /usr/local/cargo && \
    printf '%s\n' \
    '[source.crates-io]' \
    'replace-with = "rsproxy"' \
    '' \
    '[source.rsproxy]' \
    'registry = "sparse+https://rsproxy.cn/index/"' \
    '' \
    '[registries.rsproxy]' \
    'index = "sparse+https://rsproxy.cn/index/"' \
    '' \
    '[net]' \
    'git-fetch-with-cli = true' \
    > /usr/local/cargo/config.toml

# 替换 Debian apt 源为清华镜像
RUN sed -i 's|deb.debian.org|mirrors.tuna.tsinghua.edu.cn|g' /etc/apt/sources.list.d/debian.sources 2>/dev/null || \
    sed -i 's|deb.debian.org|mirrors.tuna.tsinghua.edu.cn|g' /etc/apt/sources.list

RUN apt-get update && \
    apt-get install -y --no-install-recommends curl && \
    rm -rf /var/lib/apt/lists/*

RUN ["cargo", "build", "--release"]

FROM docker.1ms.run/library/debian:bookworm-slim
WORKDIR /app

COPY --from=builder /app/target/release/goat-backend .
CMD ["./goat-backend"]
EXPOSE 8080
