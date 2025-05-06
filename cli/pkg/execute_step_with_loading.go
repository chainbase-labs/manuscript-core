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

	// clear loading line
	fmt.Printf("\r%-50s\r", "") // 用空格清除整行

	if err != nil {
		return err
	}

	if stdOut {
		fmt.Printf("\033[32m✓ %s completed successfully!\033[0m\n", stepName)
	}

	return nil
}
