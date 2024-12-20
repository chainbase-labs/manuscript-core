package commands

import (
	"encoding/json"
	"fmt"
	"log"
	"manuscript-core/pkg"
	"os"
	"path/filepath"
	"strings"
)

// ConfigSummary is a struct to hold summarized manuscript configuration details
type ConfigSummary struct {
	ConfigLocation    string
	BaseDir           string
	DiskManuscripts   []string
	ConfigManuscripts []pkg.Manuscript
	Discrepancies     []string
}

// ConfigLocation prints the location of the manuscript config file
func ConfigLocation() {
	configPath := os.ExpandEnv(manuscriptConfig)
	fmt.Printf("📁 Manuscript config location: %s\n", configPath)
}

// ConfigShow prints the whole manuscript configuration as JSON
func ConfigShow() {
	// Load manuscript config
	config, err := pkg.LoadConfig(manuscriptConfig)
	if err != nil {
		log.Fatalf("Error: Failed to load manuscript config: %v", err)
	}

	// Convert config to JSON for pretty printing
	jsonData, err := json.MarshalIndent(config, "", "  ")
	if err != nil {
		log.Fatalf("Error: Failed to marshal config: %v", err)
	}

	// Print JSON data to console
	fmt.Printf("📋 Manuscript Configuration:\n\n%s\n", string(jsonData))
}

// ConfigShowSummary prints a summarized view of the manuscript configuration
func ConfigShowSummary() {
	// Load manuscript config
	configPath := os.ExpandEnv(manuscriptConfig)
	config, err := pkg.LoadConfig(manuscriptConfig)
	if err != nil {
		log.Fatalf("Error: Failed to load manuscript config: %v", err)
	}

	// Build summary and display
	summary := buildConfigSummary(configPath, config)
	displaySummary(summary)
}

// buildConfigSummary creates a ConfigSummary struct from the manuscript config
func buildConfigSummary(configPath string, config *pkg.Config) ConfigSummary {
	// Create summary struct
	summary := ConfigSummary{
		ConfigLocation:    configPath,
		BaseDir:           config.BaseDir,
		ConfigManuscripts: config.Manuscripts,
	}

	// Get manuscripts from disk
	if config.BaseDir != "" {
		manuscripts, err := findManuscriptsOnDisk(config.BaseDir)
		if err != nil {
			fmt.Printf("⚠️  Warning: Could not read manuscripts from disk: %v\n", err)
		}
		// Tracks manuscripts which are in the base directory but not in the config
		summary.DiskManuscripts = manuscripts
		// Find discrepancies between disk and config
		summary.Discrepancies = findDiscrepancies(manuscripts, config.Manuscripts)
	}

	// Return summary
	return summary
}

// findManuscriptsOnDisk reads the base directory and returns a list of manuscripts
func findManuscriptsOnDisk(baseDir string) ([]string, error) {
	// Find all manuscripts in the manuscripts directory
	var manuscripts []string
	manuscriptsDir := filepath.Join(baseDir, "manuscripts")

	entries, err := os.ReadDir(manuscriptsDir)
	if err != nil {
		return nil, err
	}

	// Check each entry in the directory
	for _, entry := range entries {
		if entry.IsDir() {
			// Check if manuscript.yaml exists in the directory
			yamlPath := filepath.Join(manuscriptsDir, entry.Name(), "manuscript.yaml")
			if _, err := os.Stat(yamlPath); err == nil {
				manuscripts = append(manuscripts, entry.Name())
			}
		}
	}
	// Return list of manuscripts
	return manuscripts, nil
}

