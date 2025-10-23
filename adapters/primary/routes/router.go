package routes

import (
	v1 "visionary-backend/adapters/primary/routes/v1"
	v1handlers "visionary-backend/adapters/primary/routes/v1/handlers"
	v2 "visionary-backend/adapters/primary/routes/v2"
	v2handlers "visionary-backend/adapters/primary/routes/v2/handlers"
	"visionary-backend/domain/applications"

	"github.com/gofiber/fiber/v2"
)

type Router struct {
	app                    *fiber.App
	authAppService         *applications.AuthApplicationService
	notificationAppService *applications.NotificationApplicationService
}

func NewRouter(app *fiber.App, authAppService *applications.AuthApplicationService, notificationAppService *applications.NotificationApplicationService) *Router {
	return &Router{
		app:                    app,
		authAppService:         authAppService,
		notificationAppService: notificationAppService,
	}
}

func (r *Router) Setup() {
	api := r.app.Group("/api")

	// Setup V1
	v1AuthHandler := v1handlers.NewAuthHandler()
	v1AccountHandler := v1handlers.NewAccountHandler()

	v1Routes := v1.NewV1Routers(v1AuthHandler, v1AccountHandler)
	v1Routes.Setup(api)

	// Setup V2
	v2AchievementHandler := v2handlers.NewAchievementHandler()
	v2Routes := v2.NewV2Routers(v2AchievementHandler)
	v2Routes.Setup(api)
}
