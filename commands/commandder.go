package commands

import (
	"fmt"
	"github.com/spf13/cobra"
	"os"
)

func Execute(args []string) error {
	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	return nil
}

var initCmd = &cobra.Command{
	Use:     "init",
	Aliases: []string{"ini", "in", "i"},
	Short:   "Initialize and start local Flink containers",
	Long:    "Initialize Manuscript Repository and start Flink containers",
	Args:    cobra.ExactArgs(0),
	Run: func(cmd *cobra.Command, args []string) {
		InitManuscript()
	},
}

var jobCmd = &cobra.Command{
	Use:     "job",
	Aliases: []string{"jo", "j"},
	Short:   "Manage Flink jobs",
	Long:    "Manage Flink jobs, such as listing, stopping, and viewing logs of jobs",
}

var jobListCmd = &cobra.Command{
	Use:   "list",
	Short: "List all Flink jobs",
	Run: func(cmd *cobra.Command, args []string) {
		ListJobs()
	},
}

var jobStopCmd = &cobra.Command{
	Use:   "stop <job>",
	Short: "Stop a Flink job",
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		JobStop(args[0])
	},
}

var jobLogCmd = &cobra.Command{
	Use:   "log <jobid>",
	Short: "View logs of a Flink job",
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		JobLogs(args[0])
	},
}

var runManuscript = &cobra.Command{
	Use:     "run <manuscript-file>",
	Aliases: []string{"r"},
	Short:   "Run Manuscript on the local Flink cluster",
	Long:    "Run Manuscript on the local Flink cluster",
	Args:    cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		RunManuscript(args)
	},
}

var deployManuscript = &cobra.Command{
	Use:     "deploy <coming soon>",
	Aliases: []string{"d"},
	Short:   "<coming soon> deploy manuscript to chainbase network",
	Long:    "<coming soon> deploy manuscript to chainbase network",
	Args:    cobra.ExactArgs(0),
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Coming soon...")
	},
}

func init() {
	// Add the subcommands to the jobCmd
	jobCmd.AddCommand(jobListCmd)
	jobCmd.AddCommand(jobStopCmd)
	jobCmd.AddCommand(jobLogCmd)

	// Add jobCmd and other commands to the root command
	rootCmd.AddCommand(initCmd)
	rootCmd.AddCommand(jobCmd)
	rootCmd.AddCommand(runManuscript)

	// Add deployManuscript to root command
	rootCmd.AddCommand(deployManuscript)
	rootCmd.CompletionOptions.HiddenDefaultCmd = true
}

var rootCmd = &cobra.Command{
	Use: "manuscript-cli",
	Long: "\n" +
		"\033[33m ██████\033[0m ██    ██   ████   ████████ ███    ██ ███████    ████    ██████  ████████\n" +
		"\033[32m██\033[0m      ██    ██  ██  ██     ██    ████   ██ ██    ██  ██  ██  ██    ██ ██      \n" +
		"\033[32m██\033[0m      ██    ██ ██    ██    ██    ██ ██  ██ ██    ██ ██    ██ ██       ██      \n" +
		"\033[32m██\033[0m      ████████ ████████    ██    ██  ██ ██ ███████  ████████  ██████  ██████  \n" +
		"\033[32m██\033[0m      ██    ██ ██    ██    ██    ██   ████ ██    ██ ██    ██       ██ ██      \n" +
		"\033[32m██\033[0m      ██    ██ ██    ██    ██    ██    ███ ██    ██ ██    ██ ██    ██ ██      \n" +
		"\033[33m ██████\033[0m ██    ██ ██    ██ ████████ ██     ██ ███████  ██    ██  ██████  ████████\n\n" +
		"Chainbase Manuscript ™ Build The World's Largest Omnichain Data Network 🚀 🚀 🚀\n" +
		"─────────────────────────────────────────────────────────────────────────────────",
}
