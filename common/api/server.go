package api

import (
	"fmt"
	"log"
	"net"
	"net/http"
)

var APIPort int

func StartServer() (int, error) {
	mux := http.NewServeMux()

	registerAPIHandlers(mux)

	listener, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		return 0, fmt.Errorf("failed to bind to a free port: %w", err)
	}
	APIPort = listener.Addr().(*net.TCPAddr).Port

	ready := make(chan struct{})
	go func() {
		close(ready)
		log.Printf("API server started at http://127.0.0.1:%d", APIPort)
		if err := http.Serve(listener, mux); err != nil {
			log.Printf("API server failed: %s", err)
		}
	}()
	<-ready
	return APIPort, nil
}

func registerAPIHandlers(mux *http.ServeMux) {
	mux.HandleFunc("/health", healthHandler)
	mux.HandleFunc("/load_config", loadConfigHandler)
	mux.HandleFunc("/get_job_status", getJobStatusHandler)
	mux.HandleFunc("/list_job_statuses", listJobStatusesHandler)
}
