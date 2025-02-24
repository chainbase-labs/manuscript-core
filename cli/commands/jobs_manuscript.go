package commands

import (
	"bytes"
	"encoding/json"
	"fmt"
	"log"
	"manuscript-core/pkg"
	"net/http"
	"os"
	"path/filepath"
	"strings"
	"time"
)

type Args struct {
	Type string `json:"type"`
	Args struct {
		Source string `json:"source"`
		Schema string `json:"schema"`
		Name   string `json:"name"`
	} `json:"args"`
}

type Payload struct {
	Type string `json:"type"`
	Args []Args `json:"args"`
}

func formatTimestamp(ts int64) string {
	if ts == 0 {
		return "N/A"
	}
	return time.Unix(0, ts*int64(time.Millisecond)).Format("2006-01-02 15:04")
}

func formatDurationToMinutes(durationMs int64) string {
	if durationMs == 0 {
		return "N/A"
	}
	durationMinutes := durationMs / 1000 / 60
	return fmt.Sprintf("%d minutes", durationMinutes)
}

func ListJobs(config *pkg.Config) {
	_ = pkg.ExecuteStepWithLoading("Checking jobs", false, func() error {
		// Step 1: Check for running Docker containers
		dockers, err := getRunningContainers()
		if err != nil {
			return err
		}

		// Always show if no containers are running
		if len(dockers) == 0 {
			fmt.Println("\r📍 There are no jobs running...")
		}

		// Step 3: Check and display state for each manuscript
		displayManuscriptStates(config.Manuscripts, dockers)
		return nil
	})
}

// getRunningContainers retrieves all running Docker containers
func getRunningContainers() ([]pkg.ContainerInfo, error) {
	dockers, err := pkg.RunDockerPs()
	if err != nil {
		return nil, fmt.Errorf("failed to get docker processes: %w", err)
	}
	return dockers, nil
}

// getManuscripts retrieves manuscript configurations from either config file or directory
func getManuscripts(dir string) ([]pkg.Manuscript, error) {
	if dir == "" {
		return getManuscriptsFromConfig()
	}
	return getManuscriptsFromDirectory(dir)
}

// getManuscriptsFromConfig loads manuscripts from the config file
func getManuscriptsFromConfig() ([]pkg.Manuscript, error) {
	config, err := pkg.LoadConfig(manuscriptConfig)
	if err != nil {
		return nil, fmt.Errorf("failed to load configuration: %w", err)
	}
	return config.Manuscripts, nil
}

// getManuscriptsFromDirectory scans directory for manuscript.yaml files
func getManuscriptsFromDirectory(dir string) ([]pkg.Manuscript, error) {
	// Check if directory exists
	if _, err := os.Stat(dir); os.IsNotExist(err) {
		return nil, fmt.Errorf("directory does not exist: %s", dir)
	}

	var manuscripts []pkg.Manuscript
	// Read only the immediate subdirectories
	entries, err := os.ReadDir(dir)
	if err != nil {
		return nil, fmt.Errorf("failed to read directory: %w", err)
	}

	for _, entry := range entries {
		if entry.IsDir() {
			// Check for manuscript.yaml in each immediate subdirectory
			manuscriptPath := filepath.Join(dir, entry.Name(), "manuscript.yaml")
			if _, err := os.Stat(manuscriptPath); err == nil {
				manuscript, err := pkg.ParseYAML(manuscriptPath)
				if err != nil {
					log.Printf("Warning: Failed to parse %s: %v", manuscriptPath, err)
					continue
				}
				manuscripts = append(manuscripts, *manuscript)
			}
		}
	}

	return manuscripts, nil
}

// displayManuscriptStates checks and displays the state of each manuscript
func displayManuscriptStates(manuscripts []pkg.Manuscript, dockers []pkg.ContainerInfo) {
	for i, m := range manuscripts {
		detector := pkg.NewStateDetector(&m, dockers)
		state, err := detector.DetectState()
		if err != nil {
			log.Printf("Warning: Failed to detect state for %s: %v", m.Name, err)
			state = pkg.StateUnknown
		}

		displayJobStatus(i+1, &m, state)

		if state == pkg.StateRunning {
			// hasura metadata endpoint tracks tables
			url := fmt.Sprintf("http://127.0.0.1:%d/v1/metadata", m.GraphQLPort)

			payload := fmt.Sprintf(`{
				"type": "bulk",
				"source": "default", 
				"resource_version": 1,
				"args": [{
					"type": "postgres_track_tables",
					"args": {
						"allow_warnings": true,
						"tables": [{
							"table": {
								"name": "%s",
								"schema": "public"
							},
							"source": "default"
						}]
					}
				}]
			}`, m.Table)

			resp, err := http.Post(url, "application/json", strings.NewReader(payload))
			if err != nil {
				return
			}
			defer resp.Body.Close()
		}
	}
}

