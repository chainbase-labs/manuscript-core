package commands

import (
	"fmt"
	"io"
	"log"
	"manuscript-core/client"
	"manuscript-core/pkg"
	"os"
	"path/filepath"
)

func RunManuscript(args []string) {
	if len(args) < 1 {
		log.Fatalf("Error: Manuscript path is required as the first argument.")
	}
	manuscriptPath := args[0]
	var ms *pkg.Manuscript
	var manuscriptDir string

	if _, err := os.Stat(manuscriptPath); os.IsNotExist(err) {
		log.Fatalf("Error: Manuscript file does not exist: %s", manuscriptPath)
	}

	steps := []struct {
		name string
		fn   func() error
	}{
		{"Step 1: Parsing manuscript yaml", func() error {
			var err error
			ms, err = ParseManuscriptYaml(manuscriptPath)
			if err != nil {
				return err
			}
			manuscriptDir = fmt.Sprintf("manuscript/%s", ms.Name)
			if len(ms.Sinks) != 0 {
				if ms.Sinks[0].Type == "postgres" {
					ms.Sink = "postgres"
				}
			}
			return nil
		}},
		{"Step 2: Checking manuscript is already deployed", func() error {
			err := CheckManuscriptExist(ms)
			if err != nil {
				return err
			}
			return nil
		}},
		{"Step 3: Create Directory", func() error { return createDirectory(manuscriptDir) }},
		{"Step 4: Create ManuscriptFile", func() error { return copyManuscriptFile(manuscriptDir, manuscriptPath) }},
		{"Step 5: Create DockerComposeFile", func() error { return createDockerComposeFile(manuscriptDir, ms) }},
		{"Step 6: Check Docker Installed", func() error { return checkDockerInstalled() }},
		{"Step 7: Start Docker Containers", func() error { return startDockerContainers(manuscriptDir) }},
		{"Step 8: Check Container Status", func() error { return checkContainerStatus(ms) }},
	}

	for _, step := range steps {
		err := pkg.ExecuteStepWithLoading(step.name, true, step.fn)
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

func copyManuscriptFile(manuscriptDir, manuscriptPath string) error {
	_, fileName := filepath.Split(manuscriptPath)
	destinationPath := filepath.Join(manuscriptDir, fileName)

	sourceFile, err := os.Open(manuscriptPath)
	if err != nil {
		return fmt.Errorf("failed to open source file: %w", err)
	}
	defer sourceFile.Close()

	destFile, err := os.Create(destinationPath)
	if err != nil {
		return fmt.Errorf("failed to create destination file: %w", err)
	}
	defer destFile.Close()

	_, err = io.Copy(destFile, sourceFile)
	if err != nil {
		return fmt.Errorf("failed to copy file content: %w", err)
	}

	err = destFile.Sync()
	if err != nil {
		return fmt.Errorf("failed to sync destination file: %w", err)
	}

	return nil
}

func ParseManuscriptYaml(manuscriptPath string) (*pkg.Manuscript, error) {
	ms, err := pkg.ParseYAML(manuscriptPath)
	if err != nil {
		log.Fatalf("Error: Failed to parse manuscript yaml: %v", err)
		return &pkg.Manuscript{}, err
	}
	return ms, nil
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

func CheckManuscriptExist(ms *pkg.Manuscript) error {
	dockers, err := pkg.RunDockerPs()
	if err != nil {
		log.Fatalf("Error: Failed to get docker ps: %v", err)
	}
	if len(dockers) == 0 {
		return nil
	}
	for _, d := range dockers {
		if d.Name == fmt.Sprintf("%s-jobmanager-1", ms.Name) {
			return fmt.Errorf("error: Manuscript [ %s ] already deployed, please change the name in the manuscript yaml file", ms.Name)
		}
	}
	return nil
}
