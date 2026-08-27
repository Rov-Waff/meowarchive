FROM docker.1ms.run/library/rust:1.98-slim-bookworm AS builder
WORKDIR /app
COPY Cargo.toml ./
COPY Cargo.lock ./
COPY src ./src

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
ENV CARGO_REGISTRIES_CRATES_IO_REPLACE_WITH=rsproxy
ENV CARGO_REGISTRIES_RSPROXY_INDEX=https://rsproxy.cn/index
#ENV SWAGGER_UI_DOWNLOAD_URL=https://gh.jasonzeng.dev/https://github.com/swagger-api/swagger-ui/archive/refs/tags/v5.17.14.zip
RUN apt-get update && \
    apt-get install -y curl 
RUN ["cargo", "build", "--release"]

FROM docker.1ms.run/library/debian:bookworm-slim
WORKDIR /app

COPY --from=builder /app/target/release/goat-backend .
CMD ["./goat-backend"]
EXPOSE 8080
