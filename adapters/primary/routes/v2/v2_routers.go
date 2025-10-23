package v2

import (
	"visionary-backend/adapters/primary/routes/v2/handlers"

	"github.com/gofiber/fiber/v2"
)

type V2Routers struct {
	achievementHandler handlers.AchievementHandler
}

func NewV2Routers(achievementHandler handlers.AchievementHandler) *V2Routers {
	return &V2Routers{
		achievementHandler: achievementHandler,
	}
}

func (r *V2Routers) Setup(app fiber.Router) {
	v2 := app.Group("/v2")

	// Achievement routes
	achievement := v2.Group("/achievements")
	achievement.Get("/", r.achievementHandler.GetAchievements)
}
