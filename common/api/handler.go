package api

import (
	"encoding/json"
	"fmt"
	"io"
	"manuscript-core/common/config"
	"manuscript-core/common/jobs"
	"manuscript-core/common/operation"
	"net/http"
)

func getManuscriptBaseNameHandler(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusOK)
	w.Write([]byte(config.ManuscriptBaseName))
}

func getChainbaseAPIURLHandler(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusOK)
	w.Write([]byte(config.ChainbaseAPIURL))
}

func getPlatformChainURLHandler(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusOK)
	w.Write([]byte(config.PlatformChainURL))
}

func getNetworkChainEndpointHandler(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusOK)
	w.Write([]byte(config.NetworkChainEndpoint))
}

func getPlatformChainEndpointHandler(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusOK)
	w.Write([]byte(config.PlatformChainEndpoint))
}

func getMsStudioURLHandler(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusOK)
	w.Write([]byte(config.MsStudioURL))
}

// healthHandler handles the /health endpoint
func healthHandler(w http.ResponseWriter, r *http.Request) {
	_, _ = fmt.Fprint(w, "OK")
}

// loadConfigHandler handles the /load_config endpoint
func loadConfigHandler(w http.ResponseWriter, r *http.Request) {
	query := r.URL.Query()
	filePath := query.Get("path")
	if filePath == "" {
		http.Error(w, "missing 'path' query parameter", http.StatusBadRequest)
		return
	}

	config, err := config.LoadConfig(filePath)
	if err != nil {
		http.Error(w, fmt.Sprintf("failed to load config: %v", err), http.StatusInternalServerError)
		return
	}

	_ = json.NewEncoder(w).Encode(config)
}

// getJobStatusHandler handles the /get_job_status endpoint
func getJobStatusHandler(w http.ResponseWriter, r *http.Request) {
	query := r.URL.Query()
	baseDir := query.Get("baseDir")
	jobName := query.Get("jobName")
	if baseDir == "" {
		http.Error(w, "missing 'baseDir' query parameter", http.StatusBadRequest)
		return
	}

	if jobName == "" {
		http.Error(w, "missing 'jobName' query parameter", http.StatusBadRequest)
		return
	}

	status, err := jobs.GetJobStatus(baseDir, jobName)
	if err != nil {
		http.Error(w, fmt.Sprintf("failed to get job status: %v", err), http.StatusInternalServerError)
		return
	}

	_ = json.NewEncoder(w).Encode(status)
}

// listJobStatusesHandler handles the /list_job_statuses endpoint
func listJobStatusesHandler(w http.ResponseWriter, r *http.Request) {
	query := r.URL.Query()
	filePath := query.Get("path")
	if filePath == "" {
		http.Error(w, "missing 'path' query parameter", http.StatusBadRequest)
		return
	}

	statuses, err := jobs.ListJobStatuses(filePath)
	if err != nil {
		http.Error(w, fmt.Sprintf("failed to list job statuses: %v", err), http.StatusInternalServerError)
		return
	}

	_ = json.NewEncoder(w).Encode(statuses)
}

// deployHandler handles the /deploy endpoint for deploying manuscripts.
//
// Endpoint:
//
//	POST deploy?hash={hash}
//
// Content-Type:
//
//	application/json
//
// Request Body (JSON):
//
//	{
//	  "api_key": "your_api_key",           // Required. API key used for authentication.
//	  "content": "YAML content as string", // Required. The full manuscript YAML content.
//	  "schema": "some_schema",             // Optional. Logical schema name or environment label.
//	  "version": "v1.0.1"                  // Optional. Version tag for deployment.
//	}
//
// Example cURL:
//
//	curl -X POST \
//	  'http://localhost:8080/deploy?hash=xxx' \
//	  -H 'Content-Type: application/json' \
//	  -d '{
//	    "api_key": "xxxx",
//	    "content": "your_yaml_content_here",
//	    "schema": "some_schema",
//	    "version": "v1.0.1"
//	  }'
func deployHandler(w http.ResponseWriter, r *http.Request) {
	hash := r.URL.Query().Get("hash")
	if hash == "" {
		http.Error(w, "Missing 'hash' query parameter", http.StatusBadRequest)
		return
	}

	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "Failed to read request body", http.StatusBadRequest)
		return
	}
	defer r.Body.Close()

	var req operation.DeployPayload
	if err := json.Unmarshal(body, &req); err != nil {
		http.Error(w, "Invalid JSON format", http.StatusBadRequest)
		return
	}

	if req.ApiKey == "" || req.Content == "" || req.Version == "" || req.Schema == "" {
		http.Error(w, "Missing required fields: api_key, content, version, schema", http.StatusBadRequest)
		return
	}

	err = operation.DeployManuscript(req.ApiKey, hash, req.Content, req.Schema, req.Version)
	if err != nil {
		http.Error(w, fmt.Sprintf("Deployment failed: %v", err), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte("Deployment succeeded"))
}

// getTableSchemaHandler handles the /get_table_schema endpoint
// Example: GET /get_table_schema?port=8080&table=blocks
func getTableSchemaHandler(w http.ResponseWriter, r *http.Request) {
	query := r.URL.Query()
	port := query.Get("port")
	table := query.Get("table")

	if port == "" {
		http.Error(w, "missing 'port' query parameter", http.StatusBadRequest)
		return
	}
	if table == "" {
		http.Error(w, "missing 'table' query parameter", http.StatusBadRequest)
		return
	}

	schema, err := jobs.GetTableSchema(port, table)
	if err != nil {
		http.Error(w, fmt.Sprintf("failed to get table schema: %v", err), http.StatusInternalServerError)
		return
	}

	// Set response content-type and return raw schema
	w.Header().Set("Content-Type", "application/json")
	if err := json.NewEncoder(w).Encode(schema); err != nil {
		http.Error(w, fmt.Sprintf("failed to encode response: %v", err), http.StatusInternalServerError)
	}
}

// trackTableHandler handles the /track_table endpoint
// Example: GET /track_table?port=8080&table=blocks
func trackTableHandler(w http.ResponseWriter, r *http.Request) {
	query := r.URL.Query()
	port := query.Get("port")
	table := query.Get("table")

	if port == "" {
		http.Error(w, "missing 'port' query parameter", http.StatusBadRequest)
		return
	}
	if table == "" {
		http.Error(w, "missing 'table' query parameter", http.StatusBadRequest)
		return
	}

	jobs.TrackTable(port, table)
	w.WriteHeader(http.StatusOK)
}
