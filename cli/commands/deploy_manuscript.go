package commands

import (
	"fmt"
	"io"
	"log"
	"manuscript-core/client"
	"manuscript-core/pkg"
	"os"
	"path/filepath"
	"strings"
)

func DeployManuscript(args []string) {
	if len(args) < 1 {
		log.Fatalf("Error: Manuscript path is required as the first argument.")
	}
	manuscriptPath := args[0]
	var ms pkg.Manuscript
	var manuscriptDir string

	if _, err := os.Stat(manuscriptPath); os.IsNotExist(err) {
		log.Fatalf("Error: Manuscript file does not exist: %s", manuscriptPath)
	}

	steps := []struct {
		name string
		fn   func() error
	}{
		{"Step 1: Parsing manuscript yaml", func() error {
			var err error
			ms, err = ParseManuscriptYaml(manuscriptPath)
			if err != nil {
				return err
			}
			if len(ms.Sinks) != 0 {
				ms.Table = ms.Sinks[0].Table
				ms.Database = ms.Sinks[0].Database
			}
			if len(ms.Sources) != 0 {
				ms.Chain = ms.Sources[0].Dataset
			}
			if len(ms.Transforms) != 0 {
				ms.Query = ms.Transforms[0].SQL
			}

			manuscriptDir = getHomeDir()
			msConfig, err := pkg.LoadConfig(manuscriptConfig)
			if err != nil {
				logErrorAndReturn("Failed to load manuscript config", err)
			}
			if msConfig.BaseDir != "" {
				manuscriptDir = msConfig.BaseDir
			}

			if strings.HasSuffix(manuscriptDir, "/") {
				manuscriptDir = strings.TrimSuffix(manuscriptDir, "/")
			}
			if ms.BaseDir == "" {
				ms.BaseDir = fmt.Sprintf("%s/%s", manuscriptDir, manuscriptBaseName)
			}
			manuscriptDir = fmt.Sprintf("%s/%s/%s", manuscriptDir, manuscriptBaseName, ms.Name)

			if len(ms.Sinks) != 0 {
				if ms.Sinks[0].Type == "postgres" {
					ms.Sink = "postgres"
				}
			}
			return nil
		}},
		{"Step 2: Verifying Port Initialization", func() error { return pkg.InitializePorts(&ms) }},
		{"Step 3: Checking manuscript is already deployed", func() error {
			err := CheckManuscriptExist(ms)
			if err != nil {
				return err
			}
			return nil
		}},
		{"Step 4: Create Directory", func() error { return createDirectory(manuscriptDir) }},
		{"Step 5: Create ManuscriptFile", func() error { return copyManuscriptFile(manuscriptDir, manuscriptPath) }},
		{"Step 6: Create DockerComposeFile", func() error { return createDockerComposeFile(manuscriptDir, &ms) }},
		{"Step 7: Check Docker Installed", func() error { return checkDockerInstalled() }},
		{"Step 8: Start Docker Containers", func() error { return startDockerContainers(manuscriptDir) }},
		{"Step 9: Check Container Status", func() error { return checkContainerStatus(&ms) }},
	}

	for _, step := range steps {
		err := pkg.ExecuteStepWithLoading(step.name, true, step.fn)
		if err != nil {
			log.Fatalf("\033[31m✗ %s failed: %v\n", step.name, err)
		}
	}
	fmt.Println("\033[32m✓ Manuscript deployment completed successfully!")
	log.Printf("\033[32mYou can now list your job with the command: \n👉 \033[33mmanuscript-cli list\n\n"+
		"\033[32mIf you need to manually edit the manuscript, "+
		"you can edit the file '%s/manuscript.yaml' and then manually execute the 'run' command:\n"+
		"👉 \u001B[33mvim %s/manuscript.yaml\n"+
		"👉 \033[33mmanuscript-cli deploy %s/manuscript.yaml --env=local\n\n", manuscriptDir, manuscriptDir, manuscriptDir)
	log.Printf("\033[32mYou can now access your manuscript at http://localhost:%d\n", ms.Port)

	err := pkg.SaveConfig(manuscriptConfig, &pkg.Config{Manuscripts: []pkg.Manuscript{ms}})
	if err != nil {
		fmt.Printf("Failed to save manuscript config: %v", err)
		return
	}
}

