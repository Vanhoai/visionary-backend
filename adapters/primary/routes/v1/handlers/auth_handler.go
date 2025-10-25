package handlers

import (
    "visionary-backend/adapters/shared/helpers"
    "visionary-backend/core/https"
    "visionary-backend/core/safe"
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

func (handler *authHandlerImpl) SignIn(ctx *fiber.Ctx) error {
    return safe.TryNoValue(func() error {
        body := safe.Must(helpers.ParseAndValidateBody[usecases.EmailPasswordReq](ctx))
        response := safe.Must(handler.authAppService.SignIn(body))
        
        return ctx.Status(fiber.StatusOK).JSON(https.ResponseSuccess(
            response,
            "Sign in successful 🫠",
        ))
    })
}

func (handler *authHandlerImpl) RefreshToken(ctx *fiber.Ctx) error {
    body := new(usecases.RefreshTokenReq)
    if err := ctx.BodyParser(body); err != nil {
        return https.NewBadRequestError("Invalid request body", nil)
    }

    response, err := handler.authAppService.RefreshToken(body)
    if err != nil {
        return err
    }

    return ctx.Status(fiber.StatusOK).JSON(https.ResponseSuccess(
        response,
        "Token refreshed successfully 🫠",
    ))
}
