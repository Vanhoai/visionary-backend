package repositories

import "go-hexagonal-architecture/domain/entities"

type NotificationRepository interface {
	BaseRepository[*entities.Notification]
}
