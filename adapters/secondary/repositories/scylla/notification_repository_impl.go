package scylla

import (
	"visionary-backend/domain/entities"
)

type NotificationRepositoryImpl struct {
	baseRepositoryImpl[*entities.Notification]
	Database *ScyllaDatabase `inject:"ScyllaDatabase"`
}
