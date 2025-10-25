package helpers

import (
	"visionary-backend/core/https"
	"visionary-backend/core/utilities"

	"github.com/gofiber/fiber/v2"
)

func ParseAndValidateBody[T any](ctx *fiber.Ctx) (*T, error) {
	body := new(T)

	if err := ctx.BodyParser(body); err != nil {
		return nil, https.NewBadRequestError(err.Error(), nil)
	}

	if err := utilities.ValidateStruct(body); err != nil {
		return nil, err
	}

	return body, nil
}
