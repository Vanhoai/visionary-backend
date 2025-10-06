package usecases

import "app/domain/entities"

// ======================== MANAGE NOTIFICATION ========================
type ManageNotificationUseCases interface {
	FindByAccountID(accountID string) ([]*entities.Notification, error)
}

// ======================== END MANAGE NOTIFICATION ========================
