package main

import (
	"fmt"
	"visionary-backend/core/config"
)

func main() {
	config.Init()
	fmt.Printf("%+v\n", config.GlobalConfig.Cryptography)
}
