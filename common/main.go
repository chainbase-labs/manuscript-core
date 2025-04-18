package main

import (
	"fmt"
	"manuscript-core/common/api"
)

func main() {
	port, err := api.StartServer()
	if err != nil {
		panic(err)
	}
	fmt.Printf("API server started on port %d\n", port)
	select {} // 阻塞，防止进程退出
}
