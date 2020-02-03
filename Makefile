NAME       := mrsa

.PHONY: build test

# Build debug version
#---------------------------------------------------------------------------------------------------
build:
	@echo "Building ${NAME}..."
	@echo "------------------------------------------------------------------------"
	cargo build --all-features
	@mkdir -p bin
	@cp target/debug/${NAME} bin

# Test app
#---------------------------------------------------------------------------------------------------
test:
	@echo "Testing ${NAME}..."
	@echo "------------------------------------------------------------------------"
	cargo test --all-features

clean:
	@rm -rf bin
	@rm -rf target
	@mkdir bin
