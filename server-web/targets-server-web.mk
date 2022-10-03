DOCKER_BUILD_PATH=../
DOCKER_NAME=arrow-$(PACKAGE_NAME)

example: check-cargo-registry docker-pull
	@$(call cargo_run,run --example endpoints)
