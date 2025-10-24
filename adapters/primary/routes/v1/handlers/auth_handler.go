package handlers

import (
	"fmt"
	"visionary-backend/domain/applications"
	"visionary-backend/domain/usecases"

	"github.com/gofiber/fiber/v2"
)

type AuthHandler interface {
	SignIn(ctx *fiber.Ctx) error
	RefreshToken(ctx *fiber.Ctx) error
}

func NewAuthHandler(authAppService *applications.AuthAppService) AuthHandler {
	return &authHandlerImpl{authAppService: authAppService}
}

type authHandlerImpl struct {
	authAppService *applications.AuthAppService
}

var TOKEN = "MIIEowIBAAKCAQEAt0bx90lnboVrcXTaNES8YQFlu+Vo8bOjaibl9a7yGRVCNPZ/3WzBt0VgjEfxxGEExx/egiHJpA6iP0t+Yjct7SZProPKB7iI9ByblbPXiTMbbqpuWvlRVzNdoJ3zR6j4+8JgiSEkBuc70IE7BRj2LO9qIbhQnPjc9arB+kTyVwCiNGyZpoQ5uer6+l5KjhB8D4ef9G9eIy9Cm+SwQy4p1yZW4sQGt0sgeswN1zGPhWiE+jFBFcrW/mKYeavf5Ur0PDCyj7ef6m2qTtFwt1OzEU1kiSTSWjAmwFbnttnFYgzxIxdL"

func (handler *authHandlerImpl) SignIn(ctx *fiber.Ctx) error {
	body := new(usecases.EmailPasswordReq)
	if err := ctx.BodyParser(body); err != nil {
		return fiber.ErrBadRequest
	}

	fmt.Printf("SignIn request received: %+v\n", body)

	response, error := handler.authAppService.SignIn(body)
	if error != nil {
		return fiber.ErrUnauthorized
	}

	return ctx.JSON(response)
}

func (handler *authHandlerImpl) RefreshToken(ctx *fiber.Ctx) error {
	return ctx.JSON(&usecases.AuthResponse{
		AccessToken:  TOKEN,
		RefreshToken: TOKEN,
	})
}
