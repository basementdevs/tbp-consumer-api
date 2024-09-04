CARGO := $(shell command -v cargo 2> /dev/null)

ifndef CARGO
$(error "Cannot find cargo. Please install and try again!")
endif

all: help ## Default target: shows the help message with available commands.

.PHONY: clean
clean: ## Cleans up the project by removing the target directory.
	@$(CARGO) clean

.PHONY: lint
lint: ## Runs Clippy to lint the codebase.
	@$(CARGO) clippy --no-deps

.PHONY: format
format: ## Formats the codebase using rustfmt.
	@$(CARGO) fmt

.PHONY: check
check: format lint ## Formats the codebase and then lints it.

.PHONY: build
build: ## Compiles the project.
	@$(CARGO) build

.PHONY: run
run: ## Compiles and runs the project.
	@$(CARGO) run

.PHONY: release
release: clean ## Cleans up the project and compiles it for release.
	@$(CARGO) build --release

.PHONY: test
test: ## Runs the test suite.
	@$(CARGO) test

# Load .env file
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

.PHONY: print-env
print-env: ## Prints the loaded environment variables from the .env file.
	$(foreach v, $(.VARIABLES), $(info $(v)=$($(v))))

.PHONY: migrate
migrate: ## Runs database migrations
	@migrate --host=$(SCYLLA_NODES) --keyspace=$(SCYLLA_KEYSPACE) $(if $(SCYLLA_USERNAME),--user=$(SCYLLA_USERNAME)) $(if $(SCYLLA_PASSWORD),--password=$(SCYLLA_PASSWORD))

.PHONY: keyspace
keyspace: ## Configures the keyspace in the ScyllaDB
	@toolkit keyspace --host=$(SCYLLA_NODES) --keyspace=$(SCYLLA_KEYSPACE) --replication-factor="1" $(if $(SCYLLA_USERNAME),--user=$(SCYLLA_USERNAME)) $(if $(SCYLLA_PASSWORD),--password=$(SCYLLA_PASSWORD))

.PHONY: watch
watch: ## Watches for changes in the source files and runs the project on each change.
	@$(CARGO) watch -w src -x run

.PHONY: dev
dev: ## Starts the development environment using Docker Compose.
	@docker compose --file docker-compose.yml up

.PHONY: help
help: ## Shows the help message with available commands.
	@echo "Available commands:"
	@grep -E '^[^[:space:]]+:[^:]*?## .*$$' $(MAKEFILE_LIST) | \
	awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-30s\033[0m %s\n", $$1, $$2}'
