-- subscriptions 테이블 생성
-- set DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/newsletter
-- 위 명령어로 주소 설정하고
-- sqlx migrate add create_subscriptions_table
-- 위 명령어 입력하면 이 sql 파일이 생김. 그러면 스키마 입력하고,
-- sqlx migrate run
-- 위 명령어 실행하면 테이블이 생성됨.

CREATE TABLE subscriptions
(
    id            uuid        NOT NULL,
    PRIMARY KEY (id),
    email         TEXT        NOT NULL UNIQUE,
    name          TEXT        NOT NULL,
    subscribed_at timestamptz NOT NULL
)