func displayJobStatus(jobNumber int, m *pkg.Manuscript, state pkg.ManuscriptState) {
	switch state {
	case pkg.StateRunning:
		fmt.Printf("\r🟢 %d: Manuscript: \033[32m%s\033[0m | State: \033[32m%s\033[0m | GraphQL: http://127.0.0.1:%d\n",
			jobNumber, m.Name, state, m.GraphQLPort)
	case pkg.StateInitializing:
		fmt.Printf("\r🟡 %d: Manuscript: \033[34m%s\033[0m | State: \033[33m%s\033[0m\n",
			jobNumber, m.Name, state)
	case pkg.StateFailed:
		fmt.Printf("\r🔴 %d: Manuscript: %s | State: \033[31m%s\033[0m\n",
			jobNumber, m.Name, state)
	case pkg.StateStopped:
		fmt.Printf("\r⚫ %d: Manuscript: %s | State: \033[90m%s\033[0m\n",
			jobNumber, m.Name, state)
	default:
		fmt.Printf("\r⚪ %d: Manuscript: %s | State: %s\n",
			jobNumber, m.Name, state)
	}
}

func JobLogs(jobName string) {
	jobName = fmt.Sprintf("%s-jobmanager-1", jobName)
	dockers, err := pkg.RunDockerPs()
	if err != nil {
		log.Fatalf("Error: Failed to get docker ps: %v", err)
	}
	if len(dockers) == 0 {
		log.Fatalf("Error: No flink jobmanager found")
	}
	for _, d := range dockers {
		if d.Name == jobName {
			_ = pkg.GetDockerLogs(jobName)
			break
		}
	}
	fmt.Println("Job not found")
}

func JobStop(jobName string) {
	_ = pkg.ExecuteStepWithLoading("Stop job", false, func() error {
		msConfig, err := pkg.LoadConfig(manuscriptConfig)
		if err != nil {
			log.Fatalf("Error: Failed to load manuscript config: %v", err)
			return err
		}
		for _, manuscript := range msConfig.Manuscripts {
			if manuscript.Name == jobName {
				fmt.Printf("%s/%s/docker-compose.yml", manuscript.BaseDir, jobName)
				err = pkg.StopDockerCompose(fmt.Sprintf("%s/%s/docker-compose.yml", manuscript.BaseDir, jobName))
				if err != nil {
					log.Fatalf("Error: Failed to stop job: %v", err)
				}
				return nil
			}
		}
		log.Fatalf("Error: There is no job name = : %s", jobName)
		return nil
	})
	fmt.Printf("\rJob \033[33m%s\033[0m stopped successfully.\n", jobName)
}

func trackHasuraTable(jobName string) {
	msConfig, err := pkg.LoadConfig(manuscriptConfig)
	if err != nil {
		log.Fatalf("Error: Failed to load manuscript config: %v", err)
	}
	ms, err := ParseManuscriptYaml(fmt.Sprintf("%s/%s/%s/manuscript.yaml", msConfig.BaseDir, manuscriptBaseName, jobName))
	if err != nil {
		log.Fatalf("Error: Failed to parse manuscript yaml: %v", err)
	}
	for _, sink := range ms.Sinks {
		payload := Payload{
			Type: "bulk",
			Args: []Args{
				{
					Type: "pg_track_table",
					Args: struct {
						Source string `json:"source"`
						Schema string `json:"schema"`
						Name   string `json:"name"`
					}{
						Source: "default",
						Schema: "public",
						Name:   sink.Table,
					},
				},
			},
		}

		payloadBytes, err := json.Marshal(payload)
		if err != nil {
			log.Println("Error marshalling payload:", err)
		}

		for _, m := range msConfig.Manuscripts {
			if m.Name == jobName {
				url := fmt.Sprintf("http://127.0.0.1:%d/v1/metadata", m.GraphQLPort)
				req, err := http.NewRequest("POST", url, bytes.NewBuffer(payloadBytes))
				if err != nil {
					log.Println("Error creating request:", err)
				}
				req.Header.Set("Content-Type", "application/json")

				c := &http.Client{}
				_, err = c.Do(req)
				if err != nil {
					return
				}
			}
		}

	}
}
