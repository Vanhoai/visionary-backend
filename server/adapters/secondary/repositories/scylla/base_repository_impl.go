package scylla

import (
	"context"
	"server/domain/repositories"
)

type baseRepositoryImpl[T repositories.Entity] struct{}

func (r *baseRepositoryImpl[T]) Save(ctx context.Context, entity *T) error {
	return nil
}

func (r *baseRepositoryImpl[T]) FindByID(ctx context.Context, id string) (*T, error) {
	return nil, nil
}
