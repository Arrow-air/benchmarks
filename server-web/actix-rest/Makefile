SHELL := /bin/bash

SOURCE_PATH = $(PWD)/../

help: .help-base .help-rust .help-docker

include .makerc

export

include ../../.make/base.mk
include ../../.make/rust.mk
include ../../.make/docker.mk
include ../targets-server-web.mk
