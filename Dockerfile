# ------------------------------------------------------------------------------
# Создаем образ для сборки приложения
# дополнительная информация https://shaneutt.com/blog/rust-fast-small-docker-image-builds/
#
# ------------------------------------------------------------------------------
FROM rust:latest as cargo-build

RUN rustup update stable
WORKDIR /usr/src/audit-res
RUN apt-get update && \
  apt-get upgrade -y && \
  apt-get install -y \
  ca-certificates \
  musl-dev \
  musl-tools \
  file \
  nano \
  git \
  zlib1g-dev \
  cmake \
  make \
  clang \
  curl \
  pkgconf \
  linux-headers-amd64 \
  xutils-dev \
  libpq-dev \
  libssl-dev \
  librdkafka-dev \
  --no-install-recommends && \
  rm -rf /var/lib/apt/lists/*

# Клиент для MQ последняя версия
RUN mkdir MQR

WORKDIR /usr/src/audit-res/MQR
RUN wget https://public.dhe.ibm.com/ibmdl/export/pub/software/websphere/messaging/mqdev/redist/9.1.5.0-IBM-MQC-Redist-LinuxX64.tar.gz && \
  tar -xvf 9.1.5.0-IBM-MQC-Redist-LinuxX64.tar.gz

RUN rm -f 9.1.5.0-IBM-MQC-Redist-LinuxX64.tar.gz

WORKDIR /usr/src/audit-res

ENV DYLD_LIBRARY_PATH=/usr/src/audit-res/MQR/lib64/

COPY Cargo.toml .
COPY build.rs .
RUN mkdir /usr/src/audit-res/src
COPY src /usr/src/audit-res/src/

RUN cargo build --release
RUN cargo install --path .

CMD ["/bin/bash"]

# ------------------------------------------------------------------------------
# Создаем конечный образ с нашим приложением
# ------------------------------------------------------------------------------
#FROM ubuntu:latest
#RUN apt-get update -y && apt-get upgrade -y
#RUN apt-get install -y \
#  ca-certificates \
#  librdkafka++1 \
##  openssl \
##  librdkafka-dev \
#  wget
##  libssl-dev \
##  zlib1g-dev
#
#WORKDIR /usr/local/lib/MQR
#
##COPY --from=cargo-build /usr/local/cargo/bin/mq2kafka .
##COPY --from=cargo-build /usr/src/audit-res/MQR ./MQR
##COPY config/application.conf .
#
#RUN wget https://public.dhe.ibm.com/ibmdl/export/pub/software/websphere/messaging/mqdev/redist/9.1.5.0-IBM-MQC-Redist-LinuxX64.tar.gz && \
#  tar -xvf 9.1.5.0-IBM-MQC-Redist-LinuxX64.tar.gz
#
#RUN rm -f 9.1.5.0-IBM-MQC-Redist-LinuxX64.tar.gz
#
#RUN rm -rf 9.1.5.0-IBM-MQC-Redist-LinuxX64.tar.gz
#RUN rm -rf inc
#RUN rm -rf java
#RUN rm -rf bin
#
#RUN apt-get purge -y wget
#
#ENV LD_LIBRARY_PATH=/lib:/usr/lib:/usrlocal/lib:/usr/local/lib/MQR/lib64
#
#CMD ["/bin/bash"]