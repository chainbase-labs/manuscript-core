package commands

import (
	"fmt"
	"log"
	"manuscript-core/cli/static"
	"os"
	"os/exec"
	"path/filepath"
)

// DeployManuscript 部署 Manuscript 文件
func DeployManuscript(args []string) {
	if len(args) < 1 {
		log.Fatalf("Error: Manuscript path is required as the first argument.")
	}
	manuscriptPath := args[0]
	manuscriptRunner := static.ManuscriptRunner

	// 判断文件是否存在
	if _, err := os.Stat(manuscriptPath); os.IsNotExist(err) {
		log.Fatalf("Error: Manuscript file does not exist: %s", manuscriptPath)
	}

	// Step 1: 定义所有步骤
	steps := []struct {
		name string
		fn   func() error
	}{
		{"Step 1: Copying Manuscript file", func() error { return copyManuscriptFile(manuscriptPath) }},
		{"Step 2: Copying Manuscript file", func() error { return copyManuscriptRunnerFile(manuscriptRunner) }},
		{"Step 3: Running Docker command", func() error { return runDockerCommand(manuscriptPath) }},
	}

	// Step 2: 顺序执行每个步骤，并显示加载动画
	for _, step := range steps {
		err := executeStepWithLoading(step.name, step.fn)
		if err != nil {
			log.Fatalf("\033[31m✗ %s failed: %v\n", step.name, err)
		}
	}
	fmt.Println("\033[32m✓ Manuscript deployment completed successfully!")
}

// Step 2: 复制 Manuscript 文件到 proof 目录
func copyManuscriptFile(manuscriptPath string) error {
	currentDir, err := os.Getwd()
	if err != nil {
		return fmt.Errorf("failed to get current directory: %w", err)
	}

	proofDir := filepath.Join(currentDir, "manuscript/proof")
	manuscriptFileName := filepath.Base(manuscriptPath)
	proofFilePath := filepath.Join(proofDir, manuscriptFileName)
	return copyFile(manuscriptPath, proofFilePath)
}

func copyManuscriptRunnerFile(manuscriptPath string) error {
	currentDir, err := os.Getwd()
	if err != nil {
		return fmt.Errorf("failed to get current directory: %w", err)
	}

	proofRunner := filepath.Join(currentDir, "manuscript/proof/manuscript_runner.py")
	err = os.WriteFile(proofRunner, []byte(manuscriptPath), 0644)
	if err != nil {
		return fmt.Errorf("failed to write docker-compose.yml file: %w", err)
	}
	return nil
}

// Step 3: 执行 Docker 命令
func runDockerCommand(manuscriptPath string) error {
	//currentDir, err := os.Getwd()
	//if err != nil {
	//	return fmt.Errorf("failed to get current directory: %w", err)
	//}

	manuscriptFileName := filepath.Base(manuscriptPath)
	//proofFilePath := filepath.Join(currentDir, "proof", manuscriptFileName)

	// 构造 Docker 命令
	dockerCmd := exec.Command("docker-compose", "-f", "manuscript/docker-compose.yml", "exec", "-T", "jobmanager",
		"./bin/flink", "run", "-py", "/opt/proof/manuscript_runner.py", "--config", fmt.Sprintf("/opt/proof/%s", manuscriptFileName))
	dockerCmd.Stdout = os.Stdout
	dockerCmd.Stderr = os.Stderr

	// 执行 Docker 命令
	fmt.Printf("Executing Docker command to deploy Manuscript: %s\n", dockerCmd.String())
	err := dockerCmd.Run()
	if err != nil {
		return fmt.Errorf("failed to execute Docker command: %w", err)
	}

	return nil
}

// copyFile 复制文件从 src 到 dst
func copyFile(src, dst string) error {
	sourceFile, err := os.Open(src)
	if err != nil {
		return fmt.Errorf("failed to open source file: %w", err)
	}
	defer sourceFile.Close()

	destinationFile, err := os.Create(dst)
	if err != nil {
		return fmt.Errorf("failed to create destination file: %w", err)
	}
	defer destinationFile.Close()

	// 使用缓冲区复制文件内容
	buffer := make([]byte, 1024)
	for {
		n, err := sourceFile.Read(buffer)
		if err != nil {
			if err.Error() == "EOF" {
				break
			}
			return fmt.Errorf("failed to read from source file: %w", err)
		}
		if _, err := destinationFile.Write(buffer[:n]); err != nil {
			return fmt.Errorf("failed to write to destination file: %w", err)
		}
	}
	return nil
}
