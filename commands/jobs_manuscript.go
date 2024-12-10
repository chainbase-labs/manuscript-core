package commands

import (
	"bytes"
	"encoding/json"
	"fmt"
	"log"
	"manuscript-core/pkg"
	"net/http"
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

func ListJobs() {
	_ = pkg.ExecuteStepWithLoading("Checking jobs", false, func() error {
		dockers, err := pkg.RunDockerPs()
		if err != nil {
			log.Fatalf("Error: Failed to get docker ps: %v", err)
		}

		if len(dockers) == 0 {
			fmt.Println("\r🟡 There are no jobs running...")
			return nil
		}

		manuscripts, err := pkg.LoadConfig(manuscriptConfig)
		if err != nil {
			return fmt.Errorf("failed to load configuration: %w", err)
		}

		jobNumber := 0
		for _, m := range manuscripts.Manuscripts {
			jobNumber++

			detector := pkg.NewStateDetector(&m, dockers)
			state, err := detector.DetectState()
			if err != nil {
				log.Printf("Warning: Failed to detect state for %s: %v", m.Name, err)
				state = pkg.StateUnknown
			}

			displayJobStatus(jobNumber, &m, state)
		}
		return nil
	})
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
		err = pkg.StopDockerCompose(fmt.Sprintf("%s/%s/%s/docker-compose.yml", msConfig.BaseDir, manuscriptBaseName, jobName))
		if err != nil {
			log.Fatalf("Error: Failed to stop job: %v", err)
		}
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
