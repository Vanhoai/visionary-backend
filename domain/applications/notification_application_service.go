package applications

import "go-hexagonal-architecture/domain/services"

type NotificationApplicationService struct {
	notificationService *services.NotificationService `inject:"NotificationService"`
}

func NewNotificationApplicationService(notificationService *services.NotificationService) *NotificationApplicationService {
	return &NotificationApplicationService{
		notificationService: notificationService,
	}
}
