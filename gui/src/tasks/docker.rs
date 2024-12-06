pub const DOCKER_COMPOSE_TEMPLATE: &str = r#"version: '3.2'
name: {name}
services:
  jobmanager:
    image: {job_manager_image}
    user: "flink"
    command: "standalone-job --job-classname com.chainbase.manuscript.ETLProcessor /opt/flink/manuscript.yaml --fromSavepoint /opt/flink/savepoint"
    ports:
      - "{job_port}:8081"
    volumes:
      - ./data/statuspoint/checkpoint:/opt/flink/checkpoint
      - ./data/statuspoint/savepoint:/opt/flink/savepoint
      - ./data/log:/opt/flink/log
      - ./manuscript.yaml:/opt/flink/manuscript.yaml
    networks:
      - ms_network_{name}

  taskmanager:
    image: {job_manager_image}
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
      - ms_network_{name}

  postgres:
    image: postgres:16.4
    ports:
      - "{db_port}:5432"
    volumes:
      - ./postgres_data:/var/lib/postgresql/data
    environment:
      - POSTGRES_PASSWORD=${POSTGRES_PASSWORD:-postgres}
      - POSTGRES_USER=${POSTGRES_USER:-postgres}
      - POSTGRES_DB=${POSTGRES_DB:-{database}}
    networks:
      - ms_network_{name}
    restart: unless-stopped

  hasura:
    image: {hasura_image}
    ports:
      - "{graphql_port}:8080"
    depends_on:
      - postgres
    environment:
      HASURA_GRAPHQL_DATABASE_URL: postgres://postgres:${POSTGRES_PASSWORD:-postgres}@postgres:5432/{database}
      HASURA_GRAPHQL_ENABLE_CONSOLE: "true"
    networks:
      - ms_network_{name}
    restart: unless-stopped

networks:
  ms_network_{name}:"#;


pub const DOCKER_COMPOSE_TEMPLATE_SOLANA: &str = r#"version: '3.2'
name: {name}
services:
  postgres:
    image: postgres:16.4
    ports:
      - "{db_port}:5432"
    volumes:
      - ./postgres_data:/var/lib/postgresql/data
    environment:
      - POSTGRES_PASSWORD=${POSTGRES_PASSWORD:-postgres}
      - POSTGRES_USER=${POSTGRES_USER:-postgres}
      - POSTGRES_DB=${POSTGRES_DB:-{database}}
    networks:
      - ms_network
    restart: unless-stopped

  jobmanager:
    image: {job_manager_image}
    ports:
      - "{job_port}:8081"
    networks:
      - ms_network

  hasura:
    image: {hasura_image}
    ports:
      - "{graphql_port}:8080"
    depends_on:
      - postgres
    environment:
      HASURA_GRAPHQL_DATABASE_URL: postgres://postgres:${POSTGRES_PASSWORD:-postgres}@postgres:5432/{database}
      HASURA_GRAPHQL_ENABLE_CONSOLE: "true"
    networks:
      - ms_network
    restart: unless-stopped

networks:
  ms_network:"#; 

pub const JOB_CONFIG_TEMPLATE: &str = r#"
[{name}]
baseDir     = {home_dir}/manuscripts
name        = {name}
specVersion = {spec_version}
parallelism = {parallelism}
chain       = {chain}
table       = {table}
database    = {database}
query       = {query}
sink        = {sink_type}
port        = {job_port}
dbPort      = {db_port}
dbUser      = {db_user}
dbPassword  = {db_password}
graphqlPort = {graphql_port}"#; 

pub const MANUSCRIPT_TEMPLATE: &str = r#"name: {name}
specVersion: v1.0.0
parallelism: 1

sources:
  - name: {dataset_name}_{table_name} 
    type: dataset
    dataset: {dataset_name}.{table_name}

transforms:
  - name: {dataset_name}_{dataset_name}_{table_name}_transform
    sql: >
      Select * From {dataset_name}_{table_name}

sinks:
  - name: {dataset_name}_{dataset_name}_{table_name}_sink
    type: postgres
    from: {dataset_name}_{dataset_name}_{table_name}_transform
    database: {dataset_name}
    schema: public
    table: {table_name}
    primary_key: block_number
    config:
      host: postgres
      port: 5432
      username: postgres
      password: postgres
      graphqlPort: {graphql_port}"#;


pub const MANUSCRIPT_TEMPLATE_SOLANA: &str = r#"name: {name}
specVersion: v1.0.0
parallelism: 1

sources:
  - name: {dataset_name}
    type: substreams
    dataset: {dataset_name}.{table_name}

transforms:
  - name: {dataset_name}_transform
    sql: >
      Select * From {dataset_name}_{table_name}

sinks:
  - name: {dataset_name}_sink
    type: postgres
    from: {dataset_name}_transform
    database: {dataset_name}
    schema: public
    config:
      host: postgres
      port: 5432
      username: postgres
      password: postgres
      graphqlPort: {graphql_port}"#;