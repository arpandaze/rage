#!/usr/bin/env bash
set -x
set -eo pipefail


DB_USER=${POSTGRES_USER:=postuser}
DB_PASSWORD="${POSTGRES_PASSWORD:=postpass}"
DB_NAME="${POSTGRES_DB:=actix}"
DB_PORT="${POSTGRES_PORT:=5432}"


if [ "$(podman ps -q -f name=actix_postgres)" ]; then
    echo "Postgres already running!"
    exit
fi

if [ "$(podman ps -aq -f name=actix_postgres)" ]; then
    echo "Launching existing postgres container!"
    podman start actix_postgres 
else
    echo "Creating new postgres container!"
    podman run --name actix_postgres \
      -e POSTGRES_USER=${DB_USER} \
      -e POSTGRES_PASSWORD=${DB_PASSWORD} \
      -e POSTGRES_DB=${DB_NAME} \
      -p "${DB_PORT}":5432 \
      -d postgres:13-alpine \
      postgres -N 1000
fi

# Keep pinging Postgres until it's ready to accept commands
until PGPASSWORD="${DB_PASSWORD}" psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
cargo sqlx database create
cargo sqlx migrate run --database-url postgres://postuser:postpass@localhost:5432/actix
