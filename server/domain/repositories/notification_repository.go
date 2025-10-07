package repositories

import (
	"server/domain/entities"
)

type NotificationRepository interface {
	BaseRepository[*entities.Notification]
}
