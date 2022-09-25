# We might not have a Cargo.toml file in the root dir
CARGO_MANIFEST_PATH ?= "Cargo.toml"
CARGO_INCREMENTAL   ?= 1
RUSTC_BOOTSTRAP     ?= 0
RELEASE_TARGET      ?= x86_64-unknown-linux-musl

ifeq ("$(CARGO_MANIFEST_PATH)", "")
cargo_run = echo "$(BOLD)$(YELLOW)No Cargo.toml found in any of the subdirectories, skipping cargo check...$(NC)$(SGR0)"
else
cargo_run = $(call docker_run,cargo $(1) --manifest-path "$(CARGO_MANIFEST_PATH)" $(2))
endif

help-rust:
	@echo ""
	@echo "$(SMUL)$(BOLD)$(GREEN)Rust$(NC)$(SGR0)"
	@echo "  $(YELLOW)All cargo commands will use '--manifest-path $(CARGO_MANIFEST_PATH)'$(NC)"
	@echo "  $(BOLD)build$(SGR0)       -- Run 'cargo build'"
	@echo "  $(BOLD)release$(SGR0)     -- Run 'cargo build --release --target RELEASE_TARGET'"
	@echo "                 (RELEASE_TARGET=$(RELEASE_TARGET))"
	@echo "  $(BOLD)clean$(SGR0)       -- Run 'cargo clean'"
	@echo "  $(BOLD)rust-check$(SGR0)  -- Run 'cargo check'"
	@echo "  $(BOLD)rust-test$(SGR0)   -- Run 'cargo test --all'"
	@echo "  $(BOLD)rust-clippy$(SGR0) -- Run 'cargo clippy --all -- -D warnings'"
	@echo "  $(BOLD)rust-fmt$(SGR0)    -- Run 'cargo fmt --all -- --check' to check rust file formats."
	@echo "  $(BOLD)rust-tidy$(SGR0)   -- Run 'cargo fmt --all' to fix rust file formats if needed."

# Rust / cargo targets
check-cargo-registry:
	if [ ! -d "$(PWD)/.cargo/registry" ]; then mkdir -p "$(PWD)/.cargo/registry" ; fi

.SILENT: check-cargo-registry docker-pull

build: check-cargo-registry docker-pull
	@$(call cargo_run,build)

release: docker-pull
	@$(call cargo_run,build,--release --target $(RELEASE_TARGET))

clean: check-cargo-registry docker-pull
	@$(call cargo_run,clean)

rust-check: check-cargo-registry docker-pull
	@$(call cargo_run,check)

rust-test: check-cargo-registry docker-pull
	@$(call cargo_run,test,--all)

rust-clippy: check-cargo-registry docker-pull
	@$(call cargo_run,clippy,--all -- -D warnings)

rust-fmt: check-cargo-registry docker-pull
	@echo "$(YELLOW)Running and checking Rust codes formats...$(NC)"
	@$(call cargo_run,fmt,--all -- --check)

rust-tidy: check-cargo-registry docker-pull
	@echo "$(YELLOW)Running rust file formatting fixes...$(NC)"
	@$(call cargo_run,fmt,--all)

export
