package scylla

import (
	"go-hexagonal-architecture/domain/entities"
	"go-hexagonal-architecture/domain/repositories"
)

type notificationRepositoryImpl struct {
	baseRepositoryImpl[*entities.Notification]
}

func NewNotificationRepository() repositories.NotificationRepository {
	return &notificationRepositoryImpl{}
}

type NotificationRepositoryImpl struct {
	baseRepositoryImpl[*entities.Notification]
}
