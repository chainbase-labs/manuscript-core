package client

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/stretchr/testify/assert"
)

var mockResponse = ChainResponse{
	Code:    200,
	Message: "success",
	GraphData: []struct {
		Chain struct {
			ID             string                 `json:"id"`
			Name           string                 `json:"name"`
			DatabaseName   string                 `json:"databaseName"`
			DataDictionary map[string][]TableInfo `json:"dataDictionary"`
		} `json:"chain"`
	}{
		{
			Chain: struct {
				ID             string                 `json:"id"`
				Name           string                 `json:"name"`
				DatabaseName   string                 `json:"databaseName"`
				DataDictionary map[string][]TableInfo `json:"dataDictionary"`
			}{
				ID:           "14801",
				Name:         "Vana_Satori_Testnet",
				DatabaseName: "vana",
				DataDictionary: map[string][]TableInfo{
					"blocks": {
						{Name: "block_number", DataType: "bigint", Description: "Block number uniquely identifying the block"},
					},
					"transactions": {
						{Name: "hash", DataType: "varchar(66)", Description: "Hash value representing the unique identity of the transaction"},
					},
				},
			},
		},
	},
}

func TestGetChainBaseDatasetList(t *testing.T) {
	t.Skip("Skip GetChainBaseDatasetList test")
	mockServer := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/v1/metadata/network_chains" {
			http.Error(w, "Not Found", http.StatusNotFound)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(mockResponse)
	}))
	defer mockServer.Close()

	client := NewChainBaseClient(mockServer.URL, "/api/v1/metadata/network_chains")

	chains, err := client.GetChainBaseDatasetList()

	assert.NoError(t, err, "Expected no error while fetching chain list")
	assert.Len(t, chains, 1, "Expected one chain in the response")

	expectedChain := ChainBaseDatasetListItem{
		Name:         "Vana_Satori_Testnet",
		DatabaseName: "vana",
		Tables:       []string{"blocks", "transactions"},
	}
	assert.Equal(t, expectedChain, chains[0], "Expected chain data does not match")
}
