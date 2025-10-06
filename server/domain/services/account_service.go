package services

import "app/domain/repositories"

type AccountService struct {
	AccountRepository *repositories.IAccountRepository
}
