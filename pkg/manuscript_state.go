package pkg

import (
	"bufio"
	"context"
	"fmt"
	"os/exec"
	"regexp"
	"strings"
	"time"
)

type ManuscriptState string

const (
	StateRunning      ManuscriptState = "RUNNING"
	StateInitializing ManuscriptState = "INITIALIZING"
	StateFailed       ManuscriptState = "FAILED"
	StateStopped      ManuscriptState = "STOPPED"
	StateUnknown      ManuscriptState = "UNKNOWN"
)

type StateDetector struct {
	manuscript *Manuscript
	containers []ContainerInfo
}

func NewStateDetector(manuscript *Manuscript, containers []ContainerInfo) *StateDetector {
	return &StateDetector{
		manuscript: manuscript,
		containers: containers,
	}
}

func (sd *StateDetector) findContainer(name string) *ContainerInfo {
	for _, container := range sd.containers {
		if container.Name == name {
			return &container
		}
	}
	return nil
}

func (sd *StateDetector) DetectState() (ManuscriptState, error) {
	jobManagerName := fmt.Sprintf("%s-jobmanager-1", sd.manuscript.Name)
	taskManagerName := fmt.Sprintf("%s-taskmanager-1", sd.manuscript.Name)

	jobManager := sd.findContainer(jobManagerName)
	taskManager := sd.findContainer(taskManagerName)

	// If no containers are found, the job is stopped
	if jobManager == nil && taskManager == nil {
		return StateStopped, nil
	}

	// Check for failed state based on container status
	if jobManager != nil && strings.Contains(jobManager.Status, "Exit") {
		return StateFailed, nil
	}
	if taskManager != nil && strings.Contains(taskManager.Status, "Exit") {
		return StateFailed, nil
	}

	// Analyze Flink logs to determine the current state
	state, err := sd.analyzeFlinkLogs(jobManagerName)
	if err != nil {
		return StateUnknown, fmt.Errorf("failed to analyze logs: %w", err)
	}

	return state, nil
}

func (sd *StateDetector) analyzeFlinkLogs(containerName string) (ManuscriptState, error) {
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	logs, err := GetContainerLogs(ctx, containerName, 500)
	if err != nil {
		return StateUnknown, fmt.Errorf("failed to get container logs: %w", err)
	}

	var lastRunningMatch string
	var lastInitMatch string
	var lastFailMatch string
	var lastFinishMatch string

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
		// If we found a FINISHED state and there's no FAILED state after it
		if lastFailMatch == "" || strings.Index(lastFinishMatch, lastFailMatch) > 0 {
			return StateRunning, nil
		}
	}
	if lastRunningMatch != "" {
		// If we found a RUNNING state and there's no FAILED state after it
		if lastFailMatch == "" || strings.Index(lastRunningMatch, lastFailMatch) > 0 {
			return StateRunning, nil
		}
	}
	if lastFailMatch != "" {
		return StateFailed, nil
	}
	if lastInitMatch != "" {
		return StateInitializing, nil
	}

	return StateInitializing, nil
}

func GetContainerLogs(ctx context.Context, containerName string, lines int) ([]string, error) {
	cmd := exec.CommandContext(ctx, "docker", "logs", "--tail", fmt.Sprintf("%d", lines), containerName)

	output, err := cmd.Output()
	if err != nil {
		return nil, err
	}

	scanner := bufio.NewScanner(strings.NewReader(string(output)))
	var logs []string
	for scanner.Scan() {
		logs = append(logs, scanner.Text())
	}

	return logs, scanner.Err()
}
