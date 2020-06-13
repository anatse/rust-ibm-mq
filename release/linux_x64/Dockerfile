FROM rust-base:latest

WORKDIR /opt/audit
COPY mq2kafka .
COPY application.conf .

CMD ["mq2kafka", "application.conf"]