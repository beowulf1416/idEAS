FROM ubuntu:latest
LABEL maintainer="Ferd Tomale<beowulf1416@gmail.com>"

# https://github.com/actix/examples/blob/master/docker/Dockerfile

ENV TZ=Etc/UTC \
    RUST_LOG=debug \
    URL_DB_FILE= \
    URL_DB=prod \
    CERT_FILE=cert.pem \
    KEY_FILE=key.pem \
    JWT_SECRET_FILE=

WORKDIR /opt/erp/services

COPY ./target/debug/api /opt/erp/services/api
COPY ./target/debug/api.d /opt/erp/services/api.d
COPY ./target/debug/libcommon.d /opt/erp/services/libcommon.d
COPY ./target/debug/libcommon.rlib /opt/erp/services/libcommon.rlib

EXPOSE 8081/tcp

CMD [ "/opt/erp/services/api" ]