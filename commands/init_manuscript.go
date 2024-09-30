package commands

import (
	"bufio"
	"fmt"
	"log"
	"manuscript-core/client"
	"manuscript-core/pkg"
	"manuscript-core/static"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"
	"strings"
	"time"
)

func InitManuscript() {
	var chains []*client.ChainBaseDatasetListItem
	err := pkg.ExecuteStepWithLoading("Checking Datasets From Network", func() error {
		c := client.NewChainBaseClient("https://api.chainbase.com")
		var err error
		chains, err = c.GetChainBaseDatasetList()
		if err != nil {
			fmt.Printf("Error: %v\n", err)
			return err
		}
		return nil
	})
	if err != nil {
		log.Fatalf("\033[31m✗ %s failed: %v\n", fmt.Sprintf("Step %d", 1), err)
	}

	reader := bufio.NewReader(os.Stdin)

	fmt.Println("\r\033[33m🏂 1.Please select a chainbase network dataset from the list below:\033[0m")
	for i, chain := range chains {
		fmt.Printf("%d: %s (Database: %s)\n", i+1, chain.Name, chain.DatabaseName)
	}

	fmt.Print("\r\033[33m🏂 1.Enter your choice (default is zkevm): \033[0m")
	chainChoice, _ := reader.ReadString('\n')
	chainChoice = strings.TrimSpace(chainChoice)

	selectedChain := "zkevm"
	selectedDatabase := "zkevm"
	defaultChainIndex := 1

	if chainChoice != "" {
		index, err := strconv.Atoi(chainChoice)
		if err != nil || index < 1 || index > len(chains) {
			fmt.Printf("Invalid choice. Please enter a number between 1 and %d.\n", len(chains))
			return
		}
		selectedChain = chains[index-1].Name
		selectedDatabase = chains[index-1].DatabaseName
		defaultChainIndex = index - 1
		fmt.Printf("\r\033[32m\u2714 You have selected chain: %s\n\033[0m\n", selectedChain)
	} else {
		fmt.Printf("No input provided. Defaulting to chain: %s\n\033[0m\n", selectedChain)
	}

	fmt.Println("\r\033[33m🧲 2.Please select a table from the list below:\033[0m")
	for i, table := range chains[defaultChainIndex].Tables {
		fmt.Printf("%d: %s\n", i+1, table)
	}

	fmt.Print("\r\033[33mEnter your choice (default is blocks): \033[0m")
	tableChoice, _ := reader.ReadString('\n')
	tableChoice = strings.TrimSpace(tableChoice)

	selectedTable := "blocks"
	if tableChoice != "" {
		index, err := strconv.Atoi(tableChoice)
		if err != nil || index < 1 || index > len(chains[defaultChainIndex].Tables) {
			fmt.Printf("Invalid choice. Please enter a number between 1 and %d.\n", len(chains[defaultChainIndex].Tables))
			return
		}
		selectedTable = chains[defaultChainIndex].Tables[index-1]
		fmt.Printf("\r\033[32m\u2714 You have selected table: %s\n\033[0m\n", selectedTable)
	} else {
		fmt.Printf("\u001B[32m\u2714 No input provided. Defaulting to table: %s\n\033[0m\n", selectedTable)
	}

	defaultSQL := fmt.Sprintf("Select * From %s.%s Limit 100", selectedDatabase, selectedTable)
	fmt.Printf("\r\033[33m🧬 3.Enter your SQL query (default is '%s'): \033[0m", defaultSQL)
	sqlQuery, _ := reader.ReadString('\n')
	sqlQuery = strings.TrimSpace(sqlQuery)
	if sqlQuery == "" {
		sqlQuery = defaultSQL
		fmt.Printf("\r\033[32m\u2714 No input provided. Defaulting to SQL query: %s\n\033[0m\n", sqlQuery)
	} else {
		fmt.Printf("\033[32m\u2714 You have entered SQL query: %s\033\n[0m\n", sqlQuery)
	}

	fmt.Println("\033[33m📍 4.Please select a data output target:\033[0m")
	fmt.Println("1: Print (output to console)")
	fmt.Println("2: Postgresql")

	fmt.Print("\033[33mEnter your choice (default is Print): ")
	outputChoice, _ := reader.ReadString('\n')
	outputChoice = strings.TrimSpace(outputChoice)

	selectedOutput := "print"
	if outputChoice != "" {
		index, err := strconv.Atoi(outputChoice)
		if err != nil || index < 1 || index > 2 {
			fmt.Printf("Invalid choice. Please enter a number between 1 and 2.\n")
			return
		}
		if index == 2 {
			selectedOutput = "postgresql"
		}
		fmt.Printf("\r\033[32m\u2714 You have selected output target: %s\n", selectedOutput)
	} else {
		fmt.Printf("\033[32m\u2714 No input provided. Defaulting to output target: %s\033[0m\n", selectedOutput)
	}

	fmt.Printf("\n\033[33m🏄🏄 Summary of your selections:\033[0m\n")
	fmt.Printf("Selected chain: \033[32m%s\033[0m\n", selectedChain)
	fmt.Printf("Selected table: \u001B[32m%s\u001B[0m\n", selectedTable)
	fmt.Printf("SQL query: \u001B[32m%s\u001B[0m\n", sqlQuery)
	fmt.Printf("Data output target: \u001B[32m%s\u001B[0m\n", selectedOutput)

	executeInitManuscript()
}

func executeInitManuscript() {
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
