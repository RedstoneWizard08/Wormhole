#!/usr/bin/env bash

set -x
set -euo pipefail

# shellcheck disable=SC1091
. lib.sh

# For architectures except amd64 and i386, look for packages on ports.ubuntu.com instead.
# This is important if you enable additional architectures so you can install libraries to cross-compile against.
# Look for 'dpkg --add-architecture' in the README for more details.

install_packages \
    autoconf \
    automake \
    binutils \
    ca-certificates \
    curl \
    file \
    gcc \
    git \
    libtool \
    m4 \
    make

if_centos install_packages \
    clang-devel \
    gcc-c++ \
    glibc-devel \
    pkgconfig

if_ubuntu install_packages \
    g++ \
    libc6-dev \
    libclang-dev \
    pkg-config
