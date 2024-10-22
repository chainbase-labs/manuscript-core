package client

import (
	"strings"
	"testing"
)

func TestFlinkClient(t *testing.T) {
	t.Skip("Skip FlinkClient test")
	client := NewFlinkClient(strings.TrimPrefix("127.0.0.1:8083", "http://"), "use catalog default_catalog; use default_database;")

	if err := client.InitializeClient(); err != nil {
		t.Fatalf("Failed to initialize client: %v", err)
	}

	operationHandle, err := client.ExecuteSQL("select 1;")
	if err != nil {
		t.Fatalf("Failed to execute SQL: %v", err)
	}

	if operationHandle == "" {
		t.Fatalf("Unexpected operation handle: %s", operationHandle)
	}

	if err := client.CheckSQLResult(operationHandle, 60); err != nil {
		t.Fatalf("Failed to check SQL result: %v", err)
	}
}
