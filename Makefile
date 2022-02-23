.PHONY: build
build:
	wasm-pack build

.PHONY: build\:profiling
build\:profiling:
	wasm-pack build --profiling

.PHONY: test
test:
	wasm-pack test --firefox --headless

.PHONY: publish
publish: build
	wasm-pack publish
