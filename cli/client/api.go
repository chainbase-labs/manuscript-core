package client

import (
	"bufio"
	"bytes"
	"embed"
	_ "embed"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"os"
	"os/exec"
	"runtime"
	"strconv"
	"strings"
)

//go:embed api_server_darwin_amd64
//go:embed api_server_darwin_arm64
//go:embed api_server_linux_amd64
var apiServerFS embed.FS

func getEmbeddedBinary() ([]byte, error) {
	osArch := runtime.GOOS + "_" + runtime.GOARCH
	switch osArch {
	case "darwin_amd64":
		return apiServerFS.ReadFile("api_server_darwin_amd64")
	case "darwin_arm64":
		return apiServerFS.ReadFile("api_server_darwin_arm64")
	case "linux_amd64":
		return apiServerFS.ReadFile("api_server_linux_amd64")
	default:
		return nil, fmt.Errorf("unsupported platform: %s", osArch)
	}
}

var (
	apiPort    int
	apiProcess *os.Process
)

func Init() error {
	port, proc, err := writeAndRunAPIServer()
	if err != nil {
		return err
	}

	apiPort = port
	apiProcess = proc
	return nil
}

func writeAndRunAPIServer() (int, *os.Process, error) {
	tmpDir := "/tmp"
	tmpFile, err := os.CreateTemp(tmpDir, "api_server_exec_*")
	if err != nil {
		return 0, nil, err
	}
	tmpFilePath := tmpFile.Name()

	apiServerBin, err := getEmbeddedBinary()
	if err != nil {
		return 0, nil, err
	}

	if _, err := tmpFile.Write(apiServerBin); err != nil {
		tmpFile.Close()
		return 0, nil, err
	}

	if err := tmpFile.Chmod(0755); err != nil {
		tmpFile.Close()
		return 0, nil, err
	}

	if err := tmpFile.Close(); err != nil {
		return 0, nil, err
	}

	cmd := exec.Command(tmpFilePath)
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

func GetTableSchema(port int, table string) (string, error) {
	if apiPort == 0 {
		return "", fmt.Errorf("API server port is not initialized")
	}

	endpoint := fmt.Sprintf("http://localhost:%d/get_table_schema", apiPort)
	params := url.Values{}
	params.Set("port", strconv.Itoa(port))
	params.Set("table", table)
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

func TrackTable(port, table string) error {
	if apiPort == 0 {
		return fmt.Errorf("API server port is not initialized")
	}

	endpoint := fmt.Sprintf("http://localhost:%d/track_table", apiPort)
	params := url.Values{}
	params.Set("port", port)
	params.Set("table", table)
	fullURL := fmt.Sprintf("%s?%s", endpoint, params.Encode())

	resp, err := http.Get(fullURL)
	if err != nil {
		return fmt.Errorf("failed to send request to %s: %w", fullURL, err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		bodyBytes, _ := io.ReadAll(resp.Body)
		return fmt.Errorf("API returned status %d: %s", resp.StatusCode, string(bodyBytes))
	}

	return nil
}

func Deploy(content []byte, schema, hash, apiKey, version string) error {
	if apiPort == 0 {
		return fmt.Errorf("API server port is not initialized")
	}

	endpoint := fmt.Sprintf("http://localhost:%d/deploy", apiPort)
	params := url.Values{}
	params.Set("hash", hash)
	fullURL := fmt.Sprintf("%s?%s", endpoint, params.Encode())

	payload := map[string]interface{}{
		"api_key": apiKey,
		"content": string(content),
		"schema":  schema,
		"version": version,
	}

	body, err := json.Marshal(payload)
	if err != nil {
		return fmt.Errorf("failed to encode payload: %w", err)
	}

	req, err := http.NewRequest("POST", fullURL, bytes.NewReader(body))
	if err != nil {
		return fmt.Errorf("failed to create request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return fmt.Errorf("failed to send request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		bodyr, err := io.ReadAll(resp.Body)
		if err != nil {
			print("xx")
		}
		return fmt.Errorf("deployment failed with status: %s, %s", resp.Status, string(bodyr))
	}

	return nil
}

func Shutdown() {
	if apiProcess != nil {
		_ = apiProcess.Kill()
	}
}
