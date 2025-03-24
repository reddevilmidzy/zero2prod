#!/usr/bin/env bash
set -x
set -eo pipefail

DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"

# docker 사용하여 postgres 구동
docker run \
   -e POSTGRES_USER=${DB_USER} \
   -e POSTGRES_PASSWORD=${DB_PASSWORD} \
   -e POSTGRES_DB=${DB_NAME} \
   -p "${DB_PORT}":5432 \
   -d postgres \
   postgres -N 1000
   #  ^ 테스트 목적으로 증가시킨 커넥션 수

# postgres가 준비될 때까지 핑 유지
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}"

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
export DATABASE_URL
sqlx database create
