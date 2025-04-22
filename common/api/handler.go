package api

import (
	"encoding/json"
	"fmt"
	"manuscript-core/common/config"
	"manuscript-core/common/jobs"
	"net/http"
)

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
