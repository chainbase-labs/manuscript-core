package pkg

import (
	"fmt"
	"gopkg.in/ini.v1"
	"os"
	"runtime"
	"strings"
)

type Config struct {
	BaseDir     string
	SystemInfo  string
	Manuscripts []Manuscript
}

func LoadConfig(filePath string) (*Config, error) {
	if strings.Contains(filePath, "$HOME") {
		homeDir, err := os.UserHomeDir()
		if err != nil {
			return nil, err
		}
		filePath = strings.Replace(filePath, "$HOME", homeDir, 1)
	}

	_, err := os.Stat(filePath)
	if os.IsNotExist(err) {
		_, err := os.Create(filePath)
		if err != nil {
			return nil, err
		}
	}

	cfg, err := ini.Load(filePath)
	if err != nil {
		return nil, err
	}

	config := &Config{
		BaseDir:     cfg.Section("").Key("baseDir").String(),
		Manuscripts: []Manuscript{},
	}

	for _, section := range cfg.Sections() {
		if section.Name() == "DEFAULT" || section.Name() == "" {
			continue
		}

		manuscript := Manuscript{
			BaseDir:     section.Key("baseDir").String(),
			Name:        section.Key("name").String(),
			SpecVersion: section.Key("specVersion").String(),
			Parallelism: section.Key("parallelism").MustInt(1),
			Chain:       section.Key("chain").String(),
			Table:       section.Key("table").String(),
			Database:    section.Key("database").String(),
			Query:       section.Key("query").String(),
			Sink:        section.Key("sink").String(),
			Port:        section.Key("port").MustInt(8080),
			DbPort:      section.Key("dbPort").MustInt(15432),
			DbUser:      section.Key("dbUser").MustString("postgres"),
			DbPassword:  section.Key("dbPassword").MustString("postgres"),
			GraphQLPort: section.Key("graphqlPort").MustInt(8081),
		}

		config.Manuscripts = append(config.Manuscripts, manuscript)
	}

	return config, nil
}

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
	}

	err = cfg.SaveTo(filePath)
	if err != nil {
		return err
	}

	return nil
}
