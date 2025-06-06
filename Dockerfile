# Builder stage: 컴파일된 바이너리를 생성
FROM rust:1.85.0 As builder

WORKDIR /app

RUN apt update && apt install lld clang -y

COPY . .

ENV SQLX_OFFLINE true

RUN cargo build --release

# Runtime stage: 바이너리를 실행
# 가장 원시적인 운영체제
FROM debian:bullseye-slim As runtime

WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# 컴파일된 바이너리를 builder 환경에서 runtime 환경으로 복사
COPY --from=builder /app/target/release/zero2prod zero2prod

COPY configuration configuration

ENV APP_ENVIRONMENT production

ENTRYPOINT ["./target/release/zero2prod"]
