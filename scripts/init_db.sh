#/usr/bin/env bash
set -x
set -eo pipefail

#if ! [ -x "$(*command -v psql)" ]; then
#  echo >&2 "Error: psql is not installed."
#  exit 1
#fi
#
#if ! [ -x "$(command -v sqlx)" ]; then
#  echo >&2 "Error: sqlx is not installed."
#  echo >&2 "Use:"
#  echo >&2 "    cargo install --version='~0.8' sqlx-cli --no-default-features --features rustls,postgres"
#  echo >&2 "to install it."
#  exit 1
#fi

DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"

# docker 사용하여 postgres 구동
# 도커화된 Postgres 데이터베이스가 이미 실행중이면 도커가 이 단계 건너뛰게 함
if [[ -z "${SKIP_DOCKER}" ]]
then

  docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000
    #  ^ 테스트 목적으로 증가시킨 커넥션 수
fi

# postgres가 준비될 때까지 핑 유지
set PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}"

# Create the application database
DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
set DATABASE_URL
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go"
