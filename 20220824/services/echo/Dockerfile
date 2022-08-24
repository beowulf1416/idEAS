FROM ubuntu:latest
LABEL maintainer="Ferd Tomale<beowulf1416@gmail.com>"

# https://github.com/actix/examples/blob/master/docker/Dockerfile

ENV TZ=Etc/UTC \
    RUST_LOG=debug

WORKDIR /opt/erp/services

COPY ./target/debug/echo /opt/erp/services/echo
COPY ./target/debug/echo.d /opt/erp/services/echo.d

EXPOSE 8081/tcp

CMD [ "/opt/erp/services/echo" ]