#!/usr/bin/env bash
# Script to initialize Postgres database on Docker. Uses SQLX for migrations.
set -x
set -eo pipefail

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed"
    echo >&2 "Use:"
    echo >&2 "  cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres"
    echo >&2 "to install it."
    exit 1
fi

if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: psql is not installed"
    exit 1
fi

if ! [ -x "$(command -v docker)" ]; then
    echo >&2 "Error: docker is not installed"
    exit 1
fi

#Check if custom user has been set, otherwise default to 'postgres'
DB_USER=${POSTGRES_USER:=postgres}
#Check if custom password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
#Check if custom database name has been set, otherwise default to 'newsletter'
DB_NAME="${POSTGRES_DB:=newsletter}"
#Check if a custom port has been set, otherwise default to '5432', used for port forwarding in Docker.
DB_PORT="${POSTGRES_PORT:=5432}"

# Allow to skip docker if Postgres database is already running on docker
if [[ -z "${SKIP_DOCKER}" ]]; then
    echo >&2 "Creating a new Postgres Container"
    #Launch Postgres using Docker
    docker run \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_DB=${DB_NAME} \
        -p "${DB_PORT}":5432 \
        -d postgres \
        postgres -N 1000
fi
echo >&2 "Docker Launched, will try to ping Postgres now"

#Keep pinging Postgres until it's ready to accept commands
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    echo >&2 "Postgres is still unavailable - sleeping"
    sleep 1
done
echo >&2 -e "\e[32mPostgres is up and running on port ${DB_PORT} - Running migrations now!\e[0m"

export DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}"
sqlx database create
sqlx migrate run

echo >&2 "Postgres has been migrated, ready to go!"
