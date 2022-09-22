FROM ghcr.io/arrow-air/tools/arrow-rust:latest AS build

ENV CARGO_INCREMENTAL=1
ENV RUSTC_BOOTSTRAP=0

ARG PACKAGE_NAME=

COPY . /usr/src/app

RUN cd /usr/src/app/${PACKAGE_NAME}/src ; cargo build --release

FROM alpine:latest
ARG PACKAGE_NAME=
COPY --from=build /usr/src/app/target/release/${PACKAGE_NAME} /usr/local/bin/${PACKAGE_NAME}
RUN ln -s /usr/local/bin/${PACKAGE_NAME} /usr/local/bin/server

ENTRYPOINT ["/usr/local/bin/server"]
