package pkg

import (
	"bufio"
	"bytes"
	"fmt"
	"os/exec"
	"regexp"
	"strconv"
)

func GetListeningPorts() ([]int, error) {
	cmd := exec.Command("lsof", "-nP", "-iTCP", "-sTCP:LISTEN")
	var out bytes.Buffer
	cmd.Stdout = &out

	if err := cmd.Run(); err != nil {
		return nil, err
	}

	re := regexp.MustCompile(`:(\d+)\s+\(LISTEN\)`)

	var ports []int
	scanner := bufio.NewScanner(&out)
	for scanner.Scan() {
		line := scanner.Text()
		matches := re.FindStringSubmatch(line)
		if len(matches) > 1 {
			port, err := strconv.Atoi(matches[1])
			if err != nil {
				continue
			}
			ports = append(ports, port)
		}
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return ports, nil
}

func InitializePorts(ms *Manuscript) error {
	// Initialize Flink port if not set
	if ms.Port == 0 {
		port, err := FindAvailablePort(8081, 8181, nil)
		if err != nil {
			return fmt.Errorf("failed to find available port for Flink: %w", err)
		}
		ms.Port = port
	}

	// Initialize GraphQL port if not set
	if ms.GraphQLPort == 0 {
		graphQLPort, err := FindAvailablePort(8082, 8182, []int{ms.Port})
		if err != nil {
			return fmt.Errorf("failed to find available port for GraphQL: %w", err)
		}
		ms.GraphQLPort = graphQLPort
	}

	// Initialize DB port if not set
	if ms.DbPort == 0 {
		dbPort, err := FindAvailablePort(15432, 15532, []int{ms.Port, ms.GraphQLPort})
		if err != nil {
			return fmt.Errorf("failed to find available port for DB: %w", err)
		}
		ms.DbPort = dbPort
	}

	return nil
}

func FindAvailablePort(startPort, endPort int, exclude []int) (int, error) {
	listeningPorts, err := GetListeningPorts()
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
