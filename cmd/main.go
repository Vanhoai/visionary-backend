package main

import (
	"log"
	"visionary-backend/adapters/primary/routes"
	"visionary-backend/core/config"
	"visionary-backend/core/di"
	"visionary-backend/core/safe"

	"github.com/gofiber/fiber/v2"
)

func initializeServices() error {
	// Initialize application services, repositories, and other dependencies here
	container := di.CreateNewContainer()

	// Register repositories
	// FIXME: Continue here 🫠

	// Register apis

	// Register services

	// Register application services

	return nil
}

func initRouters(app *fiber.App) error {
	routers := routes.NewRouter(app)
	routers.Setup()
	return nil
}

func main() {
	// Load configuration
	config.Init()

	// Initialize services
	safe.MustNoValue(initializeServices())

	// Initialize app & routers
	app := fiber.New()
	initRouters(app)

	address := config.GlobalConfig.GetServerAddress()
	log.Fatal(app.Listen(address))
}
