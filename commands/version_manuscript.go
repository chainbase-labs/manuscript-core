package commands

import (
	"fmt"
	"os/exec"
	"runtime"
	"strings"
)

// Build-time variables set via ldflags
// Build command would be:
//
//	go build -ldflags "-X manuscript-core/commands.gitCommit=$(git rev-parse --short HEAD) \
//	                  -X manuscript-core/commands.buildDate=$(date -u '+%Y-%m-%d:%H:%M:%S') \
//						 -X manuscript-core/commands.goVersion=$(go version | cut -d' ' -f3) \
//	                  -X manuscript-core/commands.rustVersion=$(rustc --version | cut -d' ' -f2)" \
//	       -o manuscript-cli main.go
var (
	gitCommit   = "unknown" // Set by build flags
	buildDate   = "unknown" // Set by build flags
	goVersion   = "unknown" // Set by build flags
	rustVersion = "unknown" // Set by build flags
)

type VersionInfo struct {
	Version     string     `json:"version"`
	GoVersion   string     `json:"go_version"`
	RustVersion string     `json:"rust_version"`
	GitCommit   string     `json:"git_commit"`
	BuildDate   string     `json:"build_date"`
	OS          string     `json:"os"`
	Arch        string     `json:"arch"`
	DockerInfo  DockerInfo `json:"docker_info"`
}

type DockerInfo struct {
	Version        string `json:"version"`
	DesktopVersion string `json:"desktop_version"`
}

func showVersion(verbose bool) {
	info := getVersionInfo()

	if !verbose {
		fmt.Printf("manuscript-core version %s\n", info.Version)
		return
	}

	fmt.Printf("\n📜 Manuscript Core:\n")
	fmt.Printf("  Version:\t%s\n", info.Version)

	fmt.Printf("\n🏗️  Build Information:\n")
	fmt.Printf("  Go Version:\t%s\n", info.GoVersion)
	fmt.Printf("  Rust Version:\t%s\n", info.RustVersion)
	fmt.Printf("  Git Commit:\t%s\n", info.GitCommit)
	fmt.Printf("  Built:\t%s\n", info.BuildDate)

	fmt.Printf("\n💻 System Information:\n")
	uname, _ := exec.Command("uname", "-a").Output()
	fmt.Printf("  Unix Name:\t%s", string(uname))
	fmt.Printf("  OS/Arch:\t%s/%s\n", info.OS, info.Arch)

	fmt.Printf("\n🐳 Docker Environment:\n")
	if info.DockerInfo.Version != "" {
		fmt.Printf("  Docker:\t'%s'\n", info.DockerInfo.Version)
	}
	if info.DockerInfo.DesktopVersion != "" {
		fmt.Printf("  Docker Desktop:\t'%s'\n", info.DockerInfo.DesktopVersion)
	}
}

func getVersionInfo() VersionInfo {
	info := VersionInfo{
		Version:     version,
		GoVersion:   goVersion,
		RustVersion: rustVersion,
		GitCommit:   gitCommit,
		BuildDate:   buildDate,
		OS:          runtime.GOOS,
		Arch:        runtime.GOARCH,
	}

	// Get Docker version
	if dockerVersion, err := exec.Command("docker", "--version").Output(); err == nil {
		version := strings.TrimSpace(string(dockerVersion))
		parts := strings.Split(version, " ")
		if len(parts) >= 3 {
			info.DockerInfo.Version = strings.Trim(parts[2], ",")
		}
	}

	// Get Docker Desktop version
	cmd := exec.Command("docker", "version", "--format", "{{.Server}} {{.Server.Platform.Name}}")
	if desktopVersion, err := cmd.Output(); err == nil {
		// Find "Docker Desktop" and extract version
		output := string(desktopVersion)
		if strings.Contains(output, "Docker Desktop") {
			parts := strings.Split(output, "Docker Desktop")
			if len(parts) > 1 {
				info.DockerInfo.DesktopVersion = strings.Split(parts[1], " ")[1]
			}
		}
	}

	return info
}
