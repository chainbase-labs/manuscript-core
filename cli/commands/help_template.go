package commands

import "github.com/spf13/cobra"

const helpTemplate = `{{ " ██████" | yellow }} ██    ██   ████   ████████ ███    ██ ███████    ████    ██████  ████████
{{ "██" | green }}      ██    ██  ██  ██     ██    ████   ██ ██    ██  ██  ██  ██    ██ ██
{{ "██" | green }}      ██    ██ ██    ██    ██    ██ ██  ██ ██    ██ ██    ██ ██       ██
{{ "██" | green }}      ████████ ████████    ██    ██  ██ ██ ███████  ████████  ██████  ██████
{{ "██" | green }}      ██    ██ ██    ██    ██    ██   ████ ██    ██ ██    ██       ██ ██
{{ "██" | green }}      ██    ██ ██    ██    ██    ██    ███ ██    ██ ██    ██ ██    ██ ██
{{ " ██████" | yellow }} ██    ██ ██    ██ ████████ ██     ██ ███████  ██    ██  ██████  ████████

Chainbase Manuscript™ Build The World's Largest Omnichain Data Network 🚀 🚀 🚀
─────────────────────────────────────────────────────────────────────────────────

{{ ">> " | green }}{{.UseLine}}
{{if or .Long .Short}}
{{if .Long}}{{.Long | trim}}{{else}}{{.Short | trim}}{{end}}{{end}}
{{if .HasAvailableSubCommands}}⌨️  Available Commands:{{range .Commands}}{{if (or .IsAvailableCommand (eq .Name "help"))}}
  {{rpad .Name .NamePadding }} {{.Short}}{{end}}{{end}}{{end}}

{{if .Flags.HasAvailableFlags}}🚩  Flags:
{{.LocalFlags.FlagUsages | trimTrailingWhitespaces}}{{end}}{{if .HasExample}}

🧠  Examples:
{{.Example}}{{end}}
`

func configureHelp(cmd *cobra.Command) {
	cobra.AddTemplateFunc("yellow", yellow)
	cobra.AddTemplateFunc("green", green)
	cmd.SetHelpTemplate(helpTemplate)
}

func yellow(s string) string {
	return "\033[33m" + s + "\033[0m"
}

func green(s string) string {
	return "\033[32m" + s + "\033[0m"
}
