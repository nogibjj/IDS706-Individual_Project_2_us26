# Variables
CARGO = cargo

# Targets
.PHONY: all build test format lint clean

all: build

build:
	$(CARGO) build

test:
	$(CARGO) test

format:
	$(CARGO) fmt -- --check

lint:
	$(CARGO) clippy -- -D warnings

clean:
	$(CARGO) clean