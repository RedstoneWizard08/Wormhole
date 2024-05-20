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
cargo := "cargo"

set shell := ["powershell.exe", "-c"]

dev_app:
    {{ vite }}

build_app:
    {{ tsc }}
    {{ vite }} build

preview_app:
    {{ vite }} preview

dev:
    {{ tauri }} dev

build:
    {{ tauri }} build

tauri:
    {{ tauri }}

fmt:
    {{ prettier }} --write .
    {{ cargo }} fmt

web_dev:
    $env:TAURI_WEB_DEV=1
    {{ vite }}

web_build:
    $env:TAURI_WEB_DEV=1
    {{ vite }} build

lint_eslint:
    {{ eslint }} --ext .ts,.tsx,.svelte .

lint_stylelint:
    {{ stylelint }} **/*.scss **/*.css

lint_clippy:
    {{ cargo }} clippy

lint: lint_eslint lint_stylelint lint_clippy

console:
    {{ ts_node }} scripts/console.ts

check:
    {{ svelte_kit }} sync
    {{ svelte_check }}

watch_check:
    {{ svelte_kit }} sync
    {{ svelte_check }} --watch

postinstall:
    {{ svelte_kit }} sync

serve:
    {{ serve }} -p 4000 build

bindgen:
    {{ cargo }} run --bin bindings
