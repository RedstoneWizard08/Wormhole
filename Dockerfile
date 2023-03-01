FROM rust:1.67.1-bullseye as builder

RUN apt-get update && \
    apt-get -y upgrade

RUN apt-get -y install build-essential \
    git g++ make cmake ninja-build curl \
    wget libssl-dev libgtk-3-dev tar xz-utils \
    libayatana-appindicator3-dev librsvg2-dev \
    zip unzip bash gcc-mingw-w64-x86-64 \
    g++-mingw-w64-x86-64 gcc-mingw-w64-i686 \
    g++-mingw-w64-i686

SHELL [ "/bin/bash", "-c" ]

ARG NODE_VERSION=18.14.2

RUN _ARCH=$(uname -m) && \
    case $_ARCH in \
        x86_64) ARCH=x64 ;; \
        aarch64) ARCH=arm64 ;; \
        *) echo "Unknown arch!"; exit 1 ;; \
    esac && \
    wget -qO nodejs.tar.xz \
        https://nodejs.org/dist/v${NODE_VERSION}/node-v${NODE_VERSION}-linux-$ARCH.tar.xz

RUN tar -xJvf nodejs.tar.xz
RUN mv node-v${NODE_VERSION}-linux-* node
RUN cp -r node/* /usr
RUN rm -rf node-v${NODE_VERSION}-linux-* nodejs.tar.xz

RUN npm install --global pnpm

RUN rustup target add x86_64-pc-windows-gnu \
    x86_64-pc-windows-msvc i686-pc-windows-gnu \
    i686-pc-windows-msvc aarch64-apple-darwin \
    x86_64-apple-darwin aarch64-unknown-linux-gnu \
    i686-unknown-linux-gnu x86_64-unknown-linux-gnu

ADD . /app
WORKDIR /app

RUN pnpm install

FROM alpine

RUN pnpm build --target x86_64-pc-windows-gnu
RUN pnpm build --target x86_64-pc-windows-msvc
RUN pnpm build --target i686-pc-windows-gnu
RUN pnpm build --target i686-pc-windows-msvc

RUN pnpm build --target x86_64-apple-darwin
RUN pnpm build --target aarch64-apple-darwin

RUN pnpm build --target x86_64-unknown-linux-gnu
RUN pnpm build --target i686-unknown-linux-gnu
RUN pnpm build --target aarch64-unknown-linux-gnu

FROM alpine
