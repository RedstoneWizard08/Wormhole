build:
	pnpm build

build-linux-arm64:
	pnpm build --target aarch64-unknown-linux-gnu

build-linux-arm:
	pnpm build --target arm-unknown-linux-gnueabihf

build-macos-arm64:
	pnpm build --target aarch64-apple-darwin
