package applications

import "visionary-backend/domain/services"

type NotificationAppService struct {
	notificationService *services.NotificationService `inject:"NotificationService"`
}

func NewNotificationAppService(notificationService *services.NotificationService) *NotificationAppService {
	return &NotificationAppService{
		notificationService: notificationService,
	}
}
