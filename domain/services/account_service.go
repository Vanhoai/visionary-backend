package services

import "visionary-backend/domain/repositories"

type AccountService struct {
	repository *repositories.AccountRepository
}

func NewAccountService(repository *repositories.AccountRepository) *AccountService {
	return &AccountService{repository}
}
