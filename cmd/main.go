package main

import (
	"fmt"
	"log"
	"visionary-backend/adapters/primary/routes"
	"visionary-backend/adapters/secondary/apis"
	"visionary-backend/adapters/secondary/repositories/scylla"
	"visionary-backend/core/config"
	"visionary-backend/core/di"
	"visionary-backend/core/safe"
	"visionary-backend/domain/applications"
	"visionary-backend/domain/services"

	"github.com/goccy/go-json"
	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/recover"
)

// Initialize application services, repositories, and other dependencies here
func initializeServices(container *di.DIC) error {
	// Register database connections
	container.RegisterInstance("ScyllaDatabase", &scylla.ScyllaDatabase{
		Connection: "127.0.0.1",
		Port:       9042,
	})

	// Register repositories
	container.Singleton("AccountRepository", scylla.AccountRepositoryImpl{})

	// Register apis
	container.Singleton("AuthApi", apis.AuthApiImpl{})

	// Register services
	container.Singleton("AuthService", services.AuthService{})
	container.Singleton("AccountService", services.AccountService{})

	// Register application services
	container.Singleton("AccountAppService", applications.AccountAppService{})
	container.Singleton("AuthAppService", applications.AuthAppService{})

	// accountRepository := di.MustResolveTyped[*scylla.AccountRepositoryImpl](container, "AccountRepository")
	// accountService := di.MustResolveTyped[*services.AccountService](container, "AccountService")
	// accountAppService := di.MustResolveTyped[*applications.AccountAppService](container, "AccountAppService")

	// fmt.Printf("AccountRepository instance: %v\n", accountRepository)
	// fmt.Printf("AccountService instance: %v\n", accountService)
	// fmt.Printf("AccountAppService instance: %v\n", accountAppService)

	// Print all registered services
	// fmt.Println("Registered services:", container.GetRegisteredServices())
	return nil
}

func initRouters(app *fiber.App, container *di.DIC) error {
	// Resolve application services
	authAppService := di.MustResolveTyped[*applications.AuthAppService](container, "AuthAppService")

	// Setup middlewares
	app.Use(recover.New())

	// Setup routers
	routers := routes.NewRouter(app, authAppService)
	routers.Setup(container)

	// Print all registered routes
	for _, route := range app.GetRoutes() {
		if route.Path == "/" || route.Method == "HEAD" {
			continue
		}

		fmt.Printf("Method: %s, Path: %s\n", route.Method, route.Path)
	}

	return nil
}

func main() {
	// Load configuration
	config.Init()

	// Initialize services
	container := di.NewDIC()
	safe.MustNoValue(initializeServices(container))

	// Initialize app & routers
	app := fiber.New(fiber.Config{
		JSONEncoder: json.Marshal,
		JSONDecoder: json.Unmarshal,
	})

	initRouters(app, container)

	address := config.GlobalConfig.GetServerAddress()
	log.Fatal(app.Listen(address))
}
