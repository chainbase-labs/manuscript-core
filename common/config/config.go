package config

import (
	"gopkg.in/ini.v1"
	"os"
	"strings"
)

const (
	ManuscriptBaseName    = "manuscripts"
	ManuscriptBaseDir     = "$HOME"
	ManuscriptConfig      = "$HOME/.manuscript_config.ini"
	ChainbaseAPIURL       = "https://api.chainbase.com"
	MsStudioURL           = "https://net-static-dev.chainbasehq.com"
	PlatformChainURL      = "https://console.chainbase.com"
	NetworkChainEndpoint  = "/api/v1/metadata/network_chains"
	PlatformChainEndpoint = "/api/v2/datacloud/metadata"
	MsDeployEndpoint      = "/studio/v1/manuscripts"
	DefaultDatabase       = "zkevm"
	DefaultTable          = "blocks"
	DefaultSink           = "postgres"
	GraphQLImage          = "repository.chainbase.com/manuscript-node/graphql-engine:latest"
	GraphQLARMImage       = "repository.chainbase.com/manuscript-node/graphql-engine-arm64:latest"
	DefaultPrimaryKey     = "block_number"
	IcpPrimaryKey         = "block_idx"
)

type Config struct {
	BaseDir     string       `json:"baseDir"`
	Manuscripts []Manuscript `json:"manuscripts"`
}

type Manuscript struct {
	BaseDir      string `yaml:"baseDir"`
	Name         string `yaml:"name"`
	SpecVersion  string `yaml:"specVersion"`
	Parallelism  int    `yaml:"parallelism"`
	Chain        string `yaml:"chain"`
	Table        string `yaml:"table"`
	Database     string `yaml:"database"`
	Query        string `yaml:"query"`
	Sink         string `yaml:"sink"`
	Port         int    `yaml:"port"`
	DbPort       int    `yaml:"dbPort"`
	DbUser       string `yaml:"dbUser"`
	DbPassword   string `yaml:"dbPassword"`
	GraphQLImage string `yaml:"graphqlImage"`
	GraphQLPort  int    `yaml:"graphqlPort"`
	Schema       string `yaml:"schema"`
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
			Schema:      section.Key("schema").String(),
		}

		config.Manuscripts = append(config.Manuscripts, manuscript)
	}

	return config, nil
}
