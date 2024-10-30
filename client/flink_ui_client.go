package client

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
)

type JobList struct {
	Jobs []Job `json:"jobs"`
}

type Job struct {
	ID     string `json:"id"`
	Status string `json:"status"`
}

type JobDetail struct {
	Name      string `json:"name"`
	State     string `json:"state"`
	StartTime int64  `json:"start-time"`
	EndTime   int64  `json:"end-time"`
	Duration  int64  `json:"duration"`
}

type FlinkUiClient struct {
	BaseURL string
}

func NewFlinkUiClient(baseURL string) *FlinkUiClient {
	return &FlinkUiClient{
		BaseURL: baseURL,
	}
}

func (client *FlinkUiClient) GetJobsList() ([]JobDetail, error) {
	url := fmt.Sprintf("%s/v1/jobs", client.BaseURL)
	resp, err := http.Get(url)
	if err != nil {
		return nil, fmt.Errorf("failed to get jobs list: %v", err)
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("failed to read response body: %v", err)
	}

	var result struct {
		Jobs []struct {
			ID string `json:"id"`
		} `json:"jobs"`
	}
	err = json.Unmarshal(body, &result)
	if err != nil {
		return nil, fmt.Errorf("failed to unmarshal job list: %v", err)
	}

	var jobDetails []JobDetail
	for _, job := range result.Jobs {
		detail, err := client.GetJobDetail(job.ID)
		if err != nil {
			return nil, fmt.Errorf("failed to get job detail for job %s: %v", job.ID, err)
		}
		jobDetails = append(jobDetails, *detail)
	}

	return jobDetails, nil
}

func (client *FlinkUiClient) GetJobDetail(jobID string) (*JobDetail, error) {
	url := fmt.Sprintf("%s/v1/jobs/%s", client.BaseURL, jobID)
	resp, err := http.Get(url)
	if err != nil {
		return nil, fmt.Errorf("failed to get job detail: %v", err)
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("failed to read response body: %v", err)
	}

	var jobDetail JobDetail
	err = json.Unmarshal(body, &jobDetail)
	if err != nil {
		return nil, fmt.Errorf("failed to unmarshal job detail: %v", err)
	}

	return &jobDetail, nil
}
