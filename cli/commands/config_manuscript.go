package commands

import (
	"encoding/json"
	"fmt"
	"log"
	"manuscript-core/pkg"
	"os"
	"strings"
)

func ConfigLocation() {
	configPath := os.ExpandEnv(manuscriptConfig)
	fmt.Printf("📁 Manuscript config location: %s\n", configPath)
}

func ConfigShow() {
	config, err := pkg.LoadConfig(manuscriptConfig)
	if err != nil {
		log.Fatalf("Error: Failed to load manuscript config: %v", err)
	}

	// Convert config to JSON for pretty printing
	jsonData, err := json.MarshalIndent(config, "", "  ")
	if err != nil {
		log.Fatalf("Error: Failed to marshal config: %v", err)
	}

	fmt.Printf("📋 Manuscript Configuration:\n\n%s\n", string(jsonData))
}

func ConfigClean(all bool, force bool, manuscripts []string) {
	config, err := pkg.LoadConfig(manuscriptConfig)
	if err != nil {
		log.Fatalf("Error: Failed to load manuscript config: %v", err)
	}

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

		if err != nil {
			log.Fatalf("Error: Failed to clean config: %v", err)
		}

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

		if shouldKeep {
			remainingManuscripts = append(remainingManuscripts, ms)
		}
	}

	if removedCount > 0 {
		err := pkg.SaveConfigFresh(manuscriptConfig, &pkg.Config{
			BaseDir:     config.BaseDir,
			SystemInfo:  config.SystemInfo,
			Manuscripts: remainingManuscripts,
		})

		if err != nil {
			log.Fatalf("Error: Failed to update config: %v", err)
		}

		fmt.Printf("🧹 Successfully removed %d manuscript(s)\n", removedCount)
	} else {
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

func confirmRemoval() bool {
	var response string
	fmt.Scanln(&response)
	return strings.ToLower(response) == "y"
}
