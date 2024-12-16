package pkg

import (
	"fmt"
	"time"
)

func ExecuteStepWithLoading(stepName string, stdOut bool, stepFunc func() error) error {
	done := make(chan struct{})
	loading := []string{"⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"}

	go func() {
		i := 0
		for {
			select {
			case <-done:
				return
			default:
				fmt.Printf("\r%s %s Loading...", loading[i%len(loading)], stepName)
				time.Sleep(100 * time.Millisecond)
				i++
			}
		}
	}()

	err := stepFunc()

	close(done)
	if err != nil {
		fmt.Printf("\r\033[31m✗\033[0m %s failed!\n", stepName)
		fmt.Printf("\033[31mError: %v\n", err)
	}
	if stdOut {
		fmt.Printf("\r\033[32m✓\033[0m %s completed successfully!\n", stepName)
	}

	return err
}
