package entities

import "time"

// Base represents the base structure for all entities in the domain.
type Base struct {
	Id        string    `json:"id"`
	CreatedAt time.Time `json:"created_at"`
	UpdatedAt time.Time `json:"updated_at"`
	DeletedAt time.Time `json:"deleted_at"`
}

// GetID returns the unique identifier of the entity.
func (e *Base) GetID() string {
	return e.Id
}

// SetID sets the unique identifier of the entity.
func (e *Base) SetID(id string) {
	e.Id = id
}

// GetCreatedAt returns the creation timestamp of the entity.
func (e *Base) GetCreatedAt() time.Time {
	return e.CreatedAt
}

// SetCreatedAt sets the creation timestamp of the entity.
func (e *Base) SetCreatedAt(t time.Time) {
	e.CreatedAt = t
}

// GetUpdatedAt returns the last updated timestamp of the entity.
func (e *Base) GetUpdatedAt() time.Time {
	return e.UpdatedAt
}

// SetUpdatedAt sets the last updated timestamp of the entity.
func (e *Base) SetUpdatedAt(t time.Time) {
	e.UpdatedAt = t
}

// GetDeletedAt returns the deletion timestamp of the entity.
func (e *Base) GetDeletedAt() time.Time {
	return e.DeletedAt
}

// SetDeletedAt sets the deletion timestamp of the entity.
func (e *Base) SetDeletedAt(t time.Time) {
	e.DeletedAt = t
}
