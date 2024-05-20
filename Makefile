# MIT License
#
# Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
#
# Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

all:
	@echo "[INFO] Building app..."
	@cargo build

	@echo "[INFO] Building plugins (1/4)..."
	@cargo build --target wasm32-wasi -p example-plugin

	@echo "[INFO] Building plugins (2/4)..."
	@cargo build --target wasm32-wasi -p plugin_ksp1

	@echo "[INFO] Building plugins (3/4)..."
	@cargo build --target wasm32-wasi -p plugin_ksp2

	@echo "[INFO] Building plugins (4/4)..."
	@cargo build --target wasm32-wasi -p plugin_minecraft

release:
	@echo "[INFO] Building app..."
	@cargo build --release

	@echo "[INFO] Building plugins (1/4)..."
	@cargo build --target wasm32-wasi -p example-plugin --release

	@echo "[INFO] Building plugins (2/4)..."
	@cargo build --target wasm32-wasi -p plugin_ksp1 --release

	@echo "[INFO] Building plugins (3/4)..."
	@cargo build --target wasm32-wasi -p plugin_ksp2 --release

	@echo "[INFO] Building plugins (4/4)..."
	@cargo build --target wasm32-wasi -p plugin_minecraft --release
