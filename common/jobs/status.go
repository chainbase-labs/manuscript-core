package jobs

import (
	"encoding/json"
	"fmt"
	"manuscript-core/common/config"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

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

func GetJobStatus(baseDir, jobName string) (*JobStatus, error) {
	jobDir := filepath.Join(baseDir, jobName)

	if err := os.Chdir(jobDir); err != nil {
		return nil, fmt.Errorf("failed to change dir: %w", err)
	}

	cmd := exec.Command("docker", "compose", "ps", "-a", "--format", "json")
	output, err := cmd.Output()
	if err != nil {
		return nil, fmt.Errorf("failed to run docker compose: %w", err)
	}

	var containers []ContainerStatus
	if err := json.Unmarshal(output, &containers); err != nil {
		return nil, fmt.Errorf("failed to parse docker json: %w", err)
	}

	status := "pending"
	if len(containers) == 0 {
		status = "not_started"
	} else {
		stateCount := make(map[string]int)
		for _, c := range containers {
			state := strings.ToLower(c.State)
			stateCount[state]++
		}

		priority := []string{"created", "restarting", "dead", "paused"}
		for _, s := range priority {
			if count, ok := stateCount[s]; ok && count > 0 {
				status = s
				break
			}
		}

		if len(stateCount) == 1 {
			for s := range stateCount {
				status = s
			}
		} else {
			if countExited, hasExited := stateCount["exited"]; hasExited && countExited < len(containers) {
				status = "partially_running"
			}
		}
	}

	return &JobStatus{
		Name:            jobName,
		Status:          status,
		ContainerStatus: containers,
	}, nil

}

type Jobs struct {
	Jobs []JobStatus `json:"jobs"`
}

func ListJobStatuses(filePath string) (*Jobs, error) {
	msConfig, err := config.LoadConfig(filePath)
	if err != nil {
		return nil, err
	}

	var jobStatuses []JobStatus

	for _, ms := range msConfig.Manuscripts {
		status, err := GetJobStatus(ms.BaseDir, ms.Name)
		if err != nil {
			return nil, err
		}

		jobStatuses = append(jobStatuses, *status)
	}

	return &Jobs{
		Jobs: jobStatuses,
	}, nil
}
