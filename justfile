# MIT License
#
# Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
#
# Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

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
