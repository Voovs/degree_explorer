# Usage:
# make 			# Build assets (wasm, sass, and rollup)
# make run 		# Build assets then start server

.PHONY = build build_wasm rollup run

SASS_DIR = static
CSS_TARGETS = $(patsubst %.sass, %.css, $(wildcard $(SASS_DIR)/*.sass))

build : build_wasm $(CSS_TARGETS) rollup

%.css : %.sass
	@sass $? $@

build_wasm:
	@wasm-pack build --target web

rollup:
	@rollup ./main.js --format iife --file ./pkg/bundle.js

run : build
	@miniserve . --index index.html --port 17701
