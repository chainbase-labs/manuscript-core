package pkg

import (
	"gopkg.in/yaml.v3"
	"os"
)

type Source struct {
	Name    string `yaml:"name"`
	Type    string `yaml:"type"`
	Dataset string `yaml:"dataset"`
	Filter  string `yaml:"filter"`
}

type Transform struct {
	Name string `yaml:"name"`
	SQL  string `yaml:"sql"`
}

type Sink struct {
	Name       string            `yaml:"name"`
	Type       string            `yaml:"type"`
	From       string            `yaml:"from"`
	Database   string            `yaml:"database"`
	Schema     string            `yaml:"schema"`
	Table      string            `yaml:"table"`
	PrimaryKey string            `yaml:"primary_key"`
	Config     map[string]string `yaml:"config"`
}

type Manuscript struct {
	BaseDir      string      `yaml:"baseDir"`
	Name         string      `yaml:"name"`
	SpecVersion  string      `yaml:"specVersion"`
	Parallelism  int         `yaml:"parallelism"`
	Sources      []Source    `yaml:"sources"`
	Transforms   []Transform `yaml:"transforms"`
	Sinks        []Sink      `yaml:"sinks"`
	Chain        string      `yaml:"chain"`
	Table        string      `yaml:"table"`
	Database     string      `yaml:"database"`
	Query        string      `yaml:"query"`
	Sink         string      `yaml:"sink"`
	Port         int         `yaml:"port"`
	DbPort       int         `yaml:"dbPort"`
	DbUser       string      `yaml:"dbUser"`
	DbPassword   string      `yaml:"dbPassword"`
	GraphQLImage string      `yaml:"graphqlImage"`
	GraphQLPort  int         `yaml:"graphqlPort"`
}

func ParseYAML(filename string) (*Manuscript, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var manuscript Manuscript

	decoder := yaml.NewDecoder(file)
	if err := decoder.Decode(&manuscript); err != nil {
		return nil, err
	}

	return &manuscript, nil
}
