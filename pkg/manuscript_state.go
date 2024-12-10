package pkg

import (
	"bufio"
	"context"
	"fmt"
	"net/http"
	"os/exec"
	"regexp"
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
	state, err := sd.analyzeFlinkLogs(jobManagerName)
	if err != nil {
		return StateUnknown, fmt.Errorf("failed to analyze logs: %w", err)
	}

	// If the state is RUNNING, check if the GraphQL endpoint is reachable. If it's not -> INITIALIZING
	if state == StateRunning && !sd.checkGraphQLEndpoint() {
		return StateInitializing, nil
	}

	return state, nil
}

// analyzeFlinkLogs scans the logs of a Flink container to determine the current state
func (sd *StateDetector) analyzeFlinkLogs(containerName string) (ManuscriptState, error) {
	// Create a context that will timeout after 5 seconds and ensure cleanup with defer
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	// Get the last 500 lines of logs
	logs, err := GetContainerLogs(ctx, containerName, 500)
	if err != nil {
		return StateUnknown, fmt.Errorf("failed to get container logs: %w", err)
	}

	// Create variables to store the most recent relevant log entries
	var lastRunningMatch string
	var lastInitMatch string
	var lastFailMatch string
	var lastFinishMatch string

	// Define regular expressions to match relevant log entries
	patterns := map[string]*regexp.Regexp{
		string(StateRunning): regexp.MustCompile(
			`(Job .+\(.+\) (switched to RUNNING|switched from .+ to RUNNING)|` +
				`Completed checkpoint \d+ for job)`),
		string(StateFailed): regexp.MustCompile(
			`(Job [a-f0-9-]+ \(.*\) switched to FAILED|` +
				`Exception in thread|Error|FATAL|` +
				`Task failure|JobManager failure)`),
		string(StateInitializing): regexp.MustCompile(
			`(Starting JobManager|` +
				`Starting TaskManager|` +
				`Created new job|` +
				`Submitting job|` +
				`Job [a-f0-9-]+ \(.*\) is being submitted)`),
		string(StateRunning) + "_FINISHED": regexp.MustCompile(`Job [a-f0-9-]+ \(.*\) switched to FINISHED`),
	}

	// Scan logs from newest to oldest
	for i := len(logs) - 1; i >= 0; i-- {
		line := logs[i]
		// Check if the log entry matches any of the relevant patterns
		if patterns[string(StateRunning)].MatchString(line) && lastRunningMatch == "" {
			lastRunningMatch = line
		}
		if patterns[string(StateFailed)].MatchString(line) && lastFailMatch == "" {
			lastFailMatch = line
		}
		if patterns[string(StateInitializing)].MatchString(line) && lastInitMatch == "" {
			lastInitMatch = line
		}
		if patterns[string(StateRunning)+"_FINISHED"].MatchString(line) && lastFinishMatch == "" {
			lastFinishMatch = line
		}
	}

	// Return state based on most recent relevant log entry
	if lastFinishMatch != "" {
		// If we found a FINISHED state and there's no FAILED state after it -> RUNNING
		if lastFailMatch == "" || strings.Index(lastFinishMatch, lastFailMatch) > 0 {
			return StateRunning, nil
		}
	}
	if lastRunningMatch != "" {
		// If we found a RUNNING state and there's no FAILED state after it -> RUNNING
		if lastFailMatch == "" || strings.Index(lastRunningMatch, lastFailMatch) > 0 {
			return StateRunning, nil
		}
	}
	if lastFailMatch != "" {
		// If we found a FAILED pattern match, and it survived other checks -> FAILED
		return StateFailed, nil
	}
	if lastInitMatch != "" {
		// If we found an INITIALIZING pattern match, and it survived other checks -> INITIALIZING
		return StateInitializing, nil
	}

	// By default, if a manuscript survives all checks, return RUNNING state
	return StateRunning, nil
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
