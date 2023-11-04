GO_FLAGS   ?=
NAME       := kbt
PACKAGE    := github.com/bloznelis/$(NAME)
GIT_REV     = $(shell git rev-parse --short HEAD)
VERSION     = $(shell git describe --abbrev=0 --tags)

default: help

run: ## Runs the code
	@cargo run

check: ## Checks the source
	@cargo check --release

build:  ## Builds the binary
	@cargo build --release

release: ## Launches the release wizard
	./scripts/release.sh

help: ## This message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":[^:]*?## "}; {printf "\033[38;5;69m%-30s\033[38;5;38m %s\033[0m\n", $$1, $$2}'
