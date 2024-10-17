package static

var DockerComposeTemplate = `
version: '3.2'
name: {{.Name}}
services:
  jobmanager: 
    image: repository.chainbase.com/manuscript-node/manuscript-node:latest
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
      - ms_network

  taskmanager:
    image: repository.chainbase.com/manuscript-node/manuscript-node:latest
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
      - ms_network

networks:
  ms_network:`

var DockerComposeWithPostgresqlContent = `
version: '3.2'
name: {{.Name}}
services:
  jobmanager: 
    image: repository.chainbase.com/manuscript-node/manuscript-node:latest
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
      - ms_network

  taskmanager:
    image: repository.chainbase.com/manuscript-node/manuscript-node:latest
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
      - ms_network

  postgres:
    image: postgres:16.4
    ports:
      - "{{.DbPort}}:5432"
    volumes:
      - ./postgres_data:/var/lib/postgresql/data
    environment:
      - POSTGRES_PASSWORD=${POSTGRES_PASSWORD:-postgres}
      - POSTGRES_USER=${POSTGRES_USER:-postgres}
      - POSTGRES_DB=${POSTGRES_DB:-public}
    networks:
      - ms_network
    restart: unless-stopped

  hasura:
    image: {{.GraphQLImage}}
    ports:
      - "{{.GraphQLPort}}:8080"
    depends_on:
      - postgres
    environment:
      HASURA_GRAPHQL_DATABASE_URL: postgres://postgres:${POSTGRES_PASSWORD:-postgres}@postgres:5432/{{.Database}}
      HASURA_GRAPHQL_ENABLE_CONSOLE: "true"
    networks:
      - ms_network
    restart: unless-stopped

networks:
  ms_network:`
