package commands

import (
	"fmt"
	"manuscript-core/pkg"
	"os"
	"strings"

	"github.com/spf13/cobra"
)

var (
	env     string
	version = "1.1.1"
	//Output types for manuscript
	validOutputs = map[string]bool{
		"postgresql": true,
		"console":    true,
	}
	// Generate a string of valid output types for error messages
	validOutputList = func() string {
		options := make([]string, 0, len(validOutputs))
		for opt := range validOutputs {
			options = append(options, opt)
		}
		return strings.Join(options, ", ")
	}()
)

// Execute runs the CLI commands
func Execute(args []string) error {
	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	return nil
}

// *** MAIN COMMANDS *** //
// `init` command to initialize a new manuscript project
var initCmd = &cobra.Command{
	Use:     "init [manuscript-name]",
	Aliases: []string{"ini", "in", "i"},
	Short:   "Initialize and start local manuscript containers",
	Long: `📦 Initialize a new Manuscript project

	🎮 Two modes available:

	1. 📱 Interactive Mode:
				manuscript-cli init
				Let the CLI guide you through setup with prompts

	2. ⚡ Non-interactive Mode:
				manuscript-cli init <manuscript-name> \
						--output <output-type> \
						--protocol <database-name> \
						--dataset <table-name>
				Specify all parameters directly on command line

Setup Steps:
✨ Create project structure & configs
🐳 Initialize Docker containers
🔌 Configure database connections
🚀 Set up GraphQL endpoints

If interactive, you'll be prompted to select:
• Project name     ✅
• Chain type       ✅
• Tables           ✅
• Output format    ✅`,
	Example: `>> manuscript-cli init
>> manuscript-cli init my-manuscript --output postgresql --protocol solana --dataset blocks
>> manuscript-cli init ./solana_maxi --output console --protocol solana --dataset blocks`,
	// Argument Validation
	Args: cobra.MaximumNArgs(1),
	PreRunE: func(cmd *cobra.Command, args []string) error {
		output, _ := cmd.Flags().GetString("output")
		protocol, _ := cmd.Flags().GetString("protocol")
		dataset, _ := cmd.Flags().GetString("dataset")

		// If flags are provided, manuscript name must be provided
		if len(args) == 0 && (output != "" || protocol != "" || dataset != "") {
			return fmt.Errorf("manuscript name is required when using flags")
		}

		// If manuscript name is provided, all flags must be provided
		if len(args) > 0 && (output == "" || protocol == "" || dataset == "") {
			return fmt.Errorf("output, protocol, and dataset flags are required when providing manuscript name")
		}

		// If one flag is provided, all must be provided
		flagsProvided := output != "" || protocol != "" || dataset != ""
		if flagsProvided {
			if output == "" {
				return fmt.Errorf("--output flag is required when using non-interactive mode")
			}
			if protocol == "" {
				return fmt.Errorf("--protocol flag is required when using non-interactive mode")
			}
			if dataset == "" {
				return fmt.Errorf("--dataset flag is required when using non-interactive mode")
			}

			// Validate output type
			if !validOutputs[output] {
				return fmt.Errorf("invalid output type '%s': must be one of: %s",
					output, validOutputList)
			}
		}

		return nil
	},
	Run: func(cmd *cobra.Command, args []string) {
		output, _ := cmd.Flags().GetString("output")
		protocol, _ := cmd.Flags().GetString("protocol")
		dataset, _ := cmd.Flags().GetString("dataset")

		if len(args) > 0 && output != "" && protocol != "" && dataset != "" {
			// Non-interactive mode
			InitManuscriptNonInteractive(args[0], output, protocol, dataset)
		} else {
			// Interactive mode
			InitManuscriptInteractive()
		}
	},
}

// `config` command to manage manuscript configuration
var configCmd = &cobra.Command{
	Use:   "config",
	Short: "Manage manuscript configuration",
	Long: `🔧 Manage Manuscript Configuration

Actions:
📁 View config file location
📋 Show current configuration
🧹 Clean configuration file

Usage:
- Run without arguments to see config location
- Use 'show' to view full configuration
- Use 'clean' to remove all configurations`,
	Example: `>> manuscript-cli config
>> manuscript-cli config show
>> manuscript-cli config clean`,
	Run: func(cmd *cobra.Command, args []string) {
		ConfigLocation()
	},
}

// SUB-COMMANDS for `config` command //
// `config show` command to display manuscript configuration
var configShowCmd = &cobra.Command{
	Use:   "show",
	Short: "Show manuscript configuration",
	Long: `📋 Show Manuscript Configuration

Options:
--summary  Show condensed overview of configuration

Display includes:
• Config file location
• Base directory
• Manuscript counts
• Port assignments
• Disk vs config differences`,
	Example: `>> manuscript-cli config show
>> manuscript-cli config show --summary`,
	Run: func(cmd *cobra.Command, args []string) {
		summary, _ := cmd.Flags().GetBool("summary")
		if summary {
			ConfigShowSummary()
		} else {
			ConfigShow()
		}
	},
}

