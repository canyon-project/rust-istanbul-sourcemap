# Makefile for building shared library and Go example

.PHONY: all build-rust build-go clean test

# Default target
all: build-rust build-go

# Build Rust shared library
build-rust:
	cargo build --release

# Build and run Go example
build-go: build-rust
	@echo "Copying shared library..."
ifeq ($(OS),Windows_NT)
	cp target/release/istanbul_sourcemap.dll examples/go/
else
	@if [ "$$(uname)" = "Darwin" ]; then \
		cp target/release/libistanbul_sourcemap.dylib examples/go/libistanbul_sourcemap.so; \
	else \
		cp target/release/libistanbul_sourcemap.so examples/go/; \
	fi
endif
	@echo "Running Go example..."
	cd examples/go && go run main.go

# Clean build artifacts
clean:
	cargo clean
	rm -f examples/go/libistanbul_sourcemap.so
	rm -f examples/go/libistanbul_sourcemap.dylib
	rm -f examples/go/istanbul_sourcemap.dll

# Run tests
test:
	cargo test

# Build for specific target
build-target:
	@if [ -z "$(TARGET)" ]; then \
		echo "Usage: make build-target TARGET=<target>"; \
		echo "Example: make build-target TARGET=x86_64-unknown-linux-gnu"; \
		exit 1; \
	fi
	cargo build --release --target $(TARGET)

# Install cross-compilation tools (Linux only)
install-cross:
	@if [ "$$(uname)" = "Linux" ]; then \
		sudo apt-get update && sudo apt-get install -y gcc-aarch64-linux-gnu; \
	else \
		echo "Cross-compilation tools installation is only supported on Linux"; \
	fi