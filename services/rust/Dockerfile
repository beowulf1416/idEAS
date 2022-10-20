FROM rust:latest as builder
LABEL maintainer="Ferd Tomale<beowulf1416@gmail.com>"

WORKDIR /opt/ideas
COPY . .

RUN cargo install --root /opt/ideas --path api
RUN cargo install --root /opt/ideas --path consumers/qmailer

FROM debian:buster-slim
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y libssl-dev
#     apt-get install -y extra-runtime-dependencies

COPY --from=builder /opt/ideas/bin /opt/ideas/bin

ENV RUST_LOG=debug \
    CFG=/run/secrets/config.json

CMD ["/opt/ideas/bin/api", "/opt/ideas/bin/qmailer"]