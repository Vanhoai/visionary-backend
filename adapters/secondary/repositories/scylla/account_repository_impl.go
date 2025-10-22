package scylla

import (
	"context"
	"visionary-backend/domain/entities"
	"visionary-backend/domain/repositories"
)

type accountRepositoryImpl struct {
	baseRepositoryImpl[*entities.Account]
}

func NewAccountRepository() repositories.AccountRepository {
	return &accountRepositoryImpl{}
}

func (r *accountRepositoryImpl) FindByEmail(ctx context.Context, email string) (*entities.Account, error) {
	return nil, nil
}

func (r *accountRepositoryImpl) FindByName(ctx context.Context, name string) ([]*entities.Account, error) {
	return nil, nil
}

type AccountRepositoryImpl struct {
	baseRepositoryImpl[*entities.Account]
}

func (r *AccountRepositoryImpl) FindByEmail(ctx context.Context, email string) (*entities.Account, error) {
	return nil, nil
}

func (r *AccountRepositoryImpl) FindByName(ctx context.Context, name string) ([]*entities.Account, error) {
	return nil, nil
}
