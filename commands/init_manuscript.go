package commands

import (
	"bufio"
	"fmt"
	"log"
	"manuscript-core/client"
	"manuscript-core/pkg"
	"manuscript-core/static"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strconv"
	"strings"
	"text/template"
	"time"
)

const (
	manuscriptBaseName = "manuscript"
	manuscriptBaseDir  = "$HOME"
	manuscriptConfig   = "$HOME/.manuscript_config.ini"
	networkChainURL    = "https://api.chainbase.com"
	defaultDatabase    = "zkevm"
	defaultTable       = "blocks"
	defaultSink        = "postgres"
	graphQLImage       = "repository.chainbase.com/manuscript-node/graphql-engine:latest"
	graphQLARMImage    = "repository.chainbase.com/manuscript-node/graphql-engine-arm64:latest"
)

func executeInitManuscript(ms pkg.Manuscript) {
	manuscriptName := strings.ToLower(strings.ReplaceAll(ms.Name, " ", "_"))
	manuscriptDir := fmt.Sprintf("%s/%s", ms.BaseDir, manuscriptName)

	steps := []struct {
		name string
		fn   func() error
	}{
		{"Step 1: Create Directory", func() error { return createDirectory(manuscriptDir) }},
		{"Step 2: Create ManuscriptFile", func() error { return createManuscriptFile(manuscriptDir, ms) }},
		{"Step 3: Create DockerComposeFile", func() error { return createDockerComposeFile(manuscriptDir, &ms) }},
		{"Step 4: Check Docker Installed", func() error { return checkDockerInstalled() }},
		{"Step 5: Start Docker Containers", func() error { return startDockerContainers(manuscriptDir) }},
		{"Step 6: Check Container Status", func() error { return checkContainerStatus(&ms) }},
	}

	for i, step := range steps {
		err := pkg.ExecuteStepWithLoading(step.name, true, step.fn)
		if err != nil {
			log.Fatalf("\033[31m✗ %s failed: %v\n", fmt.Sprintf("Step %d", i+1), err)
		}
	}
	log.Printf("🎉 \033[32mManuscript %s deployment completed successfully!\033[0m\n", ms.Name)
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

func InitManuscript() {
	// Check if manuscript config exists
	manuscriptDir := getHomeDir()
	msConfig, err := pkg.LoadConfig(manuscriptConfig)
	if err != nil {
		logErrorAndReturn("Failed to load manuscript config", err)
	}
	if msConfig.BaseDir != "" {
		manuscriptDir = msConfig.BaseDir
	}

	// Prompt user for manuscript name, chain, table, and output target
	prompt := fmt.Sprintf("👋 1. Enter your manuscript base directory (default is %s)\u001B[0m: ", manuscriptDir)
	manuscriptDir = promptInput(prompt, manuscriptDir)
	if err := pkg.SaveConfig(manuscriptConfig, &pkg.Config{BaseDir: manuscriptDir}); err != nil {
		logErrorAndReturn("Failed to save manuscript config", err)
		return
	}
	if strings.HasSuffix(manuscriptDir, "/") {
		manuscriptDir = strings.TrimSuffix(manuscriptDir, "/")
	}
	manuscriptDir = fmt.Sprintf("%s/%s", manuscriptDir, manuscriptBaseName)
	fmt.Printf("\033[32m✓ Manuscript base directory set to: %s\033[0m\n\n", manuscriptDir)

	manuscriptName := promptInput("🏂 2. Enter your manuscript name (default is demo)\u001B[0m: ", "demo")
	if checkDockerContainerExists(manuscriptName) {
		logErrorAndReturn(fmt.Sprintf("Manuscript with name [ %s ] already exists. Please choose a different name.", manuscriptName), nil)
	}
	fmt.Printf("\u001B[32m✓ Manuscript name set to: %s\u001B[0m\n\n", manuscriptName)
	chains, err := fetchChainBaseDatasets()
	if err != nil {
		log.Fatalf("Error fetching datasets: %v\n", err)
	}

	selectedChain, selectedDatabase := selectChain(chains, "🏂 3. Please select a chainbase network dataset from the list below: ", defaultDatabase)
	selectedTable := selectTable(chains, selectedChain, "🧲 4. Please select a table from the list below: ", defaultTable)

	outputChoice := promptOutputTarget()
	fmt.Printf("\n\033[33m🏄🏄 Summary of your selections:\033[0m\n")
	fmt.Printf("Selected manuscript base directory: \033[32m%s\033[0m\n", manuscriptDir)
	fmt.Printf("Selected manuscript name: \033[32m%s\033[0m\n", manuscriptName)
	fmt.Printf("Selected chain: \033[32m%s\033[0m\n", selectedChain)
	fmt.Printf("Selected table: \u001B[32m%s\u001B[0m\n", selectedTable)
	fmt.Printf("Data output target: \u001B[32m%s\u001B[0m\n\n", outputChoice)

	// Confirm user selections
	if confirmProceed() {
		ms := pkg.Manuscript{
			BaseDir:  manuscriptDir,
			Name:     manuscriptName,
			Chain:    selectedChain,
			Table:    selectedTable,
			Database: selectedDatabase,
			Query:    fmt.Sprintf("Select * From %s_%s", selectedDatabase, selectedTable),
			Sink:     outputChoice,
		}
		fmt.Printf("\033[32m🚀 Deploying manuscript %s,%s...\033[0m\n", ms.Name, ms.BaseDir)
		executeInitManuscript(ms)
	}
}

func createDirectory(dir string) error {
	err := os.MkdirAll(dir, 0755)
	if err != nil {
		return fmt.Errorf("failed to create directory %s: %w", dir, err)
	}
	return nil
}

func createManuscriptFile(dir string, ms pkg.Manuscript) error {
	manuscriptFilePath := filepath.Join(dir, "manuscript.yaml")
	manuscriptTemplate := static.ManuscriptTemplate
	switch ms.Sink {
	case "postgres":
		manuscriptTemplate = static.ManuscriptWithPostgresqlTemplate
	default:
	}
	return createTemplateFile(manuscriptFilePath, manuscriptTemplate, ms)
}

func createDockerComposeFile(dir string, ms *pkg.Manuscript) error {
	fmt.Printf("Debug: Creating docker-compose file in directory: %s\n", dir)
	composeFilePath := filepath.Join(dir, "docker-compose.yml")
	dockComposeTemplate := static.DockerComposeTemplate
	switch ms.Sink {
	case defaultSink:
		m, err := pkg.LoadConfig(manuscriptConfig)
		if err != nil {
			fmt.Printf("Debug: Creating new config as none exists or error loading: %v\n", err)
			m = &pkg.Config{
				BaseDir:     getHomeDir(),
				Manuscripts: []pkg.Manuscript{},
			}
		}

		ms.GraphQLImage = graphQLImage
		if runtime.GOARCH == "arm64" || runtime.GOARCH == "arm" {
			ms.GraphQLImage = graphQLARMImage
		}

		// Check if manuscript already exists in config
		var existingMs *pkg.Manuscript
		for _, manuscript := range m.Manuscripts {
			if manuscript.Name == ms.Name {
				existingMs = &manuscript
				break
			}
		}

		if existingMs != nil {
			// Use existing ports if manuscript is already configured
			ms.Port = existingMs.Port
			ms.DbPort = existingMs.DbPort
			ms.DbUser = existingMs.DbUser
			ms.DbPassword = existingMs.DbPassword
			ms.GraphQLPort = existingMs.GraphQLPort
		} else {
			// Initialize new ports using the common function
			if err := pkg.InitializePorts(ms); err != nil {
				return fmt.Errorf("failed to initialize ports: %w", err)
			}
		}

		fmt.Printf("Debug: Using ports - Flink: %d, GraphQL: %d, DB: %d\n",
			ms.Port, ms.GraphQLPort, ms.DbPort)
		dockComposeTemplate = static.DockerComposeWithPostgresqlContent
	default:
	}
	return createTemplateFile(composeFilePath, dockComposeTemplate, ms)
}

func checkDockerInstalled() error {
	_, err := exec.LookPath("docker")
	if err != nil {
		return fmt.Errorf("🔔 \033[33mDocker is not installed. Please install Docker to proceed.\033[0m\n " +
			"For macOs: https://docs.docker.com/desktop/install/mac-install/\n " +
			"For Windows: https://docs.docker.com/desktop/install/windows-install/\n " +
			"For Linux: https://docs.docker.com/desktop/install/linux/\n")
	}
	return nil
}

func startDockerContainers(dir string) error {
	var cmd *exec.Cmd

	// Check if 'docker' exists first and prefer 'docker compose' if available
	if _, err := exec.LookPath("docker"); err == nil {
		cmd = exec.Command("docker", "compose", "-f", filepath.Join(dir, "docker-compose.yml"), "up", "-d")
		err = runCommand(cmd)
		if err == nil {
			return nil
		}

		fmt.Println("Failed to start containers using 'docker compose', trying 'docker-compose':", err)
	}

	// If 'docker-compose' exists, fallback to it
	if _, err := exec.LookPath("docker-compose"); err == nil {
		cmd = exec.Command("docker-compose", "-f", filepath.Join(dir, "docker-compose.yml"), "up", "-d")
		err = runCommand(cmd)
		if err == nil {
			return nil
		}

		return fmt.Errorf("failed to start Docker containers using 'docker-compose': %w", err)
	}

	return fmt.Errorf("neither 'docker-compose' nor 'docker compose' command found")
}

func runCommand(cmd *exec.Cmd) error {
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	return cmd.Run()
}

func checkContainerStatus(ms *pkg.Manuscript) error {
	maxRetries := 10
	for i := 0; i < maxRetries; i++ {
		dockers, err := pkg.RunDockerPs()
		if err != nil {
			return fmt.Errorf("failed to get docker ps: %w", err)
		}
		for _, d := range dockers {
			if d.Name == fmt.Sprintf("%s-jobmanager-1", ms.Name) && strings.Contains(d.Status, "Up") {
				fmt.Printf("\033[32m✓ Container %s is running\n", ms.Name)
				return nil
			}
		}
		time.Sleep(5 * time.Second)
	}
	return fmt.Errorf("timeout: container %s did not reach 'Up' status after %d attempts", ms.Name, maxRetries)
}

func isContainerRunning(containerName string) (bool, error) {
	cmd := exec.Command("docker", "ps", "--filter", fmt.Sprintf("name=%s", containerName), "--filter", "status=running", "--format", "{{.Names}}")
	output, err := cmd.Output()
	if err != nil {
		return false, fmt.Errorf("failed to run docker ps command: %w", err)
	}

	containers := strings.Split(strings.TrimSpace(string(output)), "\n")
	for _, name := range containers {
		if name == containerName {
			return true, nil
		}
	}
	return false, nil
}

func createTemplateFile(filePath, tmplContent string, data interface{}) error {
	tmpl, err := template.New(filepath.Base(filePath)).Parse(tmplContent)
	if err != nil {
		return fmt.Errorf("failed to parse template: %w", err)
	}

	file, err := os.Create(filePath)
	if err != nil {
		return fmt.Errorf("failed to create file: %w", err)
	}
	defer file.Close()

	err = tmpl.Execute(file, data)
	if err != nil {
		return fmt.Errorf("failed to execute template: %w", err)
	}

	return nil
}

func FindAvailablePort(startPort, endPort int, exclude []int) (int, error) {
	listeningPorts, err := pkg.GetListeningPorts()
	if err != nil {
		return 0, err
	}

	portMap := make(map[int]bool)
	for _, port := range listeningPorts {
		portMap[port] = true
	}
	for _, port := range exclude {
		portMap[port] = true
	}

	for port := startPort; port <= endPort; port++ {
		if !portMap[port] {
			return port, nil
		}
	}

	return 0, fmt.Errorf("no available ports in the range %d-%d", startPort, endPort)
}

func promptInput(prompt, defaultVal string) string {
	fmt.Printf("\r\033[33m%s\u001B[0m", prompt)
	reader := bufio.NewReader(os.Stdin)
	input, _ := reader.ReadString('\n')
	input = strings.TrimSpace(input)
	if input == "" {
		return defaultVal
	}
	return input
}

func checkDockerContainerExists(manuscriptName string) bool {
	dockers, err := pkg.RunDockerPs()
	if err != nil {
		log.Fatalf("Error fetching Docker containers: %v", err)
	}
	for _, d := range dockers {
		if d.Name == manuscriptName {
			return true
		}
	}
	return false
}

func fetchChainBaseDatasets() ([]*client.ChainBaseDatasetListItem, error) {
	var chains []*client.ChainBaseDatasetListItem
	err := pkg.ExecuteStepWithLoading("Checking Datasets From Network", false, func() error {
		c := client.NewChainBaseClient(networkChainURL)
		var err error
		chains, err = c.GetChainBaseDatasetList()
		if err != nil {
			return err
		}
		return nil
	})
	return chains, err
}

func selectChain(chains []*client.ChainBaseDatasetListItem, prompt, defaultChain string) (string, string) {
	fmt.Println("\r\033[33m" + prompt + "\u001B[0m")
	for i := len(chains) - 1; i >= 0; i-- {
		chain := chains[i]
		fmt.Printf("%d: %s (Database: %s)\n", i+1, chain.Name, chain.DatabaseName)
	}
	chainChoice := promptInput("🏂 3. Enter your chain choice (default is zkevm)\u001B[0m: ", "")
	if chainChoice == "" {
		fmt.Printf("\u001B[32m✓ Defaulting to chain: %s\u001B[0m\n\n", defaultChain)
		return defaultChain, defaultChain
	}
	index, err := strconv.Atoi(chainChoice)
	if err != nil || index < 1 || index > len(chains) {
		fmt.Printf("Invalid choice. Defaulting to chain: %s\n", defaultChain)
		return defaultChain, defaultChain
	}
	fmt.Printf("\u001B[32m✓ Selected chain: %s\n\n", chains[index-1].Name)
	return chains[index-1].Name, chains[index-1].DatabaseName
}

func selectTable(chains []*client.ChainBaseDatasetListItem, selectedChain, prompt, defaultTable string) string {
	defaultChainIndex := 1
	fmt.Println("\r\033[33m" + prompt + "\u001B[0m")
	for i, table := range chains[defaultChainIndex].Tables {
		fmt.Printf("%d: %s\n", i+1, table)
	}
	tableChoice := promptInput("Enter your choice(default is blocks)\u001B[0m: ", "")
	if tableChoice == "" {
		fmt.Printf("\u001B[32m✓ Defaulting to table: %s\u001B[0m\n\n", defaultTable)
		return defaultTable
	}
	index, err := strconv.Atoi(tableChoice)
	if err != nil || index < 1 || index > len(chains[defaultChainIndex].Tables) {
		fmt.Printf("Invalid choice. Defaulting to table: %s\n", defaultTable)
		return defaultTable
	}
	tableName := chains[defaultChainIndex].Tables[index-1]
	if tableName == "transactionLogs" {
		tableName = "transaction_logs"
	}
	fmt.Printf("\u001B[32m✓ Selected table: %s\u001B[0m\n\n", tableName)
	return tableName
}

func promptOutputTarget() string {
	fmt.Println("\033[33m📍 4. Please select a data output target:\033[0m")
	fmt.Println("1: Postgresql")
	fmt.Println("2: Print (output to console)")
	outputChoice := promptInput("Enter your choice(default is Postgresql)\u001B[0m: ", "1")
	output := defaultSink
	if outputChoice == "2" {
		output = "Print"
	}
	fmt.Printf("\u001B[32m✓ Selected output target: %s\u001B[0m\n", output)
	return output
}

func confirmProceed() bool {
	proceed := promptInput("🚀 Do you want to proceed with the above selections? (yes/no): ", "yes")
	return proceed == "yes" || proceed == "y" || proceed == ""
}

func logErrorAndReturn(message string, err error) {
	if err != nil {
		fmt.Printf("\033[31mError: %s: %v\033[0m\n", message, err)
	} else {
		fmt.Printf("\033[31mError: %s\033[0m\n", message)
	}
	return
}

func getHomeDir() string {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		log.Fatalf("Failed to get user home directory: %v\n", err)
	}
	return homeDir
}
