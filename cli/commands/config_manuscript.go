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

func ConfigClean(selective bool, manuscripts []string) {
	config, err := pkg.LoadConfig(manuscriptConfig)
	if err != nil {
		log.Fatalf("Error: Failed to load manuscript config: %v", err)
	}

	// Keep track of original system settings
	originalBaseDir := config.BaseDir
	originalSystemInfo := config.SystemInfo

	if !selective {
		// Clean all manuscripts mode
		fmt.Print("⚠️  Warning: This will remove all manuscript configurations. Continue? (y/N): ")
		var response string
		fmt.Scanln(&response)

		if strings.ToLower(response) != "y" {
			fmt.Println("Operation cancelled.")
			return
		}

		// Create new config preserving system settings
		err := pkg.SaveConfig(manuscriptConfig, &pkg.Config{
			BaseDir:     originalBaseDir,    // Preserve original BaseDir
			SystemInfo:  originalSystemInfo, // Preserve original SystemInfo
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
		if len(manuscripts) > 0 {
			// If specific manuscripts were provided
			if contains(manuscripts, ms.Name) {
				fmt.Printf("Remove manuscript '%s'? (y/N): ", ms.Name)
				var response string
				fmt.Scanln(&response)

				if strings.ToLower(response) != "y" {
					remainingManuscripts = append(remainingManuscripts, ms)
					fmt.Printf("Keeping manuscript '%s'\n", ms.Name)
				} else {
					fmt.Printf("Removed manuscript '%s'\n", ms.Name)
					removedCount++
				}
			} else {
				remainingManuscripts = append(remainingManuscripts, ms)
			}
		} else {
			// If no specific manuscripts were provided, ask for each
			fmt.Printf("Remove manuscript '%s'? (y/N): ", ms.Name)
			var response string
			fmt.Scanln(&response)

			if strings.ToLower(response) != "y" {
				remainingManuscripts = append(remainingManuscripts, ms)
				fmt.Printf("Keeping manuscript '%s'\n", ms.Name)
			} else {
				fmt.Printf("Removed manuscript '%s'\n", ms.Name)
				removedCount++
			}
		}
	}

	// Save updated config
	err = pkg.SaveConfig(manuscriptConfig, &pkg.Config{
		BaseDir:     originalBaseDir,
		SystemInfo:  originalSystemInfo,
		Manuscripts: remainingManuscripts,
	})

	if err != nil {
		log.Fatalf("Error: Failed to update config: %v", err)
	}

	if removedCount > 0 {
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
