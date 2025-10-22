package scylla

import (
	"visionary-backend/domain/entities"
	"visionary-backend/domain/repositories"
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
