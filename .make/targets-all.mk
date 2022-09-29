SHELL := /bin/bash

DOCKER_NAME      := arrow-benchmarks-$(PACKAGE_NAME)
IMAGE_NAME       := benchmarks-$(PACKAGE_NAME)
BUILD_IMAGE_NAME := ghcr.io/arrow-air/tools/arrow-rust
BUILD_IMAGE_TAG  := latest
DOCKER_PORT      := 8000
HOST_PORT        := 8080

SOURCE_PATH      ?= $(PWD)

MD_FILES         := $(shell find . -iname "*.md" -type f ! -path "./server-web/.cargo/registry/**")

# Style templates for console output.
GREEN  := $(shell echo -e `tput setaf 2`)
YELLOW := $(shell echo -e `tput setaf 3`)
CYAN   := $(shell echo -e `tput setaf 6`)
NC     := $(shell echo -e `tput sgr0`)
BOLD   := $(shell echo -e `tput bold`)
SMUL   := $(shell echo -e `tput smul`)
SGR0   := $(shell echo -e `tput sgr0`)

# function with a generic template to run docker with the required values
# Accepts $1 = command to run, $2 = additional flags (optional)
docker_run = docker run \
	--name=$(DOCKER_NAME)-$@ \
	--hostname=$(DOCKER_NAME)-$@ \
	--rm \
	--network=example-net \
	--user `id -u`:`id -g` \
	--workdir=/usr/src/app/$(PACKAGE_NAME) \
	-e CARGO_INCREMENTAL=$(CARGO_INCREMENTAL) \
	-e RUSTC_BOOTSTRAP=$(RUSTC_BOOTSTRAP) \
	-v "$(SOURCE_PATH)/:/usr/src/app" \
	-v "$(SOURCE_PATH)/.cargo/registry:/usr/local/cargo/registry" \
	$(2) \
	-t $(BUILD_IMAGE_NAME):$(BUILD_IMAGE_TAG) \
	$(1)

.SILENT: docker-pull

help-all:
	@echo ""
	@echo "$(BOLD)$(CYAN)Available targets$(NC)$(SGR0)"
	@echo ""
	@echo "$(SMUL)$(BOLD)$(GREEN)TOML$(NC)$(SGR0)"
	@echo "  $(BOLD)toml-test$(SGR0)   -- Run 'taplo format --check' to validate TOML file formats."
	@echo "  $(BOLD)toml-tidy$(SGR0)   -- Run 'taplo format' to fix TOML file formats if needed."
	@echo ""
	@echo "$(SMUL)$(BOLD)$(GREEN)CSpell$(NC)$(SGR0)"
	@echo "  $(BOLD)cspell-test$(SGR0) -- Run 'cspell --words-only --unique "**/**" -c .cspell.config.yaml'"
	@echo "                 to validate files are not containing any spelling errors."
	@echo "  $(BOLD)cspell-add-words$(SGR0) -- Run 'cspell --words-only --unique "**/**" -c .cspell.config.yaml | "
	@echo "                      sort --ignore-case >> .cspell.project-words.txt'"
	@echo "                      to add remaining words to the project's cspell ignore list"

docker-pull:
	@docker network create example-net || true
	@docker pull -q $(BUILD_IMAGE_NAME):$(BUILD_IMAGE_TAG)

# TOML / taplo targets
toml-test: docker-pull
	@echo "$(YELLOW)Running toml file formatting tests...$(NC)"
	@$(call docker_run,taplo format --check)

toml-tidy: docker-pull
	@echo "$(YELLOW)Running toml file formatting fixes...$(NC)"
	@$(call docker_run,taplo format)

# editorconfig targets
editorconfig-test:
	@echo "$(YELLOW)Checking if the codebase is compliant with the .editorconfig file...$(NC)"
	@docker run \
		--name=$(DOCKER_NAME)-$@ \
		--rm \
		--user `id -u`:`id -g` \
		-w "/usr/src/app" \
		-v "$(PWD):/usr/src/app" \
		-t mstruebing/editorconfig-checker

# cspell targets
cspell-test: docker-pull
	@echo "$(YELLOW)Checking for spelling errors...$(NC)"
	@$(call docker_run,cspell --words-only --unique "**/**" -c .cspell.config.yaml)

# cspell add words
cspell-add-words: docker-pull
	@echo "$(YELLOW)Adding words to the project's cspell word list...$(NC)"
	@$(call docker_run,sh -c 'cspell --words-only --unique "**/**" -c .cspell.config.yaml 2> /dev/null | sort -f >> .cspell.project-words.txt')

md-test:
	@echo "$(YELLOW)Checking if all document links are valid...$(NC)"
	@docker run \
		--name=$(DOCKER_NAME)-$@ \
		--rm \
		--user `id -u`:`id -g` \
		-w "/usr/src/app" \
		-v "$(PWD):/usr/src/app" \
		ghcr.io/tcort/markdown-link-check:stable \
		$(MD_FILES)


export
