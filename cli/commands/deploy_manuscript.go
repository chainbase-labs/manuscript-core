package commands

import (
	"fmt"
	"log"
	"manuscript-core/cli/client"
	"manuscript-core/cli/static"
	"os"
)

func DeployManuscript(args []string) {
	if len(args) < 1 {
		log.Fatalf("Error: Manuscript path is required as the first argument.")
	}
	manuscriptPath := args[0]
	execSql := ""
	initSql := static.InitSql
	var c *client.FlinkClient

	if _, err := os.Stat(manuscriptPath); os.IsNotExist(err) {
		log.Fatalf("Error: Manuscript file does not exist: %s", manuscriptPath)
	}

	steps := []struct {
		name string
		fn   func() error
	}{
		{"Step 1: Initializing the flink client", func() error {
			var err error
			c, err = InitFlinkClient(initSql, c)
			return err
		}},
		{"Step 2: Parsing manuscript yaml", func() error { return ParseManuscriptYaml(manuscriptPath, execSql) }},
		{"Step 3: Deploy Manuscript to flink", func() error { return DeployManuscriptToFlink(c) }},
	}

	for _, step := range steps {
		err := executeStepWithLoading(step.name, step.fn)
		if err != nil {
			log.Fatalf("\033[31m✗ %s failed: %v\n", step.name, err)
		}
	}
	fmt.Println("\033[32m✓ Manuscript deployment completed successfully!")
}

func InitFlinkClient(initSql string, flinkClient *client.FlinkClient) (*client.FlinkClient, error) {
	flinkClient = client.NewFlinkClient("127.0.0.1:8083", initSql)
	if err := flinkClient.InitializeClient(); err != nil {
		log.Fatalf("Error: Failed to initialize client: %v", err)
		return nil, err
	}
	return flinkClient, nil
}

func ParseManuscriptYaml(manuscriptPath, execSql string) error {
	// Parse the manuscript yaml file
	return nil
}

func DeployManuscriptToFlink(client *client.FlinkClient) error {
	// Deploy the manuscript to flink
	operationHandle, err := client.ExecuteSQL("select * from zkevm.blocks limit 100;")
	if err != nil {
		log.Fatalf("Failed to execute SQL: %v", err)
	}

	if operationHandle == "" {
		log.Fatalf("Unexpected operation handle: %s", operationHandle)
	}

	if err := client.CheckSQLResult(operationHandle, 60); err != nil {
		log.Fatalf("Failed to check SQL result: %v", err)
	}
	return nil
}
