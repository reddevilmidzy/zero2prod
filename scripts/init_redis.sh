#!/usr/bin/env bash
set -x
set -eo pipefail

RUNNING_CONTAINER=$(docker ps --filter 'name=redis' --format '{{.ID}}')

# 레디스 컨테이너가 실행중이라면, 종료를 위한 명령을 출력하고 중지한다.
if [[ -n $RUNNING_CONTAINER ]]; then
  echo >&2 "there is a redis container already running, kill it with"
  echo >&2 "docker kill ${RUNNING_CONTAINER}"
  exit 1
fi

# 도커를 사용해서 레디스를 구동한다.
docker run \
  -p "6379:6379" \
  -d \
  --name "redis_$(date '+%s')" \
  redis:6

>&2 echo "Redis is ready to go!"
