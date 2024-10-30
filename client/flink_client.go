package client

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strings"
	"time"
)

type FlinkClient struct {
	Endpoint  string
	SessionID string
	InitSQLs  []string
}

func NewFlinkClient(endpoint string, initSQL string) *FlinkClient {
	sqls := strings.Split(initSQL, ";")
	return &FlinkClient{
		Endpoint: endpoint,
		InitSQLs: sqls,
	}
}

func (fc *FlinkClient) CreateSession() error {
	url := fmt.Sprintf("http://%s/v1/sessions", fc.Endpoint)
	resp, err := http.Post(url, "application/json", nil)
	if err != nil {
		return fmt.Errorf("failed to create session: %v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("failed to create session, status: %s", resp.Status)
	}

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return fmt.Errorf("failed to read response: %v", err)
	}

	var result map[string]string
	if err := json.Unmarshal(body, &result); err != nil {
		return fmt.Errorf("failed to unmarshal response: %v", err)
	}

	fc.SessionID, _ = result["sessionHandle"]
	fmt.Printf("Session created: %s\n", fc.SessionID)
	return nil
}

func (fc *FlinkClient) ExecuteSQL(sql string) (string, error) {
	url := fmt.Sprintf("http://%s/v1/sessions/%s/statements", fc.Endpoint, fc.SessionID)
	requestBody, _ := json.Marshal(map[string]string{
		"statement": sql,
	})

	resp, err := http.Post(url, "application/json", bytes.NewBuffer(requestBody))
	if err != nil {
		return "", fmt.Errorf("failed to execute sql: %v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return "", fmt.Errorf("failed to execute sql, status: %s", resp.Status)
	}

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return "", fmt.Errorf("failed to read response: %v", err)
	}

	var result map[string]string
	if err := json.Unmarshal(body, &result); err != nil {
		return "", fmt.Errorf("failed to unmarshal response: %v", err)
	}

	operationHandle, _ := result["operationHandle"]
	return operationHandle, nil
}

func (fc *FlinkClient) CheckSQLResult(operationHandle string, timeout int) error {
	url := fmt.Sprintf("http://%s/v1/sessions/%s/operations/%s/result/0", fc.Endpoint, fc.SessionID, operationHandle)

	maxRetries := timeout
	retryInterval := 1000 * time.Millisecond

	for i := 0; i < maxRetries; i++ {
		resp, err := http.Get(url)
		if err != nil {
			return fmt.Errorf("failed to check sql result: %v", err)
		}
		defer resp.Body.Close()

		if resp.StatusCode == http.StatusOK {
			body, err := io.ReadAll(resp.Body)
			if err != nil {
				return fmt.Errorf("failed to read response: %v", err)
			}

			var result map[string]interface{}
			if err := json.Unmarshal(body, &result); err != nil {
				return fmt.Errorf("failed to unmarshal response: %v", err)
			}

			resultKind, _ := result["resultKind"].(string)
			if strings.Contains(resultKind, "SUCCESS") {
				return nil
			}
		} else if resp.StatusCode == http.StatusNotFound {
			fmt.Printf("Result not found, retrying... (%d/%d)\n", i+1, maxRetries)
		} else {
			return fmt.Errorf("failed to check sql result, status: %s", resp.Status)
		}

		time.Sleep(retryInterval)
	}

	return fmt.Errorf("max retries reached, operation %s result not found", operationHandle)
}

func (fc *FlinkClient) InitializeClient() error {
	if err := fc.CreateSession(); err != nil {
		return err
	}

	for _, sql := range fc.InitSQLs {
		sql = strings.TrimSpace(sql)
		if sql == "" {
			continue
		}

		operationHandle, err := fc.ExecuteSQL(sql)
		if err != nil {
			return fmt.Errorf("failed to execute init sql: %v", err)
		}

		if err := fc.CheckSQLResult(operationHandle, 30); err != nil {
			return fmt.Errorf("failed to check sql result: %v", err)
		}
	}

	fmt.Println("Client initialized successfully")
	return nil
}