func DeployManuscriptOnChainbase(args []string, apiKey, hash, version string) {
	if len(args) < 1 {
		log.Fatalf("Error: Manuscript path is required as the first argument.")
	}
	manuscriptPath := args[0]
	var ms pkg.Manuscript

	if _, err := os.Stat(manuscriptPath); os.IsNotExist(err) {
		log.Fatalf("Error: Manuscript file does not exist: %s", manuscriptPath)
	}

	steps := []struct {
		name string
		fn   func() error
	}{
		{"Step 1: Parsing manuscript.yaml", func() error {
			var err error
			ms, err = ParseManuscriptYaml(manuscriptPath)
			if err != nil {
				return err
			}
			var pgSink *pkg.Sink
			for _, s := range ms.Sinks {
				if s.Type == "postgres" {
					pgSink = &pkg.Sink{
						Name:       s.Name,
						Type:       s.Type,
						From:       s.From,
						Database:   s.Database,
						Schema:     s.Schema,
						Table:      s.Table,
						PrimaryKey: s.PrimaryKey,
						Config: map[string]string{
							"host":     "${DB_HOST}",
							"port":     "${DB_PORT}",
							"username": "${DB_USER}",
							"password": "${DB_PASS}",
						},
					}
					break
				}
			}

			if len(ms.Sources) != 0 {
				ms.Chain = ms.Sources[0].Dataset
			}
			if len(ms.Transforms) != 0 {
				ms.Query = ms.Transforms[0].SQL
			}
			if pgSink != nil {
				ms.Sink = "postgres"
				ms.Sinks = []pkg.Sink{*pgSink}
				ms.Table = ms.Sinks[0].Table
				ms.Database = ms.Sinks[0].Database
			} else {
				return fmt.Errorf("no postgres sink found")
			}

			return nil
		}},
		{"Step 2: Checking manuscript is already deployed locally", func() error {
			existMs, deployed := CheckManuscriptDeployedLocally(ms)
			if !deployed {
				return fmt.Errorf("Manuscript hasn't been deployed locally, have to deploy and running locally first\n You can use: manuscript-cli  deploy %s --env=local", manuscriptPath)
			}
			ms.GraphQLPort = existMs.GraphQLPort
			ms.BaseDir = existMs.BaseDir
			ms.Port = existMs.Port
			ms.Schema = existMs.Schema
			ms.GraphQLImage = existMs.GraphQLImage
			return nil
		}},
		{"Step 3: Deploy Manuscript to chainbase network", func() error {
			msYaml, err := pkg.ParseDeployManuscript(manuscriptPath)
			if err != nil {
				return err
			}
			if ms.Schema == "" {
				ms.Schema, err = client.GetTableSchema(ms.GraphQLPort, ms.Table)
				if err != nil {
					return err
				}
			}
			return client.Deploy(msYaml, ms.Schema, hash, apiKey, version)
		}},
	}

	for _, step := range steps {
		err := pkg.ExecuteStepWithLoading(step.name, true, step.fn)
		if err != nil {
			log.Fatalf("\033[31m✗ %s failed: %v\n", step.name, err)
		}
	}
	fmt.Println("\033[32m✓ Manuscript deployment to chainbase network completed successfully!")
	log.Printf("\033[32mYou can now view your manuscript at %s/manuscript-studio?id=%s\n", msStudioURL, hash)
}

func copyManuscriptFile(manuscriptDir, manuscriptPath string) error {
	_, fileName := filepath.Split(manuscriptPath)
	destinationPath := filepath.Join(manuscriptDir, fileName)

	sourceFile, err := os.Open(manuscriptPath)
	if err != nil {
		return fmt.Errorf("failed to open source file: %w", err)
	}
	defer sourceFile.Close()

	destFile, err := os.Create(destinationPath)
	if err != nil {
		return fmt.Errorf("failed to create destination file: %w", err)
	}
	defer destFile.Close()

	_, err = io.Copy(destFile, sourceFile)
	if err != nil {
		return fmt.Errorf("failed to copy file content: %w", err)
	}

	err = destFile.Sync()
	if err != nil {
		return fmt.Errorf("failed to sync destination file: %w", err)
	}

	return nil
}

func ParseManuscriptYaml(manuscriptPath string) (pkg.Manuscript, error) {
	ms, err := pkg.ParseYAML(manuscriptPath)
	if err != nil {
		log.Fatalf("Error: Failed to parse manuscript yaml: %v", err)
		return pkg.Manuscript{}, err
	}
	return *ms, nil
}

func CheckManuscriptExist(ms pkg.Manuscript) error {
	dockers, err := pkg.RunDockerPs()
	if err != nil {
		log.Fatalf("Error: Failed to get docker ps: %v", err)
	}
	if len(dockers) == 0 {
		return nil
	}
	for _, d := range dockers {
		if d.Name == fmt.Sprintf("%s-jobmanager-1", ms.Name) {
			return fmt.Errorf("error: Manuscript [ %s ] already deployed, please change the name in the manuscript yaml file", ms.Name)
		}
	}
	return nil
}

func CheckManuscriptDeployedLocally(ms pkg.Manuscript) (*pkg.Manuscript, bool) {
	existMs, exist := checkManuscriptConfigExist(ms)
	if exist && checkManuscriptJobRunning(*existMs) {
		return existMs, true
	}
	return nil, false
}

func checkManuscriptConfigExist(manuscript pkg.Manuscript) (*pkg.Manuscript, bool) {
	msConfig, err := pkg.LoadConfig(manuscriptConfig)
	if err != nil {
		logErrorAndReturn("Failed to load manuscript config", err)
		return nil, false
	}

	for _, ms := range msConfig.Manuscripts {
		if manuscript.Name == ms.Name && manuscript.Table == ms.Table && manuscript.Database == ms.Database {
			return &ms, true
		}
	}
	return nil, false
}

func checkManuscriptJobRunning(ms pkg.Manuscript) bool {
	state, err := pkg.GetJobStatus(ms)
	if err != nil {
		logErrorAndReturn("Failed to load manuscript config", err)
		return false
	}
	if *state == pkg.StateRunning {
		return true
	}

	return false
}
