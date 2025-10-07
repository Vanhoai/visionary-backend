package usecases

import "server/domain/entities"

// ======================== MANAGE NOTIFICATION ========================
type ManageNotificationUseCases interface {
	FindByAccountID(accountID string) ([]*entities.Notification, error)
}

// ======================== END MANAGE NOTIFICATION ========================
