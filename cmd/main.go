package main

import (
	"adapters/primary/middlewares"
	"log"

	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/adaptor"
)

const ServiceName = "Go Hexagonal Domain Driven Design"

func main() {
	app := fiber.New()
	app.Use(adaptor.HTTPMiddleware(middlewares.LogMiddleware))

	app.Get("/", func(c *fiber.Ctx) error {
		return c.SendString("Welcome to " + ServiceName)
	})

	log.Fatal(app.Listen(":3000"))
}
