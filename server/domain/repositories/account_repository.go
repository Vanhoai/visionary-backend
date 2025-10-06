package repositories

import (
	"context"
	"server/domain/entities"
)

// IAccountRepository extends base repository with account-specific operations
type IAccountRepository interface {
	IBaseRepository[*entities.Account]

	FindByEmail(ctx context.Context, email string) (*entities.Account, error)
	FindByName(ctx context.Context, name string) ([]*entities.Account, error)
}
