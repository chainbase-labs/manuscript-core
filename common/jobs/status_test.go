package jobs

import (
	"testing"
)

func TestStatus(t *testing.T) {

	schema, err := ListJobStatuses("/Users/isachoi/.manuscript_config.ini")
	if err != nil {
		t.Fatalf("Get table schema failed: %v", err)
	}
	if schema == nil {

	}

}
