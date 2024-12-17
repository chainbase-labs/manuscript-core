package pkg

import (
	"bufio"
	"context"
	"fmt"
	"net/http"
	"os/exec"
	"strings"
	"time"
)

// ManuscriptState represents the current state of a Manuscript job
type ManuscriptState string

// Possible states of a Manuscript job
const (
	StateRunning      ManuscriptState = "RUNNING"
	StateInitializing ManuscriptState = "INITIALIZING"
	StateFailed       ManuscriptState = "FAILED"
	StateStopped      ManuscriptState = "STOPPED"
	StateUnknown      ManuscriptState = "UNKNOWN"
)

// StateDetector is responsible for detecting the current state of a Manuscript job
type StateDetector struct {
	manuscript *Manuscript
	containers []ContainerInfo
}

// NewStateDetector creates a new StateDetector instance
func NewStateDetector(manuscript *Manuscript, containers []ContainerInfo) *StateDetector {
	return &StateDetector{
		manuscript: manuscript,
		containers: containers,
	}
}

// ContainerInfo represents information about a Docker container
func (sd *StateDetector) findContainer(name string) *ContainerInfo {
	for _, container := range sd.containers {
		if container.Name == name {
			return &container
		}
	}
	return nil
}

// DetectState determines the current state of the Manuscript job
func (sd *StateDetector) DetectState() (ManuscriptState, error) {
	// Find the job and task manager containers
	jobManagerName := fmt.Sprintf("%s-jobmanager-1", sd.manuscript.Name)
	taskManagerName := fmt.Sprintf("%s-taskmanager-1", sd.manuscript.Name)

	jobManager := sd.findContainer(jobManagerName)
	taskManager := sd.findContainer(taskManagerName)

	// If no containers are found, the job is considered STOPPED
	if jobManager == nil && taskManager == nil {
		return StateStopped, nil
	}

	// Check for FAILED state based on container status
	if jobManager != nil && strings.Contains(jobManager.Status, "Exit") {
		return StateFailed, nil
	}
	if taskManager != nil && strings.Contains(taskManager.Status, "Exit") {
		return StateFailed, nil
	}

	// Analyze Flink logs to determine the current state. If they're unreachable -> UNKNOWN
	state, err := sd.checkContainerStatus(jobManagerName)
	if err != nil {
		return StateUnknown, fmt.Errorf("failed to analyze logs: %w", err)
	}

	// If the state is RUNNING, check if the GraphQL endpoint is reachable. If it's not -> INITIALIZING
	if state == StateRunning && !sd.checkGraphQLEndpoint() {
		return StateInitializing, nil
	}

	return state, nil
}

func (sd *StateDetector) checkContainerStatus(containerName string) (ManuscriptState, error) {
	dockers, err := RunDockerPs()
	if err != nil {
		return StateUnknown, fmt.Errorf("failed to get container logs: %w", err)
	}

	container := sd.findContainer(containerName)
	if container == nil {
		return StateUnknown, fmt.Errorf("container not found: %s", containerName)
	}

	for _, docker := range dockers {
		if docker.Name == containerName {
			if strings.Contains(docker.Status, "Up") {
				return StateRunning, nil
			} else if strings.Contains(docker.Status, "Exited") {
				return StateFailed, nil
			}
		}
	}

	return StateUnknown, nil
}

func GetContainerLogs(ctx context.Context, containerName string, lines int) ([]string, error) {
	// Create a command to get the logs of the container
	cmd := exec.CommandContext(ctx, "docker", "logs", "--tail", fmt.Sprintf("%d", lines), containerName)

	// Run the command and capture the output
	output, err := cmd.Output()
	if err != nil {
		return nil, err
	}

	// Split the output into individual log lines
	scanner := bufio.NewScanner(strings.NewReader(string(output)))
	var logs []string
	for scanner.Scan() {
		logs = append(logs, scanner.Text())
	}

	// Return the logs and any error that occurred during scanning
	return logs, scanner.Err()
}

func (sd *StateDetector) checkGraphQLEndpoint() bool {
	url := fmt.Sprintf("http://127.0.0.1:%d/healthz", sd.manuscript.GraphQLPort)
	client := &http.Client{
		Timeout: 2 * time.Second, // Short timeout to avoid hanging
	}

	resp, err := client.Get(url)
	if err != nil {
		return false
	}
	defer resp.Body.Close()

	return resp.StatusCode == http.StatusOK
}
