package jobs

import (
	"bytes"
	"encoding/json"
	"fmt"
	"log"
	"manuscript-core/common/config"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
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
		log.Printf("failed to change dir: %v", err)
		return &JobStatus{
			Name:            jobName,
			Status:          "unknown",
			ContainerStatus: []ContainerStatus{},
		}, nil
	}

	cmd := exec.Command("docker", "compose", "ps", "-a", "--format", "json")
	output, err := cmd.Output()
	if err != nil {
		if err != nil {
			return &JobStatus{
				Name:            jobName,
				Status:          "failed",
				ContainerStatus: []ContainerStatus{},
			}, nil
		}
	}

	var containers []ContainerStatus
	goos := runtime.GOOS
	if goos == "darwin" {
		// macOS: JSON array
		if err := json.Unmarshal(output, &containers); err != nil {
			fmt.Printf("failed to parse docker json array: %s; %s in %s", output, err, jobDir)
			return &JobStatus{
				Name:            jobName,
				Status:          "unknown",
				ContainerStatus: []ContainerStatus{},
			}, nil
		}
	} else {
		// Linux: each line is one JSON object
		lines := bytes.Split(output, []byte{'\n'})
		for _, line := range lines {
			if len(bytes.TrimSpace(line)) == 0 {
				continue
			}
			var c ContainerStatus
			if err := json.Unmarshal(line, &c); err != nil {
				fmt.Printf("failed to parse line json: %s; %s\n", line, err)
				return &JobStatus{
					Name:            jobName,
					Status:          "unknown",
					ContainerStatus: []ContainerStatus{},
				}, nil
			}
			containers = append(containers, c)
		}
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
