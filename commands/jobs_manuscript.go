package commands

import (
	"fmt"
	"log"
	"manuscript-core/client"
	"manuscript-core/pkg"
	"strings"
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
	_ = pkg.ExecuteStepWithLoading("Checking jobs", false, func() error {
		dockers, err := pkg.RunDockerPs()
		if err != nil {
			log.Fatalf("Error: Failed to get docker ps: %v", err)
		}
		if len(dockers) == 0 {
			log.Fatalf("Error: No flink jobmanager found")
		}

		jobNumber := 0
		for _, d := range dockers {
			if d.Ports == nil {
				continue
			}
			c := client.NewFlinkUiClient(fmt.Sprintf("http://localhost:%s", d.Ports[0]))
			jobs, err := c.GetJobsList()
			if err != nil {
				fmt.Printf("\r🟡 %d: Name: %s | State: \033[33mInitializing...\033[0m \n", jobNumber, strings.Split(d.Name, "-jobmanager-1")[0])
			}

			for _, job := range jobs {
				jobNumber++
				startTime := formatTimestamp(job.StartTime)
				duration := formatDurationToMinutes(job.Duration)
				jobName := strings.Split(d.Name, "-jobmanager-1")[0]

				switch job.State {
				case "RUNNING":
					fmt.Printf("\r🟢 %d: Name: \033[34m%s\033[0m | State: \033[32m%s\033[0m | Start Time: %s | Duration: %v\n", jobNumber, jobName, job.State, startTime, duration)
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
		err := pkg.StopDockerCompose(jobName)
		if err != nil {
			log.Fatalf("Error: Failed to stop job: %v", err)
		}
		return nil
	})
	fmt.Printf("\rJob \033[33m%s\033[0m stopped successfully.\n", jobName)
}