// findDiscrepancies compares manuscripts on disk with those in the config
func findDiscrepancies(diskMss []string, configMss []pkg.Manuscript) []string {
	var discrepancies []string
	configMap := make(map[string]bool)
	diskMap := make(map[string]bool)

	// Create maps for easy lookup
	for _, ms := range configMss {
		configMap[ms.Name] = true
	}
	for _, ms := range diskMss {
		diskMap[ms] = true
	}

	// Find manuscripts in config but not on disk
	for _, ms := range configMss {
		if !diskMap[ms.Name] {
			discrepancies = append(discrepancies, fmt.Sprintf("'%s' exists in config but not on disk", ms.Name))
		}
	}

	// Find manuscripts on disk but not in config
	for _, ms := range diskMss {
		if !configMap[ms] {
			discrepancies = append(discrepancies, fmt.Sprintf("'%s' exists on disk but not in config", ms))
		}
	}

	// Return discrepancies as a list of strings
	return discrepancies
}

// displaySummary prints the ConfigSummary struct to the console in a human-readable format
func displaySummary(summary ConfigSummary) {
	fmt.Println("\n📊 Manuscript Configuration Summary")
	fmt.Println("──────────────────────────────────")

	fmt.Printf("📁 Config Location: \033[36m%s\033[0m\n", summary.ConfigLocation)
	fmt.Printf("📂 Manuscripts Directory: \033[36m%s%s\033[0m\n", summary.BaseDir, "/manuscripts")

	fmt.Printf("\n📈 Statistics:\n")
	fmt.Printf("   • Manuscripts on disk: \033[33m%d\033[0m\n", len(summary.DiskManuscripts))
	fmt.Printf("   • Manuscripts in config: \033[33m%d\033[0m\n", len(summary.ConfigManuscripts))

	fmt.Printf("\n🔍 Configured Manuscripts:\n")
	for _, ms := range summary.ConfigManuscripts {
		fmt.Printf("   • \033[32m%-15s\033[0m | Port: \033[36m%-5d\033[0m | DB: \033[36m%-5d\033[0m | GraphQL: \033[36m%-5d\033[0m\n",
			ms.Name, ms.Port, ms.DbPort, ms.GraphQLPort)
	}

	if len(summary.Discrepancies) > 0 {
		fmt.Printf("\n⚠️  Discrepancies Found:\n")
		for _, disc := range summary.Discrepancies {
			fmt.Printf("   • \033[31m%s\033[0m\n", disc)
		}
	} else {
		fmt.Printf("\n\033[32m✅ No discrepancies between disk and config\033[0m\n")
	}
}

// ConfigClean removes manuscript configurations from the config file
func ConfigClean(all bool, force bool, manuscripts []string) {
	// Load manuscript config
	config, err := pkg.LoadConfig(manuscriptConfig)
	if err != nil {
		log.Fatalf("Error: Failed to load manuscript config: %v", err)
	}

	// check if all flag is set
	if all {
		if !force {
			fmt.Print("⚠️  Warning: This will remove all manuscript configurations. Continue? (y/N): ")
			var response string
			fmt.Scanln(&response)
			if strings.ToLower(response) != "y" {
				fmt.Println("Operation cancelled.")
				return
			}
		}

		// Keep system settings but remove all manuscripts
		err := pkg.SaveConfigFresh(manuscriptConfig, &pkg.Config{
			BaseDir:     config.BaseDir,
			SystemInfo:  config.SystemInfo,
			Manuscripts: []pkg.Manuscript{},
		})

		// Check for errors
		if err != nil {
			log.Fatalf("Error: Failed to clean config: %v", err)
		}

		// Print success message
		fmt.Println("🧹 All manuscript configurations cleaned successfully!")
		return
	}

	// Selective cleaning mode
	var remainingManuscripts []pkg.Manuscript
	var removedCount int

	for _, ms := range config.Manuscripts {
		shouldKeep := true

		// If specific manuscripts provided
		if len(manuscripts) > 0 {
			if contains(manuscripts, ms.Name) {
				if !force {
					fmt.Printf("Remove manuscript '%s'? (y/N): ", ms.Name)
					var response string
					fmt.Scanln(&response)
					if strings.ToLower(response) == "y" {
						shouldKeep = false
						fmt.Printf("Removed manuscript '%s'\n", ms.Name)
						removedCount++
					} else {
						fmt.Printf("Keeping manuscript '%s'\n", ms.Name)
					}
				} else {
					shouldKeep = false
					fmt.Printf("Removed manuscript '%s'\n", ms.Name)
					removedCount++
				}
			} else {
				shouldKeep = true
			}
		} else {
			// No specific manuscripts provided, ask for each
			if !force {
				fmt.Printf("Remove manuscript '%s'? (y/N): ", ms.Name)
				var response string
				fmt.Scanln(&response)
				if strings.ToLower(response) == "y" {
					shouldKeep = false
					fmt.Printf("Removed manuscript '%s'\n", ms.Name)
					removedCount++
				} else {
					fmt.Printf("Keeping manuscript '%s'\n", ms.Name)
				}
			} else {
				shouldKeep = false
				fmt.Printf("Removed manuscript '%s'\n", ms.Name)
				removedCount++
			}
		}

		// Add manuscript to remaining list if it should be kept
		if shouldKeep {
			remainingManuscripts = append(remainingManuscripts, ms)
		}
	}

	// Save updated config with remaining manuscripts
	if removedCount > 0 {
		err := pkg.SaveConfigFresh(manuscriptConfig, &pkg.Config{
			BaseDir:     config.BaseDir,
			SystemInfo:  config.SystemInfo,
			Manuscripts: remainingManuscripts,
		})

		// Check for errors
		if err != nil {
			log.Fatalf("Error: Failed to update config: %v", err)
		}

		// Print success message
		fmt.Printf("🧹 Successfully removed %d manuscript(s)\n", removedCount)
	} else {
		// No manuscripts removed
		fmt.Println("ℹ️  No manuscripts were removed")
	}
}

