# Builder stage: 컴파일된 바이너리를 생성
FROM rust:1.85.0 As builder

WORKDIR /app

RUN apt update && apt install lld clang -y

COPY . .

ENV SQLX_OFFLINE true

RUN cargo build --release

# Runtime stage: 바이너리를 실행
FROM rust:1.85.0-slim As runtime

WORKDIR /app

# 컴파일된 바이너리를 builder 환경에서 runtime 환경으로 복사
COPY --from=builder /app/target/release/zero2prod zero2prod

COPY configuration configuration

ENV APP_ENVIRONMENT production

ENTRYPOINT ["./target/release/zero2prod"]
