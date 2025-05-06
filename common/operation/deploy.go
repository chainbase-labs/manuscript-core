package operation

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"manuscript-core/common/config"
	"net/http"
)

type DeployPayload struct {
	ApiKey  string `json:"api_key"`
	Content string `json:"content"`
	Schema  string `json:"schema"`
	Version string `json:"version"`
}

func DeployManuscript(apiKey, hash, content, schema, version string) error {
	url := fmt.Sprintf("%s%s/%s", config.ChainbaseAPIURL, config.MsDeployEndpoint, hash)

	payloadData := DeployPayload{
		Content: content,
		Schema:  schema,
		Version: version,
	}
	payload, err := json.Marshal(payloadData)
	if err != nil {
		return fmt.Errorf("failed to marshal payload: %w", err)
	}

	req, err := http.NewRequest("POST", url, bytes.NewBuffer(payload))
	if err != nil {
		return fmt.Errorf("failed to create request: %w", err)
	}

	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("accept", "*/*")
	req.Header.Set("X-API-KEY", apiKey)

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return fmt.Errorf("request failed: %w", err)
	}
	defer resp.Body.Close()

	body, _ := ioutil.ReadAll(resp.Body)
	fmt.Println("Response status:", resp.Status)
	fmt.Println("Response body:", string(body))

	if resp.StatusCode >= 300 {
		return fmt.Errorf("server returned non-2xx status: %s,%s", resp.Status, string(body))
	}

	return nil
}
