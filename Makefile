# Targets
.PHONY: all build test format lint clean

all: build

build:
	cargo build --release

test:
	cargo test --quiet

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

clean:
	cargo clean

# Generate and push changes to GitHub
generate_and_push:
	@if [ -n "$$(git status --porcelain)" ]; then \
		git config --local user.email "action@github.com"; \
		git config --local user.name "GitHub Action"; \
		git add .; \
		git commit -m "Add query log"; \
		git push; \
	else \
		echo "No changes to commit. Skipping commit and push."; \
	fi