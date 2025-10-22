package scylla

import (
	"context"
	"visionary-backend/domain/repositories"
)

type baseRepositoryImpl[T repositories.Entity] struct{}

func (r *baseRepositoryImpl[T]) Save(ctx context.Context, entity *T) error {
	return nil
}

func (r *baseRepositoryImpl[T]) FindByID(ctx context.Context, id string) (*T, error) {
	return nil, nil
}