// `config clean` command to remove manuscript configurations from config file
var configCleanCmd = &cobra.Command{
	Use:   "clean [manuscript-names...]",
	Short: "Clean manuscript configuration file",
	Long: `🧹 Clean Manuscript Configuration

Options:
--all    Remove all manuscripts
--force  Skip confirmation for removal

Examples:
• Clean specific manuscripts with confirmation:
  manuscript-cli config clean manuscript1 manuscript2

• Clean specific manuscripts without confirmation:
  manuscript-cli config clean manuscript1 manuscript2 --force

• Clean all manuscripts with confirmation:
  manuscript-cli config clean --all

• Clean all manuscripts without confirmation:
  manuscript-cli config clean --all --force`,
	Run: func(cmd *cobra.Command, args []string) {
		all, _ := cmd.Flags().GetBool("all")
		force, _ := cmd.Flags().GetBool("force")
		ConfigClean(all, force, args)
	},
}

// *** JOB COMMANDS *** //
// `list` command to show all manuscript jobs
var jobListCmd = &cobra.Command{
	Use:     "list",
	Aliases: []string{"ls"},
	Short:   "List all manuscript jobs",
	Long: `📋 View all running manuscript jobs

Each job shows:
🔷 Manuscript name
🔷 Status
🔷 Start time & duration
🔷 GraphQL endpoint

Status indicators:
🟢 Running - Job is active and processing data
🟡 Warning - Job needs attention
🔴 Failed - Job encountered an error
⚫ Stopped - Job was stopped

Usage:
- Run without arguments to check default directory
- Specify a directory path to check manuscripts in that location`,
	Example: `>> manuscript-cli ls
>> manuscript-cli list /path/to/manuscripts`,
	Args: cobra.MaximumNArgs(0),
	Run: func(cmd *cobra.Command, args []string) {
		config, err := pkg.LoadConfig(manuscriptConfig)
		if err != nil {
			fmt.Printf("Error: Failed to load manuscript config: %v", err)
		}
		ListJobs(config)
	},
}

// `stop` command to stop a manuscript job
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

// `logs` command to view logs of a manuscript job
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

// `chat` command to chat with the dataset AI for Text-2-SQL queries
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

// `deploy` command to deploy a manuscript.yaml file
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

// *** UTILITY COMMANDS *** //
// `version` command to show the version of manuscript-cli
var versionCmd = &cobra.Command{
	Use:   "version",
	Short: "Show the version of manuscript-cli",
	Long: `ℹ️  Manuscript CLI Version Information

Shows:
🛠️ Manuscript Core version
🏗️ Build info
💻 System information
🐳 Docker environment details`,
	Example: `>> manuscript-cli version`,
	Run: func(cmd *cobra.Command, args []string) {
		verbosity, _ := cmd.Flags().GetBool("verbose")
		showVersion(verbosity)
	},
}

func init() {
	// Set help from help_template.go for root command
	configureHelp(rootCmd)

	// Ensure commands show up as added instead of alphabetically
	cobra.EnableCommandSorting = false

	// Add commands to root
	addCLICommands()

	// Add flags to each command and subcommand
	configureFlags()

	// Disable the default completion command
	rootCmd.CompletionOptions.HiddenDefaultCmd = true
}

// Add CLI commands in logical groups
func addCLICommands() {
	// Manuscript creation & deployment commands
	rootCmd.AddCommand(initCmd)
	rootCmd.AddCommand(deployManuscript)

	// Add config commands
	configCmd.AddCommand(configShowCmd)
	configCmd.AddCommand(configCleanCmd)
	rootCmd.AddCommand(configCmd)

	// Job management commands
	rootCmd.AddCommand(jobListCmd)
	rootCmd.AddCommand(jobStopCmd)
	rootCmd.AddCommand(jobLogsCmd)

	// Interactive commands
	rootCmd.AddCommand(chatCmd)

	// Utility commands
	rootCmd.AddCommand(versionCmd)

}

// Configure flags for each command
func configureFlags() {
	// Configure init command flags
	initCmd.Flags().String("output", "", "Output type (postgresql or console)")
	initCmd.Flags().String("protocol", "", "Protocol/database name")
	initCmd.Flags().String("dataset", "", "Dataset/table name")

	// Add flags to config subcommands
	configCleanCmd.Flags().Bool("all", false, "Remove all manuscripts")
	configCleanCmd.Flags().Bool("force", false, "Skip confirmation for removal")
	configShowCmd.Flags().BoolP("summary", "s", false, "Show condensed overview of configuration")

	// Configure deployment flags
	deployManuscript.Flags().StringVar(&env, "env", "", "Specify the environment to deploy (local or chainbase)")
	deployManuscript.MarkFlagRequired("env")

	// Configure version command flags
	versionCmd.Flags().BoolP("verbose", "v", false, "Display detailed version information")
}

// Default Command for CLI
var rootCmd = &cobra.Command{
	Use: "manuscript-cli [command]",
}
