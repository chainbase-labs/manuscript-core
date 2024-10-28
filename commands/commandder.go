package commands

import (
	"fmt"
	"github.com/spf13/cobra"
	"os"
)

var (
	env     string
	version = "1.0.3"
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
	Short:   "Initialize and start local manuscript containers",
	Long:    "Initialize Manuscript Repository and start manuscript containers",
	Args:    cobra.ExactArgs(0),
	Run: func(cmd *cobra.Command, args []string) {
		InitManuscript()
	},
}

var jobListCmd = &cobra.Command{
	Use:   "list",
	Short: "List all manuscript jobs",
	Run: func(cmd *cobra.Command, args []string) {
		ListJobs()
	},
}

var jobStopCmd = &cobra.Command{
	Use:   "stop <job_name>",
	Short: "Stop a manuscript job",
	Long:  "Stop a manuscript job",
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		JobStop(args[0])
	},
}

var jobLogsCmd = &cobra.Command{
	Use:   "logs <job_name>",
	Short: "View logs of a manuscript job",
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		JobLogs(args[0])
	},
}

var chatCmd = &cobra.Command{
	Use:     "chat <job_name>",
	Aliases: []string{"c"},
	Short:   "Chat with the dataset AI",
	Long:    "Chat with the dataset AI",
	Args:    cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		Chat(args[0])
	},
}

var deployManuscript = &cobra.Command{
	Use:     "deploy <manuscript-file>",
	Aliases: []string{"d"},
	Short:   "Deploy Manuscript to a local environment or the Chainbase network.",
	Long:    "Deploy Manuscript to a local environment or the Chainbase network.",
	Example: "manuscript-cli deploy /data/manuscript/demo/manuscript.yaml --env=local",
	Args:    cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		switch env {
		case "local":
			DeployManuscript(args)
		case "chainbase":
			fmt.Println("Deploying to Chainbase network...coming soon!")
		default:
			fmt.Println("Unknown environment. Please specify --env=local or --env=chainbase")
		}
	},
}

var versionCmd = &cobra.Command{
	Use:   "version",
	Short: "Show the version of manuscript-cli",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Printf("manuscript-cli version %s\n", version)
	},
}

func init() {
	rootCmd.SetHelpFunc(func(cmd *cobra.Command, args []string) {
		// List of commands to display in the help menu
		commands := []*cobra.Command{
			cmd.Commands()[4],
			cmd.Commands()[5],
			cmd.Commands()[6],
			cmd.Commands()[7],
			cmd.Commands()[2],
			cmd.Commands()[0],
		}

		maxNameLen := 0
		for _, c := range commands {
			if len(c.Name()) > maxNameLen {
				maxNameLen = len(c.Name())
			}
		}

		fmt.Printf("\n" +
			"\033[33m ██████\033[0m ██    ██   ████   ████████ ███    ██ ███████    ████    ██████  ████████\n" +
			"\033[32m██\033[0m      ██    ██  ██  ██     ██    ████   ██ ██    ██  ██  ██  ██    ██ ██      \n" +
			"\033[32m██\033[0m      ██    ██ ██    ██    ██    ██ ██  ██ ██    ██ ██    ██ ██       ██      \n" +
			"\033[32m██\033[0m      ████████ ████████    ██    ██  ██ ██ ███████  ████████  ██████  ██████  \n" +
			"\033[32m██\033[0m      ██    ██ ██    ██    ██    ██   ████ ██    ██ ██    ██       ██ ██      \n" +
			"\033[32m██\033[0m      ██    ██ ██    ██    ██    ██    ███ ██    ██ ██    ██ ██    ██ ██      \n" +
			"\033[33m ██████\033[0m ██    ██ ██    ██ ████████ ██     ██ ███████  ██    ██  ██████  ████████\n\n" +
			"Chainbase Manuscript ™ Build The World's Largest Omnichain Data Network 🚀 🚀 🚀\n" +
			"─────────────────────────────────────────────────────────────────────────────────\n")
		fmt.Printf("Usage:\n  %s\n\n", cmd.UseLine())
		fmt.Printf("Available Commands (%s):\n", version)

		for _, c := range commands {
			fmt.Printf("  %-*s   %s\n", maxNameLen, c.Name(), c.Short)
		}
	})

	// Add jobCmd and other commands to the root command
	rootCmd.AddCommand(initCmd)

	// Add jobCmd to the root command
	rootCmd.AddCommand(jobListCmd)
	rootCmd.AddCommand(jobStopCmd)
	rootCmd.AddCommand(jobLogsCmd)

	// Add chatCmd to the root command
	rootCmd.AddCommand(chatCmd)

	rootCmd.AddCommand(versionCmd)

	// Add deployManuscript to root command
	deployManuscript.Flags().StringVar(&env, "env", "", "Specify the environment to deploy (e.g., local or chainbase)")
	deployManuscript.MarkFlagRequired("env")
	rootCmd.AddCommand(deployManuscript)
	rootCmd.CompletionOptions.HiddenDefaultCmd = true
}

var rootCmd = &cobra.Command{
	Use: "manuscript-cli [command]",
}
