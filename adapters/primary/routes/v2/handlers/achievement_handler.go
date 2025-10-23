package handlers

import "github.com/gofiber/fiber/v2"

type AchievementHandler interface {
	GetAchievements(ctx *fiber.Ctx) error
}

type achievementHandlerImpl struct{}

func NewAchievementHandler() AchievementHandler {
	return &achievementHandlerImpl{}
}

func (handler *achievementHandlerImpl) GetAchievements(ctx *fiber.Ctx) error {
	return ctx.JSON(fiber.Map{
		"achievements": []fiber.Map{
			{
				"id":   "achievement_1",
				"name": "First Steps",
			},
		},
	})
}
