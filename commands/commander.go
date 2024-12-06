package commands

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

var (
	env     string
	version = "1.0.4"
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
	Long: `📦 Initialize a new Manuscript project

Setup Steps:
✨ Create project structure & configs
🐳 Initialize Docker containers
🔌 Configure database connections
🚀 Set up GraphQL endpoints

You'll be prompted to select:
• Project name     ✅
• Chain type       ✅
• Tables           ✅
• Output format    ✅`,
	Example: `>> manuscript-cli i
>> manuscript-cli ini
>> manuscript-cli init`,
	Args: cobra.ExactArgs(0),
	Run: func(cmd *cobra.Command, args []string) {
		InitManuscript()
	},
}

var jobListCmd = &cobra.Command{
	Use:   "list",
	Short: "List all manuscript jobs",
	Long: `📋 View all running manuscript jobs

Each job shows:
🔷 Manuscript name
🔷 Status
🔷 Start time & duration
🔷 GraphQL endpoint

Status indicators:
🟢 Running - Job is active and processing data
🟡 Warning - Job needs attention
⚪️ Other - Various other states`,
	Example: `>> manuscript-cli list`,
	Run: func(cmd *cobra.Command, args []string) {
		ListJobs()
	},
}

var jobStopCmd = &cobra.Command{
	Use:   "stop <job_name>",
	Short: "Stop a manuscript job",
	Long: `🛑 Stop a running manuscript job

Actions:
✨ Graceful job termination
🧹 Docker container cleanup
💾 Config & data preservation
🔓 Port release

Note: Restart jobs using 'deploy' command`,
	Example: `>> manuscript-cli stop <my-manuscript1> <my-manuscript2>`,
	Args:    cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		JobStop(args[0])
	},
}

var jobLogsCmd = &cobra.Command{
	Use:   "logs <job_name>",
	Short: "View logs of a manuscript job",
	Long: `📋 View manuscript job logs in real-time

Shows:
🔷 Startup events
🔷 Processing stats
🔷 Error tracking
🔷 Performance data
🔷 Connection info`,
	Example: `>> manuscript-cli logs <my-manuscript>`,
	Args:    cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		JobLogs(args[0])
	},
}
var chatCmd = &cobra.Command{
	Use:     "chat <job_name>",
	Aliases: []string{"c"},
	Short:   "Chat with the dataset AI",
	Long: `🤖 Interactive AI Chat for Your Dataset

Features:
🔷 Natural language to SQL queries
🔷 Multi-AI provider support
🔷 Interactive refinement
🔷 Real-time execution
🔷 Formatted results

AI Providers:
✨ OpenAI (ChatGPT)
✨ Google Gemini
✨ Gaia

Required Env Vars:
🔑 OPENAI_API_KEY (ChatGPT)
🔑 GEMINI_API_KEY (Google Gemini)
🔑 OPENAI_API_BASE (optional/custom)`,
	Example: `>> manuscript-cli chat my-manuscript
>> manuscript-cli c my-manuscript`,
	Args: cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		Chat(args[0])
	},
}

var deployManuscript = &cobra.Command{
	Use:     "deploy <manuscript-file>",
	Aliases: []string{"d"},
	Short:   "Deploy Manuscript locally or to Chainbase network",
	Long: `🚀 Deploy Your Manuscript Configuration

Deployment Steps:
🔍 Validate configuration
🔧 Check resources
🏗️ Setup infrastructure
🚀 Deploy containers
🔌 Initialize connections
☑️ Verify deployment

Environments:
🔷 local    - Deploy to Docker
🔷 chainbase - Deploy to Network (soon)

Requirements:
📋 manuscript.yaml
🔑 Environment variables
🐳 Running Docker (local only)`,
	Example: `>> manuscript-cli deploy config.yaml --env=local
>> manuscript-cli d config.yaml --env=local`,
	Args: cobra.ExactArgs(1),
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
	Long: `ℹ️  Manuscript CLI Version Information

Shows:
🔷 CLI version
🔷 Build info (coming soon)
🔷 API versions (coming soon)`,
	Example: `>> manuscript-cli version`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Printf("manuscript-cli version %s\n", version)
	},
}

func init() {
	// Configure help from help_template.go
	configureHelp(rootCmd)

	// Add CLI commands in logical groups
	// Manuscript creation & deployment commands
	rootCmd.AddCommand(initCmd)
	rootCmd.AddCommand(deployManuscript)

	// Job management commands
	rootCmd.AddCommand(jobListCmd)
	rootCmd.AddCommand(jobStopCmd)
	rootCmd.AddCommand(jobLogsCmd)

	// Interactive commands
	rootCmd.AddCommand(chatCmd)

	// Utility commands
	rootCmd.AddCommand(versionCmd)

	// Configure deployment flags
	deployManuscript.Flags().StringVar(&env, "env", "", "Specify the environment to deploy (local or chainbase)")
	deployManuscript.MarkFlagRequired("env")

	// Disable the default completion command
	rootCmd.CompletionOptions.HiddenDefaultCmd = true
}

var rootCmd = &cobra.Command{
	Use: "manuscript-cli [command]",
}
