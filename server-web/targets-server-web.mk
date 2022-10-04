DOCKER_BUILD_PATH = ../
DOCKER_NAME       = arrow-$(PACKAGE_NAME)
DOCKER_PORT       = 8000
HOST_PORT         = 8080
HOSTNAME          = $(PACKAGE_NAME)-run

example: check-cargo-registry docker-pull
	@docker compose \
		-f ../docker-compose.yml \
		run \
		--workdir=/usr/src/app/${PACKAGE_NAME} \
		--user `id -u`:`id -g` \
		--rm \
		-e CARGO_INCREMENTAL=1 \
		-e RUSTC_BOOTSTRAP=0 \
		example
