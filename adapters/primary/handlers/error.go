package handlers

import (
	"errors"
	"visionary-backend/core/https"

	"github.com/gofiber/fiber/v2"
)

func ErrorHandler(ctx *fiber.Ctx, err error) error {
	// Handler AppError
	var appError *https.AppError
	if errors.As(err, &appError) {
		return ctx.Status(appError.StatusCode).JSON(https.NewErrorResponse(appError))
	}

	// Handler fiber.Error
	var fiberErr *fiber.Error
	if errors.As(err, &fiberErr) {
		appError = &https.AppError{
			Code:       https.BadRequest,
			Message:    fiberErr.Message,
			StatusCode: fiberErr.Code,
		}

		return ctx.Status(fiberErr.Code).JSON(https.NewErrorResponse(appError))
	}

	// Handle unknown errors
	appError = https.NewInternalServerError("An unexpected error occurred", nil)
	return ctx.Status(500).JSON(https.NewErrorResponse(appError))
}
