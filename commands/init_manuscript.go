package commands

import (
	"fmt"
	"log"
	"manuscript-core/pkg"
	"manuscript-core/static"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"time"
)

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
		err := pkg.ExecuteStepWithLoading(fmt.Sprintf("Step %d", i+1), step)
		if err != nil {
			log.Fatalf("\033[31m✗ %s failed: %v\n", fmt.Sprintf("Step %d", i+1), err)
		}
	}
	log.Println("\033[32m✓ All steps completed successfully!")
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
	err := os.WriteFile(composeFilePath, []byte(static.DockerComposeContent), 0644)
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
	err := os.WriteFile(nodeSqlSchema, []byte(static.CreateTableSQL), 0644)
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
	err := os.WriteFile(demoFilePath, []byte(static.ManuscriptDemo), 0644)
	if err != nil {
		return fmt.Errorf("failed to write demo manuscript file: %w", err)
	}
	return nil
}

func createSqlGatewayFile(dir string) error {
	sqlGatewayFilePath := filepath.Join(dir, "sql-gateway.yaml")
	err := os.WriteFile(sqlGatewayFilePath, []byte(static.InitSql), 0644)
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
