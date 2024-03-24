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
