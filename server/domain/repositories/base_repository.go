package repositories

import (
	"context"
)

type Entity interface {
	GetID() string
	SetID(id string)
}

type BaseRepository[T Entity] interface {
	Save(ctx context.Context, entity *T) error
	FindByID(ctx context.Context, id string) (*T, error)
}
