PYTHON_PATH=""

help-python:
	@echo ""
	@echo "$(SMUL)$(BOLD)$(GREEN)Python$(NC)$(SGR0)"
	@echo "  $(BOLD)python-test$(SGR0) -- Run 'yapf -r -i -vv --style google --exclude '**/.cargo/registry' .'"
	@echo "                 to validate python files against Google style guide."
	@echo "                 Run 'flake8 --exclude '**/.cargo/registry' .' to validate python files against flake8 style guide."
	@echo "  $(BOLD)python-tidy$(SGR0) -- Run 'black --extend-exclude .cargo ' to fix python style formats if needed."

# Python / yapf, flake8 targets
python-test: docker-pull
	@echo "$(YELLOW)Formatting and checking Python files with Google style...$(NC)"
	@$(call docker_run,yapf -r -i -vv --style google --exclude "$(PYTHON_PATH).cargo/registry" .)
	@echo "$(YELLOW)Formatting and checking Python files with flake8 style...$(NC)"
	@$(call docker_run,flake8 --exclude "$(PYTHON_PATH).cargo/registry" .)

python-tidy: docker-pull
	@echo "$(YELLOW)Running python file formatting fixes...$(NC)"
	@$(call docker_run,black --extend-exclude $(PYTHON_PATH).cargo .)
