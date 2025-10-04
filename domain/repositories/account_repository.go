package repositories

import (
	"context"
	"domain/entities"
)

// IAccountRepository extends base repository with account-specific operations
type IAccountRepository interface {
	// IBaseRepository[entities.AccountEntity]

	// Account-specific methods
	FindByEmail(ctx context.Context, email string) (*entities.AccountEntity, error)
	FindByName(ctx context.Context, name string) ([]entities.AccountEntity, error)
}
