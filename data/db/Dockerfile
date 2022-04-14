FROM postgres:latest
LABEL maintainer="Ferd Tomale<beowulf1416@gmail.com>"

ENV TZ=Etc/UTC \
    POSTGRES_USER=app_user \
    POSTGRES_PASSWORD=replace_me \
    POSTGRES_DB=app

EXPOSE 5432

RUN mkdir -p /docker-entrypoint-initdb.d
# COPY ./initdb.sh /docker-entrypoint-initdb.d/initdb.sh
# COPY ./update-postgis.sh /usr/local/bin
COPY ./sql /docker-entrypoint-initdb.d/