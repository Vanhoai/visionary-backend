package routes

import (
	v1 "visionary-backend/adapters/primary/routes/v1"
	v1handlers "visionary-backend/adapters/primary/routes/v1/handlers"
	v2 "visionary-backend/adapters/primary/routes/v2"
	v2handlers "visionary-backend/adapters/primary/routes/v2/handlers"
	"visionary-backend/core/di"
	"visionary-backend/domain/applications"

	"github.com/gofiber/fiber/v2"
)

type Router struct {
	app            *fiber.App
	authAppService *applications.AuthAppService
}

func NewRouter(
	app *fiber.App,
	authAppService *applications.AuthAppService,
) *Router {
	return &Router{
		app:            app,
		authAppService: authAppService,
	}
}

func (r *Router) Setup(container *di.DIC) error {
	api := r.app.Group("/api")

	authAppService := di.MustResolveTyped[*applications.AuthAppService](container, "AuthAppService")

	// Setup V1
	v1AuthHandler := v1handlers.NewAuthHandler(authAppService)

	v1Routes := v1.NewV1Routers(v1AuthHandler)
	v1Routes.Setup(api)

	// Setup V2
	v2AchievementHandler := v2handlers.NewAchievementHandler()
	v2Routes := v2.NewV2Routers(v2AchievementHandler)
	v2Routes.Setup(api)

	return nil
}
