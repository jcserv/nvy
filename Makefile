.PHONY: build test init use

build:
	cargo build

test:
	cargo test

init:
	cargo run init

use:
	cargo run use default