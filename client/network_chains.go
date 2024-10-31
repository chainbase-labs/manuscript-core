package client

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"time"
)

type ChainBaseClient struct {
	baseURL    string
	httpClient *http.Client
}

type ChainBaseDatasetListItem struct {
	Name         string   `json:"name"`
	DatabaseName string   `json:"databaseName"`
	Tables       []string `json:"tables"`
}

type ChainResponse struct {
	Code      int    `json:"code"`
	Message   string `json:"message"`
	GraphData []struct {
		Chain struct {
			ID             string                 `json:"id"`
			Name           string                 `json:"name"`
			DatabaseName   string                 `json:"databaseName"`
			DataDictionary map[string][]TableInfo `json:"dataDictionary"`
		} `json:"chain"`
	} `json:"graphData"`
}

type TableInfo struct {
	Name        string `json:"name"`
	DataType    string `json:"dataType"`
	Description string `json:"description"`
}

func NewChainBaseClient(baseURL string) *ChainBaseClient {
	return &ChainBaseClient{
		baseURL: baseURL,
		httpClient: &http.Client{
			Timeout: 10 * time.Second,
		},
	}
}

func (c *ChainBaseClient) GetChainBaseDatasetList() ([]*ChainBaseDatasetListItem, error) {
	url := fmt.Sprintf("%s/api/v1/metadata/network_chains", c.baseURL)

	resp, err := c.httpClient.Get(url)
	if err != nil {
		return nil, fmt.Errorf("failed to send request: %v", err)
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("failed to read response body: %v", err)
	}

	var response ChainResponse
	if err := json.Unmarshal(body, &response); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %v", err)
	}

	var result []*ChainBaseDatasetListItem
	for _, item := range response.GraphData {
		chain := item.Chain
		tables := extractTableNames(chain.DataDictionary)
		result = append(result, &ChainBaseDatasetListItem{
			Name:         chain.Name,
			DatabaseName: chain.DatabaseName,
			Tables:       tables,
		})
	}

	return result, nil
}

func extractTableNames(dataDictionary map[string][]TableInfo) []string {
	var tables []string
	for tableName := range dataDictionary {
		tables = append(tables, tableName)
	}
	return tables
}
