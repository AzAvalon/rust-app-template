.DEFAULT_GOAL := help
.PHONY: help build run test coverage fmt clippy lint chart-lint docker clean \
        ci-setup tests-with-coverage ci-post-run

## Local development

help: ## Show this help
	@grep -hE '^[a-zA-Z_-]+:.*?## ' $(MAKEFILE_LIST) | \
	  awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

build: ## Build the release binary
	cargo build --release --locked

run: ## Start the app on port 8000
	cargo run --locked --bin app

test: ## Run the tests
	cargo nextest run --locked

coverage: ## Run the tests with a coverage report in the terminal
	cargo llvm-cov nextest --locked

fmt: ## Format the code
	cargo fmt --all

clippy: ## Run clippy with every lint as an error
	cargo clippy --locked --all-targets --all-features -- -D warnings

lint: ## Run the pre-commit hooks on every file
	prek run --all-files

chart-lint: ## Lint the chart with chart-testing (ct.yaml, lintconf.yaml), in docker
	docker run --rm --user $$(id -u):$$(id -g) -e HOME=/tmp -v "$$PWD":/w -w /w \
	  quay.io/helmpack/chart-testing:v3.14.0 \
	  sh -c 'git config --global --add safe.directory /w && ct lint --config ct.yaml'

docker: ## Build the image for the local platform
	DOCKER_PLATFORMS=linux/$$(docker version -f '{{.Server.Arch}}') \
	  docker buildx bake -f dockerfiles/docker-bake.hcl --load

clean: ## Remove the build output and test reports
	cargo clean
	rm -f coverage.info junit.xml
	rm -rf chart/charts

## CI targets, called by AzAvalon/workflows _tests_app.yml

# Private git dependencies are fetched with the App token of the job
ci-setup:
	git config --global url."https://x-access-token:$${SCOPED_GITHUB_TOKEN}@github.com".insteadOf https://github.com

# _tests_app.yml uploads coverage.info (lcov) and junit.xml
tests-with-coverage:
	cargo llvm-cov nextest --locked --lcov --output-path coverage.info

ci-post-run:
	cp target/nextest/default/junit.xml junit.xml
