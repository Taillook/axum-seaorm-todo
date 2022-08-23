FROM debian:stable

ENV DEBIAN_FRONTEND=noninteractive \
    LC_CTYPE=ja_JP.utf8 \
    LANG=ja_JP.utf8 \
    SQLDEF_VERSION=v0.11.15

RUN apt-get update && \
    apt-get install -y -q \
    ca-certificates \
    locales \
    libpq-dev \
    gnupg \
    apt-transport-https\
    libssl-dev \
    pkg-config \
    curl \
    build-essential \
    git \
    wget && \
    echo "ja_JP UTF-8" > /etc/locale.gen && \
    locale-gen && \
    echo "install sqldef" && \
    curl -L -O https://github.com/k0kubun/sqldef/releases/download/${SQLDEF_VERSION}/mysqldef_linux_amd64.tar.gz && \
    tar xf mysqldef_linux_amd64.tar.gz && \
    rm mysqldef_linux_amd64.tar.gz && \
    mv mysqldef /usr/local/bin
