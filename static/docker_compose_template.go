package static

var DockerComposeTemplate = `
version: '3.2'
services:
  jobmanager: 
    image: ms_flink:cobra-cli-5732e3f
    container_name: {{.Name}}
    user: "flink"
    command: "standalone-job --job-classname com.chainbase.manuscript.ETLProcessor /opt/flink/manuscript.yaml --fromSavepoint /opt/flink/savepoint"
    ports:
      - "{{.Port}}:8081"
    volumes:
      - ./data/statuspoint/checkpoint:/opt/flink/checkpoint
      - ./data/statuspoint/savepoint:/opt/flink/savepoint
      - ./data/log:/opt/flink/log
      - ./manuscript.yaml:/opt/flink/manuscript.yaml
    networks:
      - ms_network_{{.Name}}

  taskmanager:
    image: ms_flink:cobra-cli-5732e3f
    user: "flink"
    depends_on:
      - jobmanager
    command: "taskmanager"
    scale: 1
    volumes:
      - ./data/statuspoint/checkpoint:/opt/flink/checkpoint
      - ./data/statuspoint/savepoint:/opt/flink/savepoint
      - ./data/log:/opt/flink/log
      - ./manuscript.yaml:/opt/flink/manuscript.yaml
    networks:
      - ms_network_{{.Name}}

networks:
  ms_network_{{.Name}}:`

var DockerComposeWithPostgresqlContent = `
version: '3.2'
services:
  jobmanager: 
    image: ms_flink:cobra-cli-5732e3f
    container_name: {{.Name}}
    user: "flink"
    command: "standalone-job --job-classname com.chainbase.manuscript.ETLProcessor /opt/flink/manuscript.yaml --fromSavepoint /opt/flink/savepoint"
    ports:
      - "{{.Port}}:8081"
    volumes:
      - ./data/statuspoint/checkpoint:/opt/flink/checkpoint
      - ./data/statuspoint/savepoint:/opt/flink/savepoint
      - ./data/log:/opt/flink/log
      - ./manuscript.yaml:/opt/flink/manuscript.yaml
    networks:
      - ms_network_{{.Name}}

  taskmanager:
    image: ms_flink:cobra-cli-5732e3f
    user: "flink"
    depends_on:
      - jobmanager
    command: "taskmanager"
    scale: 1
    volumes:
      - ./data/statuspoint/checkpoint:/opt/flink/checkpoint
      - ./data/statuspoint/savepoint:/opt/flink/savepoint
      - ./data/log:/opt/flink/log
      - ./manuscript.yaml:/opt/flink/manuscript.yaml
    networks:
      - ms_network_{{.Name}}

  postgres:
    image: postgres:16.4
    volumes:
      - ./postgres_data:/var/lib/postgresql/data
    environment:
      - POSTGRES_PASSWORD=${POSTGRES_PASSWORD:-postgres}
      - POSTGRES_USER=${POSTGRES_USER:-postgres}
      - POSTGRES_DB=${POSTGRES_DB:-public}
    networks:
      - ms_network_{{.Name}}
    restart: unless-stopped

networks:
  ms_network_{{.Name}}:`
