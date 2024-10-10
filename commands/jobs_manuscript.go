package commands

import (
	"bytes"
	"encoding/json"
	"fmt"
	"log"
	"manuscript-core/client"
	"manuscript-core/pkg"
	"net/http"
	"strconv"
	"strings"
	"time"
)

const GraphQLContainerName = "hasura"

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

		jobNumber := 0
		for _, d := range dockers {
			if d.Ports == nil {
				continue
			}
			c := client.NewFlinkUiClient(fmt.Sprintf("http://localhost:%s", d.Ports[0]))
			jobs, err := c.GetJobsList()
			if err != nil {
				fmt.Printf("\r🟡 %d: Name: %s | State: \033[33mInitializing...(may wait for 2 minutes)\033[0m \n", jobNumber, strings.Split(d.Name, "-jobmanager-1")[0])
			}

			for _, job := range jobs {
				jobNumber++
				startTime := formatTimestamp(job.StartTime)
				duration := formatDurationToMinutes(job.Duration)
				jobName := strings.Split(d.Name, "-jobmanager-1")[0]

				switch job.State {
				case "RUNNING":
					graphQLPort := CheckGraphQLContainer(dockers, jobName)
					if graphQLPort != 0 {
						fmt.Printf("\r🟢 %d: Name: \033[34m%s\033[0m | State: \033[32m%s\033[0m | Start Time: %s | Duration: %v | GraphQL: http://127.0.0.1:%d\n", jobNumber, jobName, job.State, startTime, duration, graphQLPort)
					} else {
						fmt.Printf("\r🟢 %d: Name: \033[34m%s\033[0m | State: \033[32m%s\033[0m | Start Time: %s | Duration: %v\n", jobNumber, jobName, job.State, startTime, duration)
					}
				case "CANCELED":
					fmt.Printf("\r🟡 %d: Name: %s | State: \033[33m%s\033[0m | Start Time: %s | Duration: %v\n", jobNumber, jobName, job.State, startTime, duration)
				default:
					fmt.Printf("\r⚪️ %d: Name: %s | State: %s | Start Time: %s | Duration: %v\n", jobNumber, jobName, job.State, startTime, duration)
				}
			}
		}
		return err
	})
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

func CheckGraphQLContainer(dockers []pkg.ContainerInfo, jobName string) int {
	graphQLContainerPort := 0
	for _, d := range dockers {
		if d.Name == fmt.Sprintf("%s-%s-1", jobName, GraphQLContainerName) {
			if len(d.Ports) > 0 {
				port, err := strconv.Atoi(d.Ports[0])
				if err != nil {
					log.Fatalf("Error: Failed to convert port to integer: %v", err)
					return 0
				}
				graphQLContainerPort = port
				trackHasuraTable(jobName, graphQLContainerPort)
			}
		}
	}
	return graphQLContainerPort
}

func trackHasuraTable(jobName string, graphQLContainerPort int) {
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

		url := fmt.Sprintf("http://127.0.0.1:%d/v1/metadata", graphQLContainerPort)
		req, err := http.NewRequest("POST", url, bytes.NewBuffer(payloadBytes))
		if err != nil {
			log.Println("Error creating request:", err)
		}
		req.Header.Set("Content-Type", "application/json")

		client := &http.Client{}
		resp, err := client.Do(req)
		if err != nil {
			log.Println("Error making request:", err)
		}
		defer resp.Body.Close()
	}
}
