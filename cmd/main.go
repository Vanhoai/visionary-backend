package main

import (
	"log"
	"visionary-backend/adapters/primary/handlers"
	"visionary-backend/adapters/primary/routes"
	"visionary-backend/adapters/secondary/apis"
	"visionary-backend/adapters/secondary/repositories/scylla"
	"visionary-backend/core/config"
	"visionary-backend/core/di"
	"visionary-backend/core/safe"
	"visionary-backend/core/utilities"
	"visionary-backend/domain/applications"
	"visionary-backend/domain/services"

	"github.com/goccy/go-json"
	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/logger"
	"github.com/gofiber/fiber/v2/middleware/recover"
)

// Initialize application services, repositories, and other dependencies here
func initializeServices(container *di.DIC) {
	// Register database connections
	safe.MustNoValue(container.RegisterInstance("ScyllaDatabase", &scylla.ScyllaDatabase{
		Connection: "127.0.0.1",
		Port:       9042,
	}))

	// Register repositories
	safe.MustNoValue(container.Singleton("AccountRepository", scylla.AccountRepositoryImpl{}))

	// Register apis
	safe.MustNoValue(container.Singleton("AuthApi", apis.AuthApiImpl{}))

	// Register services
	safe.MustNoValue(container.Singleton("AuthService", services.AuthService{}))
	safe.MustNoValue(container.Singleton("AccountService", services.AccountService{}))

	// Register application services
	safe.MustNoValue(container.Singleton("AccountAppService", applications.AccountAppService{}))
	safe.MustNoValue(container.Singleton("AuthAppService", applications.AuthAppService{}))

	// accountRepository := di.MustResolveTyped[*scylla.AccountRepositoryImpl](container, "AccountRepository")
	// accountService := di.MustResolveTyped[*services.AccountService](container, "AccountService")
	// accountAppService := di.MustResolveTyped[*applications.AccountAppService](container, "AccountAppService")

	// fmt.Printf("AccountRepository instance: %v\n", accountRepository)
	// fmt.Printf("AccountService instance: %v\n", accountService)
	// fmt.Printf("AccountAppService instance: %v\n", accountAppService)

	// Print all registered services
	// fmt.Println("Registered services:", container.GetRegisteredServices())
}

func initRouters(app *fiber.App, container *di.DIC) {
	// Resolve application services
	authAppService := di.MustResolveTyped[*applications.AuthAppService](container, "AuthAppService")

	// Setup middlewares
	app.Use(logger.New())
	app.Use(recover.New())

	// Setup routers
	routers := routes.NewRouter(app, authAppService)
	routers.Setup(container)
}

func main() {
	// Load configuration & common initializations
	config.Init()
	utilities.InitValidator()

	// Initialize services
	container := di.NewDIC()
	initializeServices(container)

	// Initialize app & routers
	app := fiber.New(fiber.Config{
		JSONEncoder:  json.Marshal,
		JSONDecoder:  json.Unmarshal,
		ErrorHandler: handlers.ErrorHandler,
	})

	initRouters(app, container)

	address := config.GlobalConfig.GetServerAddress()
	log.Fatal(app.Listen(address))
}
