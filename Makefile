.PHONY: build
build:
	wasm-pack build

.PHONY: test
test:
	wasm-pack test --firefox --headless

.PHONY: publish
publish: build
	wasm-pack publish
