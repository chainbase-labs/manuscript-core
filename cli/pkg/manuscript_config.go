package pkg

import (
	"encoding/json"
	"fmt"
	"manuscript-core/client"
	"os"
	"runtime"
	"strings"

	"gopkg.in/ini.v1"
)

type Config struct {
	BaseDir     string
	SystemInfo  string
	Manuscripts []Manuscript
}

func LoadConfig(filePath string) (*Config, error) {
	var config Config
	resp, err := client.LoadConfig(filePath)
	if err != nil {
		return nil, fmt.Errorf("failed to load config: %w", err)
	}
	err = json.Unmarshal([]byte(resp), &config)
	if err != nil {
		return nil, fmt.Errorf("failed to decode response: %w", err)
	}

	return &config, nil
}

// SaveConfig saves the config to the specified file path, merging with existing settings
func SaveConfig(filePath string, newConfig *Config) error {
	if newConfig.BaseDir != "" {
		_, err := os.Stat(newConfig.BaseDir)
		if os.IsNotExist(err) {
			return err
		}
	}

	if strings.Contains(filePath, "$HOME") {
		homeDir, err := os.UserHomeDir()
		if err != nil {
			return err
		}
		filePath = strings.Replace(filePath, "$HOME", homeDir, 1)
	}

	// Get system info
	newConfig.SystemInfo = runtime.GOOS

	_, err := os.Stat(filePath)
	if os.IsNotExist(err) {
		_, err := os.Create(filePath)
		if err != nil {
			return err
		}
	}

	cfg, err := ini.Load(filePath)
	if err != nil {
		cfg = ini.Empty()
	}

	if newConfig.BaseDir != "" {
		cfg.Section("").Key("baseDir").SetValue(newConfig.BaseDir)
		cfg.Section("").Key("systemInfo").SetValue(newConfig.SystemInfo)
	}

	for _, manuscript := range newConfig.Manuscripts {
		if manuscript.DbUser == "" {
			manuscript.DbUser = "postgres"
			manuscript.DbPassword = "postgres"
		}

		section := cfg.Section(manuscript.Name)
		section.Key("baseDir").SetValue(manuscript.BaseDir)
		section.Key("name").SetValue(manuscript.Name)
		section.Key("specVersion").SetValue(manuscript.SpecVersion)
		section.Key("parallelism").SetValue(strings.TrimSpace(fmt.Sprintf("%d", manuscript.Parallelism)))
		section.Key("chain").SetValue(manuscript.Chain)
		section.Key("table").SetValue(manuscript.Table)
		section.Key("database").SetValue(manuscript.Database)
		section.Key("query").SetValue(manuscript.Query)
		section.Key("sink").SetValue(manuscript.Sink)
		section.Key("port").SetValue(strings.TrimSpace(fmt.Sprintf("%d", manuscript.Port)))
		section.Key("dbPort").SetValue(strings.TrimSpace(fmt.Sprintf("%d", manuscript.DbPort)))
		section.Key("dbUser").SetValue(strings.TrimSpace(fmt.Sprintf("%s", manuscript.DbUser)))
		section.Key("dbPassword").SetValue(strings.TrimSpace(fmt.Sprintf("%s", manuscript.DbPassword)))
		section.Key("graphqlPort").SetValue(strings.TrimSpace(fmt.Sprintf("%d", manuscript.GraphQLPort)))
		section.Key("schema").SetValue(strings.TrimSpace(fmt.Sprintf("%s", manuscript.Schema)))
	}

	err = cfg.SaveTo(filePath)
	if err != nil {
		return err
	}

	return nil
}

// SaveConfigFresh creates a completely new config file without merging existing settings
func SaveConfigFresh(filePath string, newConfig *Config) error {
	if strings.Contains(filePath, "$HOME") {
		homeDir, err := os.UserHomeDir()
		if err != nil {
			return err
		}
		filePath = strings.Replace(filePath, "$HOME", homeDir, 1)
	}

	// Create new empty INI file
	cfg := ini.Empty()

	// Set base configuration
	if newConfig.BaseDir != "" {
		cfg.Section("").Key("baseDir").SetValue(newConfig.BaseDir)
	}
	if newConfig.SystemInfo != "" {
		cfg.Section("").Key("systemInfo").SetValue(newConfig.SystemInfo)
	}

	// Save manuscripts directly
	for _, manuscript := range newConfig.Manuscripts {
		section := cfg.Section(manuscript.Name)
		section.Key("baseDir").SetValue(manuscript.BaseDir)
		section.Key("name").SetValue(manuscript.Name)
		section.Key("specVersion").SetValue(manuscript.SpecVersion)
		section.Key("parallelism").SetValue(fmt.Sprintf("%d", manuscript.Parallelism))
		section.Key("chain").SetValue(manuscript.Chain)
		section.Key("table").SetValue(manuscript.Table)
		section.Key("database").SetValue(manuscript.Database)
		section.Key("query").SetValue(manuscript.Query)
		section.Key("sink").SetValue(manuscript.Sink)
		section.Key("port").SetValue(fmt.Sprintf("%d", manuscript.Port))
		section.Key("dbPort").SetValue(fmt.Sprintf("%d", manuscript.DbPort))
		section.Key("dbUser").SetValue(manuscript.DbUser)
		section.Key("dbPassword").SetValue(manuscript.DbPassword)
		section.Key("graphqlPort").SetValue(fmt.Sprintf("%d", manuscript.GraphQLPort))
	}

	// Write directly to file
	return cfg.SaveTo(filePath)
}
