.DEFAULT_GOAL := all

.PHONY: 1
1: 
	cargo test --manifest-path=day1/Cargo.toml

.PHONY: all
all: 1
