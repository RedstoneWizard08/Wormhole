#!/bin/bash

set -xe

if [[ "$(uname)" == *"Darwin"* ]]; then
    pip3 install yq --break-system-packages
else
    pip3 install yq
fi

DIR="$(dirname "$(realpath "$0")")"

cd "$DIR"

VER="$(cat ../../Cargo.toml | tomlq -r '.workspace.package.version')"

CARGO_MANIFEST_DIR="$DIR" \
CARGO_PKG_VERSION="$VER" \
    rustc "conf.rs" -o conf

./conf
rm conf
