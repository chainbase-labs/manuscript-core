.PHONY: run build help

help:
	@echo "Available commands:"
	@echo "  build  - Build the Docker image for the Flink job"
	@echo "  run    - Run the manuscript (usage: make run path=/path/to/manuscript.yaml)"
	@echo "  help   - Display this help message"

build:
	@bash local-build.sh

run:
	@if [ -z "$(path)" ]; then \
		echo "Error: Please provide the path to manuscript.yaml using 'path=/path/to/manuscript.yaml'"; \
		exit 1; \
	fi; \
	ms_abs_path=$$(realpath $(path)); \
	if [ ! -f "$$ms_abs_path" ]; then \
		echo "Error: manuscript.yaml not found at $$ms_abs_path"; \
		exit 1; \
	fi; \
	image=$$(docker images -q ms_flink:$$(git rev-parse --abbrev-ref HEAD)-$$(git rev-parse --short HEAD)); \
	if [ -z "$$image" ]; then \
		echo "Error: Docker image not found. Please run 'make build' first."; \
		exit 1; \
	fi; \
	docker run -it --rm \
		-v $$ms_abs_path:/opt/flink/manuscript.yaml \
		ms_flink:$$(git rev-parse --abbrev-ref HEAD)-$$(git rev-parse --short HEAD)


