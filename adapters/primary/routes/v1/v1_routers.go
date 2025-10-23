package v1

import (
	"visionary-backend/adapters/primary/routes/v1/handlers"

	"github.com/gofiber/fiber/v2"
)

type V1Routers struct {
	authHandler    handlers.AuthHandler
	accountHandler handlers.AccountHandler
}

func NewV1Routers(authHandler handlers.AuthHandler, accountHandler handlers.AccountHandler) *V1Routers {
	return &V1Routers{
		authHandler:    authHandler,
		accountHandler: accountHandler,
	}
}

func (r *V1Routers) Setup(app fiber.Router) {
	v1 := app.Group("/v1")

	// Auth routes
	auth := v1.Group("/auth")
	auth.Post("/sign-in-with-email", r.authHandler.SignInWithEmail)
	auth.Post("/refresh-token", r.authHandler.RefreshToken)

	// Account routes
	account := v1.Group("/account")
	account.Get("/:id", r.accountHandler.GetAccountById)
}
