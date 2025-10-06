package services

import "server/domain/entities"

type NotificationService struct{}

func (service *NotificationService) FindByAccountID(accountID string) []*entities.Notification {
	return []*entities.Notification{}
}
