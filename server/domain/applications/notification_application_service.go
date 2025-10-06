package applications

import (
	"server/domain/entities"
	"server/domain/services"
)

type NotificationApplicationService struct {
	notificationService *services.NotificationService
}

func (applicationService *NotificationApplicationService) FindByAccountID(accountID string) ([]*entities.Notification, error) {
	return []*entities.Notification{}, nil
}
