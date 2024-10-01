package commands

import (
	"fmt"
	"log"
	"manuscript-core/client"
	"manuscript-core/pkg"
	"time"
)

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
	_ = pkg.ExecuteStepWithLoading("Checking jobs", func() error {
		dockers, err := pkg.RunDockerPs()
		if err != nil {
			log.Fatalf("Error: Failed to get docker ps: %v", err)
		}
		if len(dockers) == 0 {
			log.Fatalf("Error: No flink jobmanager found")
		}

		for _, d := range dockers {
			if d.Ports == nil {
				continue
			}
			c := client.NewFlinkUiClient(fmt.Sprintf("http://localhost:%s", d.Ports[0]))
			jobs, err := c.GetJobsList()
			if err != nil {
				log.Fatalf("Error: Failed to get jobs list: %v", err)
			}

			for i, job := range jobs {
				startTime := formatTimestamp(job.StartTime)
				duration := formatDurationToMinutes(job.Duration)

				switch job.State {
				case "RUNNING":
					fmt.Printf("\r🟢 %d: Name: %s | State: \033[32m%s\033[0m | Start Time: %s | Duration: %v\n", i+1, d.Name, job.State, startTime, duration)
				case "CANCELED":
					fmt.Printf("\r🟡 %d: Name: %s | State: \033[33m%s\033[0m | Start Time: %s | Duration: %v\n", i+1, d.Name, job.State, startTime, duration)
				default:
					fmt.Printf("\r⚪️ %d: Name: %s | State: %s | Start Time: %s | Duration: %v\n", i+1, job.Name, job.State, startTime, duration)
				}
			}
		}
		return err
	})
}

func JobLogs(jobName string) {
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
