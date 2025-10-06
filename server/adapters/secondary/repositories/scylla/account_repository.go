package scylla

import (
	"app/domain/entities"
	"app/domain/repositories"
)

type AccountRepository struct {
	repositories.IBaseRepository[*entities.Account]
}
