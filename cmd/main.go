package main

import (
	"go-hexagonal-architecture/adapters/primary/middlewares"
	"go-hexagonal-architecture/adapters/secondary/repositories/scylla"
	"go-hexagonal-architecture/core/config"
	"go-hexagonal-architecture/core/di"
	"log"

	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/adaptor"
)

func main() {
	// Initialize configuration
	config.Init()

	// Initialize services using dependency injection

	// Create context and cancel function
	// ctx, cancel := context.WithCancel(context.Background())
	// defer cancel()

	di := di.CreateNewContainer()

	// Register repositories
	di.SingletonV2(scylla.AccountRepositoryImpl{})
	di.SingletonV2(scylla.NotificationRepositoryImpl{})

	// di.Get("AccountRepository")
	// di.Get("NotificationRepository")

	// Register services
	// di.Singleton(services.NewAccountService)
	// di.Singleton(services.NewAuthService)

	// Register application services
	// di.Singleton(applications.NewAuthApplicationService)
	// di.Singleton(applications.NewNotificationApplicationService)

	app := fiber.New()
	app.Use(adaptor.HTTPMiddleware(middlewares.LogMiddleware))

	log.Fatal(app.Listen(":8080"))
}
