FROM rust:latest as builder
LABEL maintainer="Ferd Tomale<beowulf1416@gmail.com>"

WORKDIR /opt/ideas
COPY . .

RUN cargo install --root /opt/ideas/bin --path .

FROM debian:buster-slim
RUN apt-get update && \
    apt-get install -y extra-runtime-dependencies

COPY --from=builder /usr/local/cargo/bin/ideas /opt/ideas/bin
COPY secrets/config.json /opt/ideas/etc/config.json

ENV RUST_LOG=debug \
    CFG=/opt/ideas/etc/ideas/config.json

CMD ["/opt/ideas/bin/api", "/opt/ideas/bin/qmailer"]