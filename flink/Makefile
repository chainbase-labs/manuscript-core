.PHONY: run build help

help:
	@echo "Available commands:"
	@echo "  build  - Build the Docker image for the Flink job"
	@echo "  run    - Run the manuscript (usage: make run path=/path/to/manuscript.yaml [image=custom_image] [data_path=/custom/data/path])"
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
	image=$${image:-ms_flink:$$(git rev-parse --abbrev-ref HEAD | sed "s#/#-#g")-$$(git rev-parse --short HEAD)}; \
	data_path=$${data_path:-./data}; \
	export MS_FLINK_IMAGE=$$image; \
	export MANUSCRIPT_DATA_PATH=$$data_path; \
	export MANUSCRIPT_PATH=$$ms_abs_path; \
	docker-compose up -d