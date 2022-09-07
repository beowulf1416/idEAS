FROM postgres:latest
LABEL maintainer="Ferd Tomale<beowulf1416@gmail.com>"

ENV TZ=Etc/UTC \
    POSTGRES_USER=app_user \
    POSTGRES_PASSWORD=replace_me \
    POSTGRES_DB=app

EXPOSE 5432


RUN mkdir -p /docker-entrypoint-initdb.d
# COPY ./csv /docker-entrypoint-initdb.d/
COPY ./sql/postgres /docker-entrypoint-initdb.d/