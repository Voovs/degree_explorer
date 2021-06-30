# Usage:
# make 			# Build assets (wasm, sass, and rollup)
# make run 		# Build assets then start server

.PHONY = build run

SASS_DIR = static
CSS_TARGETS = $(patsubst %.sass, %.css, $(wildcard $(SASS_DIR)/*.sass))

build : $(CSS_TARGETS)
	@wasm-pack build --target web
	@rollup ./main.js --format iife --file ./pkg/bundle.js

%.css : %.sass
	@sass $? $@

run : build
	@miniserve . --index index.html --port 17701
