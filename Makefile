# ------------------------
# Config
# ------------------------

APP_NAME := game
WASM_TARGET := wasm32-unknown-unknown
WASM_OUT_DIR := src/lib/assets/wasm

DEBUG_WASM := target/$(WASM_TARGET)/debug/$(APP_NAME).wasm
RELEASE_WASM := target/$(WASM_TARGET)/release/$(APP_NAME).wasm
BOUND_WASM := $(WASM_OUT_DIR)/$(APP_NAME)_bg.wasm

.PHONY: dev build preview check check-watch lint format prepare \
        dev-wasm build-wasm \
        dev-generate-wasm-bindings build-generate-wasm-bindings \
        build-optimize-wasm clean

# ------------------------
# High-level targets
# ------------------------

dev: dev-wasm dev-generate-wasm-bindings
	pnpm exec vite dev

build: build-wasm build-generate-wasm-bindings build-optimize-wasm
	pnpm exec vite build

preview:
	sirv build --host --single --gzip --brotli

# ------------------------
# Tooling / checks
# ------------------------

prepare:
	svelte-kit sync || true

check:
	svelte-kit sync
	svelte-check --tsconfig ./tsconfig.json

check-watch:
	svelte-kit sync
	svelte-check --tsconfig ./jsconfig.json --watch

lint:
	prettier --check .
	eslint .

format:
	prettier --write .

# ------------------------
# WASM pipeline
# ------------------------

dev-wasm:
	cargo build --bin $(APP_NAME) --target $(WASM_TARGET)

build-wasm:
	cargo build --release --bin $(APP_NAME) --target $(WASM_TARGET)

dev-generate-wasm-bindings: dev-wasm
	wasm-bindgen \
		--out-dir $(WASM_OUT_DIR) \
		--target web \
		$(DEBUG_WASM)

build-generate-wasm-bindings: build-wasm
	wasm-bindgen \
		--out-dir $(WASM_OUT_DIR) \
		--target web \
		$(RELEASE_WASM)

build-optimize-wasm:
	wasm-opt \
		--strip-producers \
		--enable-mutable-globals \
		--enable-sign-ext \
		--enable-bulk-memory \
		--enable-threads \
		--enable-nontrapping-float-to-int \
		-Oz \
		--converge \
		--strip-debug \
		-o $(BOUND_WASM) \
		$(BOUND_WASM)

# ------------------------
# Cleanup
# ------------------------

clean:
	cargo clean
	rm -rf $(WASM_OUT_DIR)
