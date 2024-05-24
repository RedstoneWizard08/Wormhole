vite := "node_modules/.bin/vite"
tsc := "node_modules/.bin/tsc"
tauri := "node_modules/.bin/tauri"
prettier := "node_modules/.bin/prettier"
eslint := "node_modules/.bin/eslint"
stylelint := "node_modules/.bin/stylelint"
ts_node := "node_modules/.bin/ts-node"
svelte_kit := "node_modules/.bin/svelte-kit"
svelte_check := "node_modules/.bin/svelte-check"
serve := "node_modules/.bin/serve"
wormhole := "target/debug/wormhole"
cargo := "cargo"

set windows-shell := ["powershell.exe", "-c"]

bin:
    cargo build --bin wormhole

bin_release:
    cargo build --bin wormhole --release

web_dev: bin
    {{ wormhole }} server

[private]
_web_build:
    {{ tsc }}
    {{ vite }} build

web_build: _web_build bin_release

dev:
    {{ tauri }} dev

build:
    {{ tauri }} build

fmt:
    {{ prettier }} --write .
    {{ cargo }} fmt

lint_eslint:
    {{ eslint }} --ext .ts,.tsx,.svelte .

lint_stylelint:
    {{ stylelint }} **/*.scss **/*.css

lint_clippy:
    {{ cargo }} clippy

lint: lint_eslint lint_stylelint lint_clippy

check:
    {{ svelte_kit }} sync
    {{ svelte_check }}

watch_check:
    {{ svelte_kit }} sync
    {{ svelte_check }} --watch

sync:
    {{ svelte_kit }} sync

bindgen: bin
    {{ wormhole }} bindgen
