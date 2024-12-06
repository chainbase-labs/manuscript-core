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
	Long: `Initialize a new Manuscript project and start the required containers.

This command will:
1. Create a new manuscript directory structure
2. Set up required configuration files
3. Initialize Docker containers
4. Configure database connections
5. Set up GraphQL endpoints

The initialization process will prompt for:
- Manuscript name
- Chain selection
- Table selection
- Output target (PostgreSQL or Print)`,
	Example: `  # Initialize a new manuscript project
  manuscript-cli init

  # Using alias
  manuscript-cli i`,
	Args: cobra.ExactArgs(0),
	Run: func(cmd *cobra.Command, args []string) {
		InitManuscript()
	},
}

var jobListCmd = &cobra.Command{
	Use:   "list",
	Short: "List all manuscript jobs",
	Long: `List all running manuscript jobs and their current status.

Displays the following information for each job:
- Manuscript name
- Job state (RUNNING/CANCELED/etc)
- Start time
- Duration
- GraphQL endpoint (if running)

Status indicators:
🟢 Running - Job is active and processing data
🟡 Warning - Job needs attention
⚪️ Other - Various other states`,
	Example: `  # List all jobs
  manuscript-cli list`,
	Run: func(cmd *cobra.Command, args []string) {
		ListJobs()
	},
}

var jobStopCmd = &cobra.Command{
	Use:   "stop <job_name>",
	Short: "Stop a manuscript job",
	Long: `Stop a running manuscript job and clean up its resources.

This command will:
1. Stop the specified job gracefully
2. Clean up associated Docker containers
3. Preserve job configuration and data
4. Release allocated ports

The job can be restarted later using the deploy command.`,
	Example: `  # Stop a specific job
  manuscript-cli stop <my-manuscript>

  # Stop multiple jobs
  manuscript-cli stop <job1> <job2>`,
	Args: cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		JobStop(args[0])
	},
}

var jobLogsCmd = &cobra.Command{
	Use:   "logs <job_name>",
	Short: "View logs of a manuscript job",
	Long: `Display the logs for a specified manuscript job.

Shows detailed operational logs including:
- Job startup information
- Processing statistics
- Error messages
- Performance metrics
- Connection status

The logs are retrieved from the job's Docker containers in real-time.`,
	Example: `  # View logs of a specific job
  manuscript-cli logs my-manuscript

  # Follow logs in real-time
  manuscript-cli logs my-manuscript -f`,
	Args: cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		JobLogs(args[0])
	},
}

var chatCmd = &cobra.Command{
	Use:     "chat <job_name>",
	Aliases: []string{"c"},
	Short:   "Chat with the dataset AI",
	Long: `Start an interactive AI chat session for your dataset.

Features:
- Natural language queries to SQL conversion
- Multiple AI provider support (OpenAI, Gemini, Gaia)
- Interactive query refinement
- Real-time query execution
- Formatted results display

Supported AI Providers:
1. OpenAI (ChatGPT)
2. Google Gemini
3. Gaia

Environment Variables Required:
- OPENAI_API_KEY for OpenAI
- GEMINI_API_KEY for Google Gemini
- OPENAI_API_BASE (optional) for custom endpoints`,
	Example: `  # Start chat with default AI provider
  manuscript-cli chat my-manuscript

  # Using alias
  manuscript-cli c my-manuscript`,
	Args: cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		Chat(args[0])
	},
}

var deployManuscript = &cobra.Command{
	Use:     "deploy <manuscript-file>",
	Aliases: []string{"d"},
	Short:   "Deploy Manuscript to a local environment or the Chainbase network.",
	Long: `Deploy a Manuscript configuration to either a local environment or the Chainbase network.

Deployment Process:
1. Validate manuscript configuration
2. Check resource availability
3. Set up required infrastructure
4. Deploy containers and services
5. Initialize connections
6. Verify deployment status

Environments:
- local: Deploy to local Docker environment
- chainbase: Deploy to Chainbase network (coming soon)

Configuration Requirements:
- Valid manuscript.yaml file
- Required environment variables
- Docker daemon running (for local deployment)`,
	Example: `  # Deploy to local environment
  manuscript-cli deploy /path/to/manuscript.yaml --env=local

  # Using alias
  manuscript-cli d /path/to/manuscript.yaml --env=local`,
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
	Long: `Display the current version of manuscript-cli and related information.

Shows:
- CLI version number
- Build information
- Compatible API versions
- Supported features`,
	Example: `  # Display version information
  manuscript-cli version`,
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
