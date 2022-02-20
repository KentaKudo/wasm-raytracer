.PHONY: build
build:
	wasm-pack build

.PHONY: publish
publish: build
	wasm-pack publish
