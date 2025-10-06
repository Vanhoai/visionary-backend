package postgresql

import (
	"app/domain/repositories"
	"context"
	"fmt"
)

type BaseRepository[T repositories.Entity] struct {
	tableName string
}

// NewBaseRepository create a new base repository
func NewBaseRepository[T repositories.Entity](tableName string) *BaseRepository[T] {
	return &BaseRepository[T]{
		tableName: tableName,
	}
}

// FindByID retrieves an entity by its ID
func (r *BaseRepository[T]) FindByID(ctx context.Context, id string) (*T, error) {
	return nil, fmt.Errorf("please implement for this function")
}
