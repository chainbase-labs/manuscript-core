package config

import (
	"os"
	"testing"
)

func TestLoadConfig(t *testing.T) {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		t.Fatalf("failed to get user home directory: %v", err)
	}

	config, err := LoadConfig(homeDir + "/.manuscript_config.ini")
	if err != nil {
		t.Fatalf("Load config failed: %v", err)
	}

	if config == nil {
		t.Errorf("config is null")
	}
}
