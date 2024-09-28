package static

var DockerComposeContent = `version: '3.2'
services:
  jobmanager:
    image: repository.chainbase.com/network/flink:v1.18-0.2
    container_name: chainbase_jobmanager
    hostname: chainbase_jobmanager
    user: "flink"
    command:
      - /bin/bash
      - -c
      - |
        ./bin/sql-gateway.sh start -Dsql-gateway.endpoint.rest.address=localhost &
        ./bin/jobmanager.sh start-foreground
    ports:
      - "8081:8081"
      - "8083:8083"
    volumes:
      - ./tmp:/opt/flink/tmp
      - ./statuspoint/checkpoint:/opt/flink/checkpoint
      - ./statuspoint/savepoint:/opt/flink/savepoint
      - ./log:/opt/flink/log
      - ./proof:/opt/proof
    networks:
      - avs_network

  taskmanager:
    image: repository.chainbase.com/network/flink:v1.18-0.2
    container_name: chainbase_taskmanager
    hostname: chainbase_taskmanager
    user: "flink"
    depends_on:
      - jobmanager
    command: "./bin/taskmanager.sh start-foreground"
    volumes:
      - ./statuspoint/checkpoint:/opt/flink/checkpoint
      - ./statuspoint/savepoint:/opt/flink/savepoint
      - ./tmp:/opt/flink/tmp
      - ./log:/opt/flink/log
    networks:
      - avs_network

  postgres:
    image: postgres:16.4
    container_name: chainbase_postgres
    hostname: chainbase_postgres
    ports:
      - "5432:5432"
    volumes:
      - ./postgres_data:/var/lib/postgresql/data
      - ./schema:/schema
    environment:
      - POSTGRES_PASSWORD=${POSTGRES_PASSWORD:-postgres}
      - POSTGRES_USER=${POSTGRES_USER:-postgres}
      - POSTGRES_DB=${POSTGRES_DB:-manuscript_node}
    networks:
      - avs_network
    restart: unless-stopped

networks:
  avs_network:`
