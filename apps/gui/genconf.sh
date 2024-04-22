#!/bin/bash

set -xe

pip3 install yq

DIR="$(dirname "$(realpath "$0")")"

cd "$DIR"

VER="$(cat ../../Cargo.toml | tomlq -r '.workspace.package.version')"

CARGO_MANIFEST_DIR="$DIR" \
CARGO_PKG_VERSION="$VER" \
    rustc "conf.rs" -o conf

./conf
rm conf
