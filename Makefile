CARGO := $(shell command -v cargo 2> /dev/null)

ifndef CARGO
$(error "Cannot find cargo. Please install and try again!")
endif

.PHONY: clean
clean:
	@$(CARGO) clean

.PHONY: lint
lint:
	@$(CARGO) clippy

.PHONY: format
format:
	@$(CARGO) fmt

.PHONY: check
check: format lint

.PHONY: build
build:
	@$(CARGO) build

.PHONY: run
run:
	@$(CARGO) run

.PHONY: release
release: clean
	@$(CARGO) build --release

.PHONY: test
test:
	@$(CARGO) test

# Load specific variables from .env file
ifneq (,$(wildcard .env))
    export SCYLLA_NODES := $(shell grep -E '^SCYLLA_NODES=' .env | cut -d '=' -f2)
    export SCYLLA_KEYSPACE := $(shell grep -E '^SCYLLA_KEYSPACE=' .env | cut -d '=' -f2)
    export SCYLLA_USERNAME := $(shell grep -E '^SCYLLA_USERNAME=' .env | cut -d '=' -f2)
    export SCYLLA_PASSWORD := $(shell grep -E '^SCYLLA_PASSWORD=' .env | cut -d '=' -f2)
endif

.PHONY: migrate
migrate: keyspace
	@migrate --host=$(SCYLLA_NODES) --keyspace=$(SCYLLA_KEYSPACE) $(if $(SCYLLA_USERNAME), --user=$(SCYLLA_USERNAME),) $(if $(SCYLLA_PASSWORD),--password=$(SCYLLA_PASSWORD),)

.PHONY: keyspace
keyspace:
	@toolkit keyspace --host=$(SCYLLA_NODES) --keyspace=$(SCYLLA_KEYSPACE) --replication-factor="1" $(if $(SCYLLA_USERNAME), --user=$(SCYLLA_USERNAME),) $(if $(SCYLLA_PASSWORD),--password=$(SCYLLA_PASSWORD),)
