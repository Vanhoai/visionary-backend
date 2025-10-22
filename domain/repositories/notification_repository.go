package repositories

import "visionary-backend/domain/entities"

type NotificationRepository interface {
	BaseRepository[*entities.Notification]
}
