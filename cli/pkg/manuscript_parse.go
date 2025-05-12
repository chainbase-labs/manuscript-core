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
	BaseDir      string      `json:"BaseDir" yaml:"-"`
	Name         string      `json:"Name" yaml:"name"`
	SpecVersion  string      `json:"SpecVersion" yaml:"specVersion"`
	Parallelism  int         `json:"Parallelism" yaml:"parallelism"`
	Sources      []Source    `json:"Sources" yaml:"sources,omitempty"`
	Transforms   []Transform `json:"Transforms" yaml:"transforms,omitempty"`
	Sinks        []Sink      `json:"Sinks" yaml:"sinks,omitempty"`
	Chain        string      `json:"Chain" yaml:"-"`
	Table        string      `json:"Table" yaml:"-"`
	Database     string      `json:"Database" yaml:"-"`
	Query        string      `json:"Query" yaml:"-"`
	Sink         string      `json:"Sink" yaml:"-"`
	Port         int         `json:"Port" yaml:"-"`
	DbPort       int         `json:"DbPort" yaml:"-"`
	DbUser       string      `json:"DbUser" yaml:"-"`
	DbPassword   string      `json:"DbPassword" yaml:"-"`
	GraphQLImage string      `json:"GraphQLImage" yaml:"-"`
	GraphQLPort  int         `json:"GraphQLPort" yaml:"-"`
	Schema       string      `json:"Schema" yaml:"-"`
	CkDir        string      `json:"ckDir" yaml:"-"`
	SpDir        string      `json:"spDir" yaml:"-"`
	LogDir       string      `json:"logDir" yaml:"-"`
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

func ParseDeployManuscript(filename string) ([]byte, error) {
	file, err := os.ReadFile(filename)
	if err != nil {
		return nil, err
	}

	var rootNode yaml.Node
	if err := yaml.Unmarshal(file, &rootNode); err != nil {
		return nil, err
	}

	if err := retainOnlyPostgresSink(&rootNode); err != nil {
		return nil, err
	}

	newYamlBytes, err := yaml.Marshal(&rootNode)
	if err != nil {
		return nil, err
	}
	return newYamlBytes, nil
}

func retainOnlyPostgresSink(node *yaml.Node) error {
	if node.Kind == yaml.MappingNode {
		for i := 0; i < len(node.Content); i += 2 {
			keyNode := node.Content[i]
			valueNode := node.Content[i+1]

			// get sinks
			if keyNode.Value == "sinks" && valueNode.Kind == yaml.SequenceNode {
				var newSinks []*yaml.Node
				for _, sinkNode := range valueNode.Content {
					if sinkNode.Kind == yaml.MappingNode {
						// check type = pg?
						for j := 0; j < len(sinkNode.Content); j += 2 {
							if sinkNode.Content[j].Value == "type" &&
								sinkNode.Content[j+1].Value == "postgres" {
								// modify config
								for k := 0; k < len(sinkNode.Content); k += 2 {
									if sinkNode.Content[k].Value == "config" {
										configNode := sinkNode.Content[k+1]
										if configNode.Kind == yaml.MappingNode {
											// update config
											newConfig := &yaml.Node{
												Kind: yaml.MappingNode,
												Content: []*yaml.Node{
													{Kind: yaml.ScalarNode, Value: "host"},
													{Kind: yaml.ScalarNode, Value: "<<<DB_HOST>>>"},
													{Kind: yaml.ScalarNode, Value: "port"},
													{Kind: yaml.ScalarNode, Value: "<<<DB_PORT>>>"},
													{Kind: yaml.ScalarNode, Value: "username"},
													{Kind: yaml.ScalarNode, Value: "<<<DB_USER>>>"},
													{Kind: yaml.ScalarNode, Value: "password"},
													{Kind: yaml.ScalarNode, Value: "<<<DB_PASS>>>"},
												},
											}
											sinkNode.Content[k+1] = newConfig
										}
									}
								}
								newSinks = append(newSinks, sinkNode)
								break
							}
						}
					}
				}
				// edit sinks
				valueNode.Content = newSinks
			}
		}
	}

	for _, child := range node.Content {
		if err := retainOnlyPostgresSink(child); err != nil {
			return err
		}
	}

	return nil
}
