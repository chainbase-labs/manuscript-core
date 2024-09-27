package commands

import (
	"fmt"
	"github.com/spf13/cobra"
	"os"
	"time"
)

var rootCmd = &cobra.Command{
	Use: "manuscript-cli",
	Long: "\n" +
		"\033[33m ██████\033[0m  ██    ██    ████   ████████ ███    ██ ███████    ████    ██████  ████████\n" +
		"\033[32m██\033[0m       ██    ██   ██  ██     ██    ████   ██ ██    ██  ██  ██  ██    ██ ██      \n" +
		"\033[32m██\033[0m       ██    ██  ██    ██    ██    ██ ██  ██ ██    ██ ██    ██ ██       ██      \n" +
		"\033[32m██\033[0m       ████████  ████████    ██    ██  ██ ██ ███████  ████████  ██████  ██████  \n" +
		"\033[32m██\033[0m       ██    ██  ██    ██    ██    ██   ████ ██    ██ ██    ██       ██ ██      \n" +
		"\033[32m██\033[0m       ██    ██  ██    ██    ██    ██    ███ ██    ██ ██    ██ ██    ██ ██      \n" +
		"\033[33m ██████\033[0m  ██    ██  ██    ██ ████████ ██     ██ ███████  ██    ██  ██████  ████████\n\n" +
		"Chainbase Manuscript ™ Build The World's Largest Omnichain Data Network 🚀 🚀 🚀\n" +
		"─────────────────────────────────────────────────────────────────────────────────",
}

func Execute(args []string) error {
	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	return nil
}

var shouldRoundUp bool
var initCmd = &cobra.Command{
	Use:     "init",
	Aliases: []string{"ini", "in", "i"},
	Short:   "Initialize and start Flink containers",
	Long:    "Initialize Manuscript Repository and start Flink containers",
	Args:    cobra.ExactArgs(0),
	Run: func(cmd *cobra.Command, args []string) {
		InitManuscript()
	},
}

var jobCmd = &cobra.Command{
	Use:     "job",
	Aliases: []string{"ini", "in", "i"},
	Short:   "Initialize and start Flink containers",
	Long:    "Initialize Manuscript Repository and start Flink containers",
	Args:    cobra.ExactArgs(0),
	Run: func(cmd *cobra.Command, args []string) {
		loading := []string{"⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"}
		for i := 0; i < 20; i++ {
			fmt.Printf("\r%s Loading...", loading[i%len(loading)])
			time.Sleep(100 * time.Millisecond)
		}
		fmt.Print("\rLoading complete!     \n")
	},
}

var deployCmd = &cobra.Command{
	Use:     "deploy [manuscript_path]",
	Aliases: []string{"deplo", "depl", "dep", "de", "d"},
	Short:   "Deploy manuscript to flink cluster",
	Long:    "Deploy manuscript to flink cluster",
	Args:    cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		DeployManuscript(args)
	},
}

func init() {
	//multiplyCmd.Flags().BoolVarP(&shouldRoundUp, "round", "r", false, "Round results up to 2 decimal places")
	rootCmd.AddCommand(initCmd)
	rootCmd.AddCommand(jobCmd)
	rootCmd.AddCommand(deployCmd)
	rootCmd.CompletionOptions.HiddenDefaultCmd = true
}
