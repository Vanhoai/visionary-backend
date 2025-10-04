package entities

import "time"

// BaseEntity represents the base structure for all entities in the domain.
type BaseEntity struct {
	ID        string    `json:"id"`
	CreatedAt time.Time `json:"created_at"`
	UpdatedAt time.Time `json:"updated_at"`
}

// GetID returns the unique identifier of the entity.
func (e *BaseEntity) GetID() string {
	return e.ID
}

// SetID sets the unique identifier of the entity.
func (e *BaseEntity) SetID(id string) {
	e.ID = id
}

// GetCreatedAt returns the creation timestamp of the entity.
func (e *BaseEntity) GetCreatedAt() time.Time {
	return e.CreatedAt
}

// SetCreatedAt sets the creation timestamp of the entity.
func (e *BaseEntity) SetCreatedAt(t time.Time) {
	e.CreatedAt = t
}

// GetUpdatedAt returns the last updated timestamp of the entity.
func (e *BaseEntity) GetUpdatedAt() time.Time {
	return e.UpdatedAt
}

// SetUpdatedAt sets the last updated timestamp of the entity.
func (e *BaseEntity) SetUpdatedAt(t time.Time) {
	e.UpdatedAt = t
}
