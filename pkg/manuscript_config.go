package pkg

import (
	"gopkg.in/ini.v1"
	"os"
	"strings"
)

type Config struct {
	BaseDir string
	Items   map[string]map[string]string
}

func LoadConfig(filePath string) (*Config, error) {
	if strings.Contains(filePath, "$HOME") {
		homeDir, err := os.UserHomeDir()
		if err != nil {
			return nil, err
		}
		filePath = strings.Replace(filePath, "$HOME", homeDir, 1)
	}

	cfg, err := ini.Load(filePath)
	if err != nil {
		return nil, err
	}

	config := &Config{
		BaseDir: cfg.Section("").Key("baseDir").String(),
		Items:   make(map[string]map[string]string),
	}

	for _, section := range cfg.Sections() {
		if section.Name() == "DEFAULT" {
			continue
		}
		config.Items[section.Name()] = section.KeysHash()
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

	cfg, err := ini.Load(filePath)
	if err != nil {
		cfg = ini.Empty()
	}

	if newConfig.BaseDir != "" {
		cfg.Section("").Key("baseDir").SetValue(newConfig.BaseDir)
	}

	for sectionName, items := range newConfig.Items {
		section := cfg.Section(sectionName)
		for key, value := range items {
			section.Key(key).SetValue(value)
		}
	}

	err = cfg.SaveTo(filePath)
	if err != nil {
		return err
	}

	return nil
}
