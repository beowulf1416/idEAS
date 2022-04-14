#!/bin/bash
set -e
set -x


psql -e -V ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dname "$POSTGRES_DB" \
    -f ./sql/create.sql \
    -f ./sql/create_app_role.sql