package handlers

import (
	"visionary-backend/domain/usecases"

	"github.com/gofiber/fiber/v2"
)

type AuthHandler interface {
	SignInWithEmail(ctx *fiber.Ctx) error
	RefreshToken(ctx *fiber.Ctx) error
}

func NewAuthHandler() AuthHandler {
	return &authHandlerImpl{}
}

type authHandlerImpl struct{}

func (a *authHandlerImpl) SignInWithEmail(ctx *fiber.Ctx) error {
	return ctx.JSON(&usecases.AuthResponse{
		AccessToken:  "dummy_access_token",
		RefreshToken: "dummy_refresh_token",
	})
}

func (a *authHandlerImpl) RefreshToken(ctx *fiber.Ctx) error {
	return ctx.JSON(&usecases.AuthResponse{
		AccessToken:  "dummy_access_token",
		RefreshToken: "dummy_refresh_token",
	})
}
