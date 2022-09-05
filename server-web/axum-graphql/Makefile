SHELL := /bin/bash

PACKAGE_NAME     := axum-graphql
DOCKER_NAME      := arrow-benchmarks-$(PACKAGE_NAME)
IMAGE_NAME       := benchmarks-$(PACKAGE_NAME)
BUILD_IMAGE_NAME := ghcr.io/arrow-air/tools/arrow-rust
BUILD_IMAGE_TAG  := latest
DOCKER_PORT      := 8080
HOST_PORT        := 8080

# Style templates for console output.
GREEN  := $(shell echo -e `tput setaf 2`)
YELLOW := $(shell echo -e `tput setaf 3`)
BOLD   := $(shell echo -e `tput bold`)
NC     := $(shell echo -e $$'\e[0m')

# We might not have a Cargo.toml file in the root dir
CARGO_MANIFEST_PATH ?= $(shell find -maxdepth 2 -name Cargo.toml)
CARGO_INCREMENTAL   ?= 1
RUSTC_BOOTSTRAP     ?= 0

.PHONY: test build release stop run tidy

# function with a generic template to run docker with the required values
# Accepts $1 = command to run, $2 = additional flags (optional)
docker_run = docker run \
    --name=$(DOCKER_NAME)-$@ \
    --rm \
    --user `id -u`:`id -g` \
    --workdir=/usr/src/app/$(PACKAGE_NAME) \
    -e CARGO_INCREMENTAL=$(CARGO_INCREMENTAL) \
    -e RUSTC_BOOTSTRAP=$(RUSTC_BOOTSTRAP) \
    -v "$(PWD)/..:/usr/src/app" \
    -v "$(PWD)/../.cargo/registry:/usr/local/cargo/registry" \
    $(2) \
    -t $(BUILD_IMAGE_NAME):$(BUILD_IMAGE_TAG) \
    $(1)

check-cargo-registry:
	if [ ! -d "$(PWD)/../.cargo/registry" ]; then mkdir -p "$(PWD)/../.cargo/registry" ; fi

build: check-cargo-registry
	@$(call docker_run,cargo build)

test: test-rust

test-rust: check-cargo-registry
	@echo "$(YELLOW)Finding manifest-path for cargo...$(NC)"
ifeq ("$(CARGO_MANIFEST_PATH)", "")
	@echo "$(BOLD)$(YELLOW)No Cargo.toml found in any of the subdirectories, skipping cargo tests...$(NC)$(NC)"
else
	@echo "$(YELLOW)Running cargo tests...$(NC)"
	@$(call docker_run,cargo test --manifest-path "$(CARGO_MANIFEST_PATH)" --all)
	@echo "$(YELLOW)Linting Rust codes through clippy...$(NC)"
	@$(call docker_run,cargo clippy --manifest-path "$(CARGO_MANIFEST_PATH)" --all -- -D warnings)
	@echo "$(YELLOW)Running and checking Rust codes formats...$(NC)"
	@$(call docker_run,cargo fmt --manifest-path "$(CARGO_MANIFEST_PATH)" --all -- --check)
endif

tidy-rust: check-cargo-registry
	@echo "$(YELLOW)Running rust file formatting fixes...$(NC)"
	@$(call docker_run,cargo fmt --all)

tidy: tidy-rust

release:
	@$(call docker_run,cargo build --release)

clean: check-cargo-registry
	@cargo clean

run: stop
    # Run docker container as a deamon and map a port
	@$(call docker_run,sh,-d -p $(HOST_PORT):$(DOCKER_PORT))

stop:
	@docker kill $${DOCKER_NAME}-run || true
	@docker rm $${DOCKER_NAME}-run || true

all: test build release run
