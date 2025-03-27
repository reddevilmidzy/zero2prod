-- Add migration script here
-- subscriptions 테이블 생성
CREATE TABLE subscriptions
(
    id            uuid        NOT NULL,
    PRIMARY KEY (id),
    email         TEXT        NOT NULL UNIQUE,
    name          TEXT        NOT NULL,
    subscribed_at timestamptz NOT NULL
)
