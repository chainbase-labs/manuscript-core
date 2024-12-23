package commands

import (
	"os"
	"os/exec"
	"path/filepath"
	"testing"
	"time"
)

func TestInitManuscriptDockerIntegration(t *testing.T) {
	t.Skip("Skip InitManuscriptDockerIntegration test")
	tempDir, err := os.MkdirTemp(".", "manuscript_test")
	if err != nil {
		t.Fatalf("Failed to create temporary directory: %v", err)
	}
	//defer os.RemoveAll(tempDir)

	originalDir, err := os.Getwd()
	if err != nil {
		t.Fatalf("Failed to get current directory: %v", err)
	}
	defer os.Chdir(originalDir)
	err = os.Chdir(tempDir)
	if err != nil {
		t.Fatalf("Failed to change to temporary directory: %v", err)
	}

	t.Log("Changed to temporary directory")

	InitManuscriptInteractive()
	t.Log("InitManuscript was called")

	manuscriptDir := "manuscript"
	t.Log(manuscriptDir)
	if _, err := os.Stat(manuscriptDir); os.IsNotExist(err) {
		t.Fatalf("manuscript directory was not created")
	}
	t.Log("manuscript directory was created111")
	composeFilePath := filepath.Join(manuscriptDir, "docker-compose.yml")
	if _, err := os.Stat(composeFilePath); os.IsNotExist(err) {
		t.Fatalf("docker-compose.yml file was not created")
	}

	time.Sleep(10 * time.Second)
	containerNames := []string{"chainbase_jobmanager", "chainbase_taskmanager", "chainbase_postgres"}
	for _, containerName := range containerNames {
		ok, err := isContainerRunning(containerName)
		if err != nil {
			t.Fatalf("Failed to check if container %s is running: %v", containerName, err)
		}
		if !ok {
			t.Fatalf("Container %s is not running", containerName)
		}
	}
	//cleanupDockerEnvironment(t, composeFilePath)
}

func cleanupDockerEnvironment(t *testing.T, composeFilePath string) {
	t.Log("Cleaning up Docker environment...")
	cmd := exec.Command("docker-compose", "-f", composeFilePath, "down")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err := cmd.Run()
	if err != nil {
		t.Fatalf("Failed to clean up Docker environment: %v", err)
	}
}
