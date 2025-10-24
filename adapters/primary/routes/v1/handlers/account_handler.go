package handlers

import (
	"visionary-backend/domain/applications"

	"github.com/gofiber/fiber/v2"
)

type AccountHandler interface {
	GetAccountById(ctx *fiber.Ctx) error
}

type accountHandlerImpl struct {
	AccountAppService *applications.AccountAppService `inject:"AccountAppService"`
}

func NewAccountHandler() AccountHandler {
	return &accountHandlerImpl{}
}

func (a *accountHandlerImpl) GetAccountById(ctx *fiber.Ctx) error {
	return ctx.JSON(fiber.Map{
		"id":   "dummy_account_id",
		"name": "Dummy Account",
	})
}
