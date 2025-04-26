SHELL := /bin/bash
.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort  | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

clean: ## Clean the project using cargo
		cargo clean

build: ## Build the project using cargo
		cargo build

lint: ## Run clippy
		@rustup component add clippy 2> /dev/null
		cargo clippy --all-targets --all-features -- -D warnings

fmt: ## Format the project using rustfmt
		@rustup component add rustfmt 2> /dev/null
		cargo fmt

bump: ## Bump the version using cargo
		@echo "Current version: $(shell cargo pkgid | cut -d# -f2)"
		@read -p "Enter new version: " new_version; \
		update_version=$$(cargo pkgid | cut -d# -f2); \
		sed -i -E "s/^version = .*/version = \"$$new_version\"/" Cargo.toml
		@echo "Version updated to: $(shell cargo pkgid | cut -d# -f2)"