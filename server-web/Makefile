SHELL := /bin/bash

CARGO_MANIFEST_PATH = "Cargo.toml"

export

help: help-all help-rust help-python
	@echo ""
	@echo "$(SMUL)$(BOLD)$(GREEN)Combined targets$(NC)$(SGR0)"
	@echo "  $(BOLD)test$(SGR0) -- Run targets; rust-check rust-test rust-clippy rust-fmt toml-test python-test"
	@echo "  $(BOLD)tidy$(SGR0) -- Run targets; rust-tidy toml-tidy python-tidy"
	@echo "  $(BOLD)all$(SGR0)  -- Run targets; test build release"

include ../.make/targets-all.mk
include ../.make/targets-rust.mk
include ../.make/targets-python.mk

# Combined targets
test: rust-check rust-test rust-clippy rust-fmt toml-test python-test
tidy: rust-tidy toml-tidy python-tidy
all: test build release
