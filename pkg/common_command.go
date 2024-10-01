package pkg

import (
	"bufio"
	"bytes"
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