// Helper function to check if a string is in a slice
func contains(slice []string, str string) bool {
	for _, v := range slice {
		if v == str {
			return true
		}
	}
	return false
}

// Helper function removeManuscriptsFromConfig removes specific manuscripts from the config
func removeManuscriptsFromConfig(config *pkg.Config, manuscriptsToRemove []string, force bool) (*pkg.Config, int) {
	var remainingManuscripts []pkg.Manuscript
	removedCount := 0

	for _, ms := range config.Manuscripts {
		shouldKeep := true

		// If specific manuscripts listed, check if this one should be removed
		if len(manuscriptsToRemove) > 0 {
			if contains(manuscriptsToRemove, ms.Name) {
				if !force {
					fmt.Printf("Remove manuscript '%s'? (y/N): ", ms.Name)
					if confirmRemoval() {
						shouldKeep = false
						removedCount++
						fmt.Printf("Removed manuscript '%s'\n", ms.Name)
					} else {
						fmt.Printf("Keeping manuscript '%s'\n", ms.Name)
					}
				} else {
					shouldKeep = false
					removedCount++
					fmt.Printf("Removed manuscript '%s'\n", ms.Name)
				}
			}
		} else {
			// No specific manuscripts listed, ask about each one
			if !force {
				fmt.Printf("Remove manuscript '%s'? (y/N): ", ms.Name)
				if confirmRemoval() {
					shouldKeep = false
					removedCount++
					fmt.Printf("Removed manuscript '%s'\n", ms.Name)
				} else {
					fmt.Printf("Keeping manuscript '%s'\n", ms.Name)
				}
			} else {
				shouldKeep = false
				removedCount++
				fmt.Printf("Removed manuscript '%s'\n", ms.Name)
			}
		}

		// Add manuscript to remaining list if it should be kept
		if shouldKeep {
			remainingManuscripts = append(remainingManuscripts, ms)
		}
	}

	// Create new config with same settings but updated manuscript list
	return &pkg.Config{
		BaseDir:     config.BaseDir,
		SystemInfo:  config.SystemInfo,
		Manuscripts: remainingManuscripts,
	}, removedCount
}

// Helper function to confirm removal of a manuscript
func confirmRemoval() bool {
	var response string
	fmt.Scanln(&response)
	return strings.ToLower(response) == "y"
}
