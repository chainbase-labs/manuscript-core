package commands

import (
	"fmt"
	"log"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"time"
)

var composeContent = `version: '3.2'
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

var createTableSQL = `
\c manuscript_node;

CREATE TABLE IF NOT EXISTS pow_results (
    chain VARCHAR(255),
    block_number BIGINT,
    block_hash VARCHAR(255),
    pow_result VARCHAR(255),
    insert_at TIMESTAMP(3),
    difficulty SMALLINT,
    task_index BIGINT,
    PRIMARY KEY (chain, block_number)
);
CREATE INDEX idx_pow_results_chain_block_number ON pow_results (chain, block_number);
CREATE INDEX idx_pow_results_task_index ON pow_results (task_index);
`

var manuscriptDemo = `name: demo
specVersion: v0.1.0
parallelism: 1

sources:
  - name: zkevm_blocks
    type: dataset
    dataset: zkevm.blocks
    filter: "block_number > 100000"

transforms:
  - name: zkevm_blocks_transform
    sql: >
      SELECT
          *
      FROM zkevm_blocks
      limit 100

sinks:
  - name: zkevm_blocks_sink
    type: print
    from: zkevm_blocks_transform`

var initSQL = `
CREATE CATALOG paimon WITH (
    'type' = 'paimon',
    'warehouse' = 'oss://network-testnet/warehouse',
    'fs.oss.endpoint' = 'network-testnet.chainbasehq.com',
    'fs.oss.accessKeyId' = '${OSS_ACCESS_KEY_ID}',
    'fs.oss.accessKeySecret' = '${OSS_ACCESS_KEY_SECRET}',
    'table-default.merge-engine' = 'deduplicate',
    'table-default.changelog-producer' = 'input',
    'table-default.metastore.partitioned-table' = 'false',
    'table-default.lookup.cache-file-retention' = '1 h',
    'table-default.lookup.cache-max-memory-size' = '256 mb',
    'table-default.lookup.cache-max-disk-size' = '10 gb',
    'table-default.log.scan.remove-normalize' = 'true',
    'table-default.changelog-producer.row-deduplicate' = 'false',
    'table-default.consumer.expiration-time' = '24 h',
    'table-default.streaming-read-mode' = 'file',
    'table-default.orc.bloom.filter.fpp' = '0.00001',
    'table-default.scan.plan-sort-partition' = 'true',
    'table-default.snapshot.expire.limit' = '10000',
    'table-default.snapshot.num-retained.max' = '2000'
);
set 'table.exec.sink.upsert-materialize' = 'NONE';
set 'state.backend.type' = 'rocksdb';
set 'state.checkpoints.dir' = 'file:///opt/flink/checkpoint';
set 'state.savepoints.dir' = 'file:///opt/flink/savepoint';
set 'execution.checkpointing.interval' = '60s';
set 'execution.checkpointing.min-pause' = '1s';
set 'execution.checkpointing.externalized-checkpoint-retention' = 'RETAIN_ON_CANCELLATION';
set 'execution.checkpointing.timeout' = '30 min';
set 'execution.checkpointing.max-concurrent-checkpoints' = '1';
set 'state.backend.incremental' = 'true';
set 'restart-strategy.fixed-delay.delay' = '10 s';
set 'execution.checkpointing.tolerable-failed-checkpoints' = '2147483647';
set 'sql-client.execution.result-mode' = 'tableau';
set 'table.exec.sink.not-null-enforcer' = 'ERROR';
use catalog paimon;
`

func InitManuscript() {
	manuscriptDir := "manuscript"
	steps := []func() error{
		func() error { return createDirectory(manuscriptDir) },
		func() error { return createDockerComposeFile(manuscriptDir) },
		func() error { return checkDockerInstalled() },
		func() error { return startDockerContainers(manuscriptDir) },
		func() error { return waitForContainers() },
		func() error { return checkContainerStatus() },
		func() error { return createSchemaFile(manuscriptDir) },
		func() error { return executeSQLCommands(manuscriptDir) },
		func() error { return createDemoManuscriptFile(manuscriptDir) },
	}

	for i, step := range steps {
		err := executeStepWithLoading(fmt.Sprintf("Step %d", i+1), step)
		if err != nil {
			log.Fatalf("\033[31m✗ %s failed: %v\n", fmt.Sprintf("Step %d", i+1), err)
		}
	}
	log.Println("\033[32m✓ All steps completed successfully!")
}

func executeStepWithLoading(stepName string, stepFunc func() error) error {
	done := make(chan struct{})
	loading := []string{"⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"}

	go func() {
		i := 0
		for {
			select {
			case <-done:
				return
			default:
				fmt.Printf("\r%s %s Loading... ", loading[i%len(loading)], stepName)
				time.Sleep(100 * time.Millisecond)
				i++
			}
		}
	}()

	err := stepFunc()

	close(done)
	if err != nil {
		fmt.Printf("\r\033[31m✗\033[0m %s failed!\n", stepName)
		fmt.Printf("\033[31mError: %v\n", err)
	} else {
		fmt.Printf("\r\033[32m✓\033[0m %s complete!\n", stepName)
	}

	return err
}

func createDirectory(dir string) error {
	err := os.MkdirAll(dir, 0755)
	if err != nil {
		return fmt.Errorf("failed to create directory %s: %w", dir, err)
	}

	sqlGatewayFile := fmt.Sprintf("%s/proof", dir)
	err = os.MkdirAll(sqlGatewayFile, 0755)
	if err != nil {
		return fmt.Errorf("failed to create directory %s: %w", dir, err)
	}

	err = createSqlGatewayFile(sqlGatewayFile)
	if err != nil {
		return fmt.Errorf("failed to create sql-gateway.yaml file: %w", err)
	}

	return nil
}

func createDockerComposeFile(dir string) error {
	composeFilePath := filepath.Join(dir, "docker-compose.yml")
	err := os.WriteFile(composeFilePath, []byte(composeContent), 0644)
	if err != nil {
		return fmt.Errorf("failed to write docker-compose.yml file: %w", err)
	}
	return nil
}

func checkDockerInstalled() error {
	_, err := exec.LookPath("docker")
	if err != nil {
		return fmt.Errorf("Docker is not installed. Please install Docker to proceed.")
	}
	return nil
}

func startDockerContainers(dir string) error {
	cmd := exec.Command("docker-compose", "-f", filepath.Join(dir, "docker-compose.yml"), "up", "-d")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err := cmd.Run()
	if err != nil {
		return fmt.Errorf("failed to start Docker containers: %w", err)
	}
	return nil
}

func waitForContainers() error {
	time.Sleep(5 * time.Second)
	return nil
}

func checkContainerStatus() error {
	containerNames := []string{"chainbase_jobmanager", "chainbase_taskmanager", "chainbase_postgres"}
	for _, containerName := range containerNames {
		isRunning, err := isContainerRunning(containerName)
		if err != nil {
			return fmt.Errorf("failed to check container %s status: %w", containerName, err)
		}
		if !isRunning {
			return fmt.Errorf("container %s is not running", containerName)
		}
	}
	return nil
}

func createSchemaFile(dir string) error {
	nodeSqlSchema := filepath.Join(dir, "schema/node.sql")
	err := os.WriteFile(nodeSqlSchema, []byte(createTableSQL), 0644)
	if err != nil {
		return fmt.Errorf("failed to write node.sql file: %w", err)
	}
	return nil
}

func executeSQLCommands(dir string) error {
	for i := 0; i < 30; i++ {
		execSQLCmd := []string{"docker", "exec", "-i", "chainbase_postgres", "psql", "-U", "postgres", "-f", "/schema/node.sql"}
		execSQL := exec.Command(execSQLCmd[0], execSQLCmd[1:]...)
		err := execSQL.Run()
		if err != nil {
			log.Println("waiting for container start...")
			time.Sleep(2 * time.Second)
			continue
		}
		return nil
	}
	return fmt.Errorf("failed to create database and tables")
}

func createDemoManuscriptFile(dir string) error {
	demoFilePath := filepath.Join(dir, "manuscript.yaml")
	err := os.WriteFile(demoFilePath, []byte(manuscriptDemo), 0644)
	if err != nil {
		return fmt.Errorf("failed to write demo manuscript file: %w", err)
	}
	return nil
}

func createSqlGatewayFile(dir string) error {
	sqlGatewayFilePath := filepath.Join(dir, "sql-gateway.yaml")
	err := os.WriteFile(sqlGatewayFilePath, []byte(initSQL), 0644)
	if err != nil {
		return fmt.Errorf("failed to write sql-gateway.yaml file: %w", err)
	}
	return nil
}

func isContainerRunning(containerName string) (bool, error) {
	cmd := exec.Command("docker", "ps", "--filter", fmt.Sprintf("name=%s", containerName), "--filter", "status=running", "--format", "{{.Names}}")
	output, err := cmd.Output()
	if err != nil {
		return false, fmt.Errorf("failed to run docker ps command: %w", err)
	}

	containers := strings.Split(strings.TrimSpace(string(output)), "\n")
	for _, name := range containers {
		if name == containerName {
			return true, nil
		}
	}
	return false, nil
}
