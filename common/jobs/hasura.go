package jobs

import (
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"strings"
)

func TrackTable(port string, table string) {
	url := fmt.Sprintf("http://127.0.0.1:%s/v1/metadata", port)

	payload := fmt.Sprintf(`{
				"type": "bulk",
				"source": "default", 
				"resource_version": 1,
				"args": [{
					"type": "postgres_track_tables",
					"args": {
						"allow_warnings": true,
						"tables": [{
							"table": {
								"name": "%s",
								"schema": "public"
							},
							"source": "default"
						}]
					}
				}]
			}`, table)

	resp, err := http.Post(url, "application/json", strings.NewReader(payload))
	if err != nil {
		return
	}
	defer resp.Body.Close()
	body, _ := ioutil.ReadAll(resp.Body)
	if resp.StatusCode != http.StatusOK {
		log.Printf("deployment failed with status: %s, %d", string(body), resp.StatusCode)
	}

}

func GetTableSchema(port, table string) (string, error) {
	url := fmt.Sprintf("http://127.0.0.1:%s/v1/metadata", port)

	payload := fmt.Sprintf(`{
  "type": "pg_get_table_info",
  "args": {
    "source": "default",
    "table": {
      "schema": "public",
      "name": "%s"
    }
  }
}`, table)
	resp, err := http.Post(url, "application/json", strings.NewReader(payload))
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()
	body, _ := ioutil.ReadAll(resp.Body)
	if resp.StatusCode != http.StatusOK {
		log.Printf("deployment failed with status: %s", string(body))
		return "", fmt.Errorf("deployment failed with status: %s, code: %d", string(body), resp.StatusCode)
	}

	return string(body), nil
}
