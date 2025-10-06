package main

import (
	"app/adapters/primary/middlewares"
	"log"

	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/adaptor"
)

func catchme() {
	panic("oh no")
}

func main() {
	app := fiber.New()
	app.Use(adaptor.HTTPMiddleware(middlewares.LogMiddleware))

	log.Fatal(app.Listen(":3000"))
}
