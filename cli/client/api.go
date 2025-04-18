package client

import (
	"bufio"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"os"
	"os/exec"
	"strconv"
	"strings"
)

var (
	apiPort    int
	apiProcess *os.Process
)

func Init() error {
	port, proc, err := startAPIServerProcess()
	if err != nil {
		return err
	}

	apiPort = port
	apiProcess = proc
	return nil
}

func startAPIServerProcess() (int, *os.Process, error) {
	cmd := exec.Command("../common/api_server")
	stdout, err := cmd.StdoutPipe()
	if err != nil {
		return 0, nil, err
	}

	if err := cmd.Start(); err != nil {
		return 0, nil, err
	}

	scanner := bufio.NewScanner(stdout)
	for scanner.Scan() {
		line := scanner.Text()
		if strings.HasPrefix(line, "API server started on port ") {
			portStr := strings.TrimPrefix(line, "API server started on port ")
			port, err := strconv.Atoi(portStr)
			if err != nil {
				return 0, nil, err
			}
			return port, cmd.Process, nil
		}
	}

	return 0, nil, fmt.Errorf("failed to detect API server port")
}

func getAPIBaseURL() string {
	return fmt.Sprintf("http://localhost:%d", apiPort)
}
func LoadConfig(filePath string) (string, error) {
	if apiPort == 0 {
		return "", fmt.Errorf("API server port is not initialized")
	}

	endpoint := fmt.Sprintf("http://localhost:%d/load_config", apiPort)
	params := url.Values{}
	params.Set("path", filePath)
	fullURL := fmt.Sprintf("%s?%s", endpoint, params.Encode())

	resp, err := http.Get(fullURL)
	if err != nil {
		return "", fmt.Errorf("failed to send request to %s: %w", fullURL, err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		bodyBytes, _ := io.ReadAll(resp.Body)
		return "", fmt.Errorf("API returned status %d: %s", resp.StatusCode, string(bodyBytes))
	}

	bodyBytes, err := io.ReadAll(resp.Body)
	if err != nil {
		return "", fmt.Errorf("failed to read response body: %w", err)
	}

	return string(bodyBytes), nil
}

func ListJobStatuses(filePath string) (string, error) {
	if apiPort == 0 {
		return "", fmt.Errorf("API server port is not initialized")
	}

	endpoint := fmt.Sprintf("http://localhost:%d/list_job_statuses", apiPort)
	params := url.Values{}
	params.Set("path", filePath)
	fullURL := fmt.Sprintf("%s?%s", endpoint, params.Encode())

	resp, err := http.Get(fullURL)
	if err != nil {
		return "", fmt.Errorf("failed to send request to %s: %w", fullURL, err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		bodyBytes, _ := io.ReadAll(resp.Body)
		return "", fmt.Errorf("API returned status %d: %s", resp.StatusCode, string(bodyBytes))
	}

	bodyBytes, err := io.ReadAll(resp.Body)
	if err != nil {
		return "", fmt.Errorf("failed to read response body: %w", err)
	}

	return string(bodyBytes), nil
}

func GetJobStatus(baseDir, jobName string) (string, error) {
	if apiPort == 0 {
		return "", fmt.Errorf("API server port is not initialized")
	}

	endpoint := fmt.Sprintf("http://localhost:%d/get_job_status", apiPort)
	params := url.Values{}
	params.Set("baseDir", baseDir)
	params.Set("jobName", jobName)
	fullURL := fmt.Sprintf("%s?%s", endpoint, params.Encode())

	resp, err := http.Get(fullURL)
	if err != nil {
		return "", fmt.Errorf("failed to send request to %s: %w", fullURL, err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		bodyBytes, _ := io.ReadAll(resp.Body)
		return "", fmt.Errorf("API returned status %d: %s", resp.StatusCode, string(bodyBytes))
	}

	bodyBytes, err := io.ReadAll(resp.Body)
	if err != nil {
		return "", fmt.Errorf("failed to read response body: %w", err)
	}

	return string(bodyBytes), nil
}

func Shutdown() {
	if apiProcess != nil {
		_ = apiProcess.Kill()
	}
}
