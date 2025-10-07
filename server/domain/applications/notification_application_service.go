package applications

import (
	"server/domain/services"
)

type NotificationApplicationService struct {
	notificationService *services.NotificationService `inject:"NotificationService"`
}
