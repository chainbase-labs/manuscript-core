package commands

import (
	"io/ioutil"
	"os"
	"os/exec"
	"path/filepath"
	"testing"
	"time"
)

func TestInitManuscriptDockerIntegration(t *testing.T) {
	// 创建临时目录
	tempDir, err := ioutil.TempDir(".", "manuscript_test")
	if err != nil {
		t.Fatalf("Failed to create temporary directory: %v", err)
	}
	//defer os.RemoveAll(tempDir)

	// 切换到临时目录
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

	// 调用 InitManuscript
	InitManuscript()
	t.Log("InitManuscript was called")

	// 检查 manuscript 目录是否被创建
	manuscriptDir := "manuscript"
	t.Log(manuscriptDir)
	if _, err := os.Stat(manuscriptDir); os.IsNotExist(err) {
		t.Fatalf("manuscript directory was not created")
	}
	t.Log("manuscript directory was created111")
	// 检查 docker-compose.yml 文件是否被创建
	composeFilePath := filepath.Join(manuscriptDir, "docker-compose.yml")
	if _, err := os.Stat(composeFilePath); os.IsNotExist(err) {
		t.Fatalf("docker-compose.yml file was not created")
	}

	// 检查 Docker 容器是否正在运行
	time.Sleep(10 * time.Second) // 等待容器启动
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

	// 清理 Docker 容器
	//cleanupDockerEnvironment(t, composeFilePath)
}

// 停止并删除 Docker 容器
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
