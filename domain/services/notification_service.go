package services

import "go-hexagonal-architecture/domain/repositories"

type NotificationService struct {
	repository repositories.NotificationRepository
}

func NewNotificationService(repository repositories.NotificationRepository) *NotificationService {
	return &NotificationService{repository}
}
