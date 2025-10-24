package scylla

import (
	"context"
	"visionary-backend/domain/entities"
)

type AccountRepositoryImpl struct {
	baseRepositoryImpl[*entities.Account]
	Database *ScyllaDatabase `inject:"ScyllaDatabase"`
}

func (r *AccountRepositoryImpl) FindByEmail(ctx context.Context, email string) (*entities.Account, error) {
	return nil, nil
}

func (r *AccountRepositoryImpl) FindByName(ctx context.Context, name string) ([]*entities.Account, error) {
	return nil, nil
}
