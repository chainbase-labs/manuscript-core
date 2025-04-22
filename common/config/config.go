package config

import (
	"gopkg.in/ini.v1"
	"os"
	"strings"
)

type Config struct {
	BaseDir     string       `json:"baseDir"`
	Manuscripts []Manuscript `json:"manuscripts"`
}

type Manuscript struct {
	BaseDir     string `json:"baseDir"`
	Name        string `json:"name"`
	SpecVersion string `json:"specVersion"`
	Parallelism int    `json:"parallelism"`
	Chain       string `json:"chain"`
	Table       string `json:"table"`
	Database    string `json:"database"`
	Query       string `json:"query"`
	Sink        string `json:"sink"`
	Port        int    `json:"port"`
	DbPort      int    `json:"dbPort"`
	DbUser      string `json:"dbUser"`
	DbPassword  string `json:"dbPassword"`
	GraphQLPort int    `json:"graphqlPort"`
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

	if len(cfg.Sections()) == 1 && cfg.Section("") != nil {
		err := os.Remove(filePath)
		if err != nil {
			return nil, err
		}
		_, err = os.Create(filePath)
		if err != nil {
			return nil, err
		}
		cfg, err = ini.Load(filePath)
		if err != nil {
			return nil, err
		}
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
