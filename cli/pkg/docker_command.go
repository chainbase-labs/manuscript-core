package pkg

import (
	"bufio"
	"context"
	"fmt"
	"os"
	"os/exec"
	"os/signal"
	"regexp"
	"strings"
	"syscall"
)

type ContainerInfo struct {
	ContainerID string
	Image       string
	Command     string
	Created     string
	Status      string
	Ports       []string
	Name        string
}

func extractPorts(portInfo string) []string {
	var ports []string
	regex := regexp.MustCompile(`0\.0\.0\.0:(\d+)->`)
	matches := regex.FindAllStringSubmatch(portInfo, -1)
	for _, match := range matches {
		if len(match) > 1 {
			ports = append(ports, match[1])
		}
	}
	return ports
}

// getDockerContainers filters and returns a list of Docker containers.
// It checks if the container name contains any of the following substrings:
// "manager", "postgres", or "hasura".
//
// Returns:
//
//	[]string - A list of matching container names.
func RunDockerPs() ([]ContainerInfo, error) {
	cmd := exec.Command("docker", "ps", "-a")
	output, err := cmd.Output()
	if err != nil {
		return nil, fmt.Errorf("failed to execute docker ps -a: %w", err)
	}

	lines := strings.Split(string(output), "\n")
	if len(lines) <= 1 {
		return nil, fmt.Errorf("no containers found")
	}

	var containers []ContainerInfo
	regex := regexp.MustCompile(`\s{2,}`)

	for _, line := range lines[1:] {
		if strings.TrimSpace(line) == "" {
			continue
		}

		parts := regex.Split(line, -1)
		if len(parts) < 7 {
			continue
		}

		ports := extractPorts(strings.TrimSpace(parts[5]))

		// The container name must include one of the following keywords: manager、postgres
		if !strings.Contains(parts[6], "manager") && !strings.Contains(parts[6], "postgres") && !strings.Contains(parts[6], "hasura") {
			continue
		}

		container := ContainerInfo{
			ContainerID: strings.TrimSpace(parts[0]),
			Image:       strings.TrimSpace(parts[1]),
			Command:     strings.TrimSpace(parts[2]),
			Created:     strings.TrimSpace(parts[3]),
			Status:      strings.TrimSpace(parts[4]),
			Ports:       ports,
			Name:        strings.TrimSpace(parts[6]),
		}
		containers = append(containers, container)
	}

	return containers, nil
}

func GetDockerLogs(containerName string) error {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	signalChan := make(chan os.Signal, 1)
	signal.Notify(signalChan, os.Interrupt, syscall.SIGTERM)

	go func() {
		<-signalChan
		cancel()
	}()

	cmd := exec.CommandContext(ctx, "docker", "logs", "-f", containerName)

	stdout, err := cmd.StdoutPipe()
	if err != nil {
		return fmt.Errorf("failed to get stdout: %w", err)
	}

	if err := cmd.Start(); err != nil {
		return fmt.Errorf("failed to start command: %w", err)
	}

	scanner := bufio.NewScanner(stdout)
	for scanner.Scan() {
		select {
		case <-ctx.Done():
			return nil
		default:
			fmt.Println(scanner.Text())
		}
	}

	if err := scanner.Err(); err != nil {
		return fmt.Errorf("error reading logs: %w", err)
	}

	if err := cmd.Wait(); err != nil {
		return fmt.Errorf("command finished with error: %w", err)
	}

	return nil
}

func StopDockerCompose(Name string) error {
	manuscriptDockerComposeFile := Name

	if _, err := os.Stat(manuscriptDockerComposeFile); os.IsNotExist(err) {
		return fmt.Errorf("error: Manuscript %s does not exist", Name)
	}

	if _, err := exec.LookPath("docker"); err == nil {
		cmd := exec.Command("docker", "compose", "-f", manuscriptDockerComposeFile, "down")
		err = runCommand(cmd)
		if err == nil {
			return nil
		}
		fmt.Println("Failed to stop container using 'docker compose', trying 'docker-compose':", err)
	}

	if _, err := exec.LookPath("docker-compose"); err == nil {
		cmd := exec.Command("docker-compose", "-f", manuscriptDockerComposeFile, "down")
		err = runCommand(cmd)
		if err == nil {
			return nil
		}
		return fmt.Errorf("failed to stop container using 'docker-compose': %w", err)
	}

	return fmt.Errorf("neither 'docker-compose' nor 'docker compose' command found")
}

func runCommand(cmd *exec.Cmd) error {
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	return cmd.Run()
}
