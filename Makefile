.PHONY: all build test clean container container-build container-run docs

all: build

build:
	cargo build --release

test:
	cargo test

clean:
	cargo clean
	rm -f container/*.tar.gz

container:
	docker build -t argent:latest -f container/Dockerfile .

container-run:
	docker run -it argent:latest

container-build:
	bash container/build.sh

fmt:
	cargo fmt --check

clippy:
	cargo clippy -- -D warnings

audit:
	cargo audit

deps:
	cargo tree

install:
	cargo install --path .

uninstall:
	cargo uninstall argent

help:
	@echo "Argent Makefile Commands:"
	@echo "  make build          - Build release binary"
	@echo "  make test           - Run tests"
	@echo "  make container      - Build Docker container"
	@echo "  make container-run  - Run container"
	@echo "  make fmt            - Check formatting"
	@echo "  make clippy         - Run linter"
	@echo "  make audit          - Security audit"
	@echo "  make install        - Install binary"
