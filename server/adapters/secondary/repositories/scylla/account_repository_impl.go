package scylla

import (
	"context"
	"server/domain/entities"
	"server/domain/repositories"
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
