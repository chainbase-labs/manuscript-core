package pkg

import (
	"bufio"
	"context"
	"encoding/json"
	"fmt"
	"manuscript-core/client"
	"net/http"
	"os/exec"
	"strings"
	"time"
)

// ManuscriptState represents the current state of a Manuscript job
type ManuscriptState string

// Possible states of a Manuscript job
const (
	StateRunning          ManuscriptState = "RUNNING"
	StatePending          ManuscriptState = "PENDING"
	StateFailed           ManuscriptState = "FAILED"
	StateNotStarted       ManuscriptState = "NOT STARTED"
	StatePullingImage     ManuscriptState = "PULLING IMAGE"
	StateCreating         ManuscriptState = "CREATING"
	StateExited           ManuscriptState = "EXITED"
	StateDead             ManuscriptState = "DEAD"
	StatePaused           ManuscriptState = "PAUSED"
	StatePartiallyRunning ManuscriptState = "PARTIALLY RUNNING"
	StateUnknown          ManuscriptState = "UNKNOWN"
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

// findContainer represents information about a Docker container
func (sd *StateDetector) findContainer(name string) *ContainerInfo {
	for _, container := range sd.containers {
		if container.Name == name {
			return &container
		}
	}
	return nil
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
	if sd.manuscript.GraphQLPort == 0 {
		return true
	}
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

func MapStatusToState(status string) ManuscriptState {
	switch strings.ToLower(status) {
	case "unknown":
		return StateUnknown
	case "not_started":
		return StateNotStarted
	case "running":
		return StateRunning
	case "created":
		return StateCreating
	case "pending":
		return StatePending
	case "failed":
		return StateFailed
	case "pulling", "pullingimage":
		return StatePullingImage
	case "exited":
		return StateExited
	case "dead":
		return StateDead
	case "paused":
		return StatePaused
	case "partially_running":
		return StatePartiallyRunning
	default:
		return StatePending
	}
}

type ContainerStatus struct {
	Name   string `json:"Name"`
	State  string `json:"State"`
	Status string `json:"Status"`
}

type JobStatus struct {
	Name            string            `json:"Name"`
	Status          string            `json:"Status"`
	ContainerStatus []ContainerStatus `json:"ContainerStatus"`
}

func GetJobStatus(ms Manuscript) (*ManuscriptState, error) {
	var jobStatus JobStatus
	resp, err := client.GetJobStatus(ms.BaseDir, ms.Name)
	if err != nil {
		return nil, err
	}
	err = json.Unmarshal([]byte(resp), &jobStatus)
	if err != nil {
		return nil, err
	}
	state := MapStatusToState(jobStatus.Status)
	return &state, err
}
