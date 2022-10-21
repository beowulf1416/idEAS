FROM rust:latest as builder
LABEL maintainer="Ferd Tomale<beowulf1416@gmail.com>"

WORKDIR /opt/ideas
COPY . .

RUN cargo install --root /opt/ideas --path api
RUN cargo install --root /opt/ideas --path consumers/qmailer

FROM debian:buster-slim
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y libssl-dev git

WORKDIR /opt/extras
RUN git clone https://github.com/vishnubob/wait-for-it.git


COPY --from=builder /opt/ideas/bin /opt/ideas/bin
COPY --from=builder /opt/ideas/etc /opt/ideas/etc
COPY --from=builder /opt/ideas/wrapper.sh /opt/ideas/bin/wrapper.sh

RUN chmod ugo+x /opt/ideas/bin/wrapper.sh

EXPOSE 8081/tcp
EXPOSE 8081/udp

ENV RUST_LOG=debug \
    CFG=/opt/ideas/etc/config.json

CMD [ "/opt/extras/wait-for-it/wait-for-it.sh", "-t", "5", "kafka:9093", "--", "/opt/ideas/bin/wrapper.sh"]