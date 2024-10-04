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
	"strconv"
	"strings"
	"text/template"
	"time"
)

func executeInitManuscript(ms pkg.Manuscript) {
	manuscriptName := strings.ToLower(strings.ReplaceAll(ms.Name, " ", "_"))
	manuscriptDir := fmt.Sprintf("manuscript/%s", manuscriptName)

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
	log.Printf("\033[32mYou can now list your job with the command: \n👉 \033[33mmanuscript-cli job list\n\n"+
		"\033[32mIf you need to manually edit the manuscript, "+
		"you can edit the file '%s/manuscript.yaml' and then manually execute the 'run' command:\n"+
		"👉 \033[33mmanuscript-cli run %s/manuscript.yaml\n\n", manuscriptDir, manuscriptDir)
	log.Printf("\033[32mYou can now access your manuscript at http://localhost:%d\n", ms.Port)
}

func InitManuscript() {
	fmt.Printf("\r\033[33m🏂 1. Enter your manuscript name: (default is demo)\033[0m")
	reader := bufio.NewReader(os.Stdin)
	manuscriptName, _ := reader.ReadString('\n')
	manuscriptName = strings.TrimSpace(manuscriptName)
	if manuscriptName == "" {
		manuscriptName = "demo"
	}

	dockers, err := pkg.RunDockerPs()
	if err != nil {
		log.Fatalf("Error: Failed to get docker ps: %v", err)
	}
	for _, d := range dockers {
		if d.Name == manuscriptName {
			fmt.Printf("\033[33mError: Manuscript with name [ %s ] already exists. Please choose a different name.\033[0m\n", manuscriptName)
			return
		}
	}

	var chains []*client.ChainBaseDatasetListItem
	err = pkg.ExecuteStepWithLoading("Checking Datasets From Network", false, func() error {
		c := client.NewChainBaseClient("https://api.chainbase.com")
		var err error
		chains, err = c.GetChainBaseDatasetList()
		if err != nil {
			fmt.Printf("Error: %v\n", err)
			return err
		}
		return nil
	})
	if err != nil {
		log.Fatalf("\033[31m✗ %s failed: %v\n", fmt.Sprintf("Step %d", 1), err)
	}

	fmt.Println("\r\033[33m🏂 2.Please select a chainbase network dataset from the list below:\033[0m")
	for i := len(chains) - 1; i >= 0; i-- {
		chain := chains[i]
		fmt.Printf("%d: %s (Database: %s)\n", i+1, chain.Name, chain.DatabaseName)
	}

	fmt.Print("\r\033[33m🏂 1.Enter your chain choice (default is zkevm): \033[0m")
	chainChoice, _ := reader.ReadString('\n')
	chainChoice = strings.TrimSpace(chainChoice)

	selectedChain := "zkevm"
	selectedDatabase := "zkevm"
	defaultChainIndex := 1

	if chainChoice != "" {
		index, err := strconv.Atoi(chainChoice)
		if err != nil || index < 1 || index > len(chains) {
			fmt.Printf("Invalid choice. Please enter a number between 1 and %d.\n", len(chains))
			return
		}
		selectedChain = chains[index-1].Name
		selectedDatabase = chains[index-1].DatabaseName
		defaultChainIndex = index - 1
		fmt.Printf("\r\033[32m\u2714 You have selected chain: %s\n\033[0m\n", selectedChain)
	} else {
		fmt.Printf("No input provided. Defaulting to chain: %s\n\033[0m\n", selectedChain)
	}

	fmt.Println("\r\033[33m🧲 3.Please select a table from the list below:\033[0m")
	for i, table := range chains[defaultChainIndex].Tables {
		fmt.Printf("%d: %s\n", i+1, table)
	}

	fmt.Print("\r\033[33mEnter your choice (default is blocks): \033[0m")
	tableChoice, _ := reader.ReadString('\n')
	tableChoice = strings.TrimSpace(tableChoice)

	selectedTable := "blocks"
	if tableChoice != "" {
		index, err := strconv.Atoi(tableChoice)
		if err != nil || index < 1 || index > len(chains[defaultChainIndex].Tables) {
			fmt.Printf("Invalid choice. Please enter a number between 1 and %d.\n", len(chains[defaultChainIndex].Tables))
			return
		}
		selectedTable = chains[defaultChainIndex].Tables[index-1]
		fmt.Printf("\r\033[32m\u2714 You have selected table: %s\n\033[0m\n", selectedTable)
	} else {
		fmt.Printf("\u001B[32m\u2714 No input provided. Defaulting to table: %s\n\033[0m\n", selectedTable)
	}

	defaultSQL := fmt.Sprintf("Select * From %s_%s", selectedDatabase, selectedTable)
	sqlQuery := defaultSQL

	fmt.Println("\033[33m📍 3.Please select a data output target:\033[0m")
	fmt.Println("1: Postgresql")
	fmt.Println("2: Print (output to console)")

	fmt.Print("\033[33mEnter your choice (default is Postgresql): ")
	outputChoice, _ := reader.ReadString('\n')
	outputChoice = strings.TrimSpace(outputChoice)

	selectedOutput := "postgres"
	if outputChoice != "" {
		index, err := strconv.Atoi(outputChoice)
		if err != nil || index < 1 || index > 2 {
			fmt.Printf("Invalid choice. Please enter a number between 1 and 2.\n")
			return
		}
		if index == 2 {
			selectedOutput = "Print"
		}
		fmt.Printf("\r\033[32m\u2714 You have selected output target: %s\n", selectedOutput)
	} else {
		fmt.Printf("\033[32m\u2714 No input provided. Defaulting to output target: %s\033[0m\n", selectedOutput)
	}

	fmt.Printf("\n\033[33m🏄🏄 Summary of your selections:\033[0m\n")
	fmt.Printf("Selected manuscript name: \033[32m%s\033[0m\n", manuscriptName)
	fmt.Printf("Selected chain: \033[32m%s\033[0m\n", selectedChain)
	fmt.Printf("Selected table: \u001B[32m%s\u001B[0m\n", selectedTable)
	fmt.Printf("Data output target: \u001B[32m%s\u001B[0m\n", selectedOutput)

	fmt.Print("\n\033[33m🚀 Do you want to proceed with the above selections? (yes/no): \033[0m")
	proceed, _ := reader.ReadString('\n')
	proceed = strings.TrimSpace(proceed)
	if proceed == "yes" || proceed == "y" || proceed == "" {
		ms := pkg.Manuscript{
			Name:     manuscriptName,
			Chain:    selectedChain,
			Table:    selectedTable,
			Database: selectedDatabase,
			Query:    sqlQuery,
			Sink:     selectedOutput,
		}
		executeInitManuscript(ms)
		return
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
	composeFilePath := filepath.Join(dir, "docker-compose.yml")
	dockComposeTemplate := static.DockerComposeTemplate
	switch ms.Sink {
	case "postgres":
		port, err := FindAvailablePort(8081, 8181, 0)
		if err != nil {
			return fmt.Errorf("failed to find available port: %w", err)
		}
		ms.Port = port
		graphQLPort, err := FindAvailablePort(8081, 8181, port)
		if err != nil {
			return fmt.Errorf("failed to find available port: %w", err)
		}
		ms.GraphQLPort = graphQLPort
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
	cmd := exec.Command("docker-compose", "-f", filepath.Join(dir, "docker-compose.yml"), "up", "-d")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err := cmd.Run()
	if err != nil {
		return fmt.Errorf("failed to start Docker containers: %w", err)
	}
	return nil
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

func FindAvailablePort(startPort, endPort int, exclude int) (int, error) {
	listeningPorts, err := pkg.GetListeningPorts()
	if err != nil {
		return 0, err
	}

	portMap := make(map[int]bool)
	for _, port := range listeningPorts {
		portMap[port] = true
	}
	if exclude != 0 {
		portMap[exclude] = true
	}

	for port := startPort; port <= endPort; port++ {
		if !portMap[port] {
			return port, nil
		}
	}

	return 0, fmt.Errorf("no available ports in the range %d-%d", startPort, endPort)
}
