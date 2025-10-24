package v1

import (
	"visionary-backend/adapters/primary/routes/v1/handlers"

	"github.com/gofiber/fiber/v2"
)

type V1Routers struct {
	authHandler handlers.AuthHandler
}

func NewV1Routers(authHandler handlers.AuthHandler) *V1Routers {
	return &V1Routers{
		authHandler: authHandler,
	}
}

func (r *V1Routers) Setup(apiGroup fiber.Router) {
	v1 := apiGroup.Group("/v1")

	// Auth Routes
	v1.Post("/auth/sign-in/email", r.authHandler.SignIn)
	v1.Post("/auth/refresh-token", r.authHandler.RefreshToken)
}
