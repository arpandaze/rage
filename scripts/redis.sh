#!/usr/bin/env bash
set -x
set -eo pipefail

if [ "$(podman ps -q -f name=actix_redis)" ]; then
    echo "Redis already running!"
    exit
fi

if [ "$(podman ps -aq -f name=actix_redis)" ]; then
    echo "Launching existing redis container!"
    podman start actix_redis
    exit
fi

echo "Creating new redis container!"

podman run --name actix_redis \
  -h redis \
  -e REDIS_PASSWORD=redispass \
  -p 6379:6379 \
  -d redis:6-alpine /bin/sh -c 'redis-server --requirepass ${REDIS_PASSWORD}'
