CARGO ?= cargo
RUSTUP ?= rustup


help: Makefile
	@echo
	@echo " Choose a command run in Seal:"
	@echo
	@sed -n 's/^##//p' $< | column -t -s ':' |  sed -e 's/^/ /'
	@echo


## config: Install dependencies.
.PHONY: config
config:
	$(RUSTUP) component add rustfmt


## build: Build binary
.PHONY: build
build:
	@echo "\n>> ============= Cargo Build ============= <<"
	rm -rf target
	$(CARGO) build -j 1 --verbose --all


## release: Build releases
.PHONY: release
release:
	@echo "\n>> ============= Cargo Release ============= <<"
	rm -rf target
	$(CARGO) build -j 1 --release --verbose


## test: Run test cases
.PHONY: test
test:
	@echo "\n>> ============= Cargo Test ============= <<"
	$(CARGO) test --verbose --all


## fmt: Format code
.PHONY: fmt
fmt:
	@echo "\n>> ============= Cargo Format ============= <<"
	$(CARGO) fmt


## fmt_check: Check format
.PHONY: fmt_check
fmt_check:
	@echo "\n>> ============= Cargo Format Check ============= <<"
	$(CARGO) fmt -- --check


## run: Run project
.PHONY: run
run:
	@echo "\n>> ============= Cargo Run ============= <<"
	$(CARGO) run -j 1


## publish: Publish project
.PHONY: publish
publish:
	@echo "\n>> ============= Cargo Publish ============= <<"
	$(CARGO) login ${CARGO_TOKEN}
	$(CARGO) publish


## ci: Run all CI tests.
.PHONY: ci
ci: build test fmt_check
	@echo "\n>> ============= All quality checks passed ============= <<"


.PHONY: help
