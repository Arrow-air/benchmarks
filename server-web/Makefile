DOCKER_NAME := arrow-benchmarks-server-web

CARGO_MANIFEST_PATH = "Cargo.toml"

help: .help-base .help-rust .help-python
	@echo ""
	@echo "$(SMUL)$(BOLD)$(GREEN)Combined targets$(NC)$(SGR0)"
	@echo "  $(BOLD)test$(SGR0) -- Run targets; rust-check rust-test rust-clippy rust-fmt toml-test python-test"
	@echo "  $(BOLD)tidy$(SGR0) -- Run targets; rust-tidy toml-tidy python-tidy"
	@echo "  $(BOLD)all$(SGR0)  -- Run targets; test build release"

include ../.make/base.mk
include ../.make/rust.mk
include ../.make/toml.mk
include ../.make/python.mk

# Combined targets
build: rust-build
release: rust-release
test: rust-test-all toml-test python-test
tidy: rust-tidy toml-tidy python-tidy
all: test build release
