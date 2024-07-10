
#!/usr/bin/env bash

set -x
set -eo pipefail

# Check if a custom user has been set, otherwise default to 'postgres'
DB_USER=${POSTGRESS_USER:=postgres}

# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRESS_PASSWORD:=password}"

# Check if a custom database name has been set, otherwise default to 'newsletter'
DB_NAME="${POSTGRESS_DB:=newsletter}"

# Check if a custom port has been set, otherwise default to '5432'
DB_PORT="${POSTGRESS_PORT:=5432}"

# Check if a custom host has been set, otherwise default to 'localhost'
DB_HOST="${POSTGRESS_HOST:=localhost}"

# Launch postgres using Docker
docker run \
    -e POSTGRESS_USER={DB_USER} \
    -e POSTGRESS_PASSWORD={DB_PASSWORD} \
    -e POSTGRESS_DB={DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000
# ^ Increased maximum number of connections for testing purposes
