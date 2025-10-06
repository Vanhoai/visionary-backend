package repositories

import (
	"context"
)

// Entity constraint - all entities must have these methods
type Entity interface {
	GetID() string
	SetID(id string)
}

// IBaseRepository defines common repository operations using generics
// T must implement the Entity interface
type IBaseRepository[T Entity] interface {
	// FindByID retrieves an entity by its ID
	FindByID(ctx context.Context, id string) (*T, error)

	// FindAll retrieves all entities with optional pagination
	FindAll(ctx context.Context, limit, offset int) ([]T, error)

	// Save creates or updates an entity
	Save(ctx context.Context, entity *T) error

	// Delete removes an entity by its ID
	Delete(ctx context.Context, id string) error

	// Count returns the total number of entities
	Count(ctx context.Context) (int64, error)

	// Exists checks if an entity exists by ID
	Exists(ctx context.Context, id string) (bool, error)
}
