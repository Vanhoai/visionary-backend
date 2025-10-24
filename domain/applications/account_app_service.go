package applications

import "visionary-backend/domain/services"

type AccountAppService struct {
	accountService *services.AccountService `inject:"AccountService"`
}

func NewAccountAppService(accountService *services.AccountService) *AccountAppService {
	return &AccountAppService{accountService}
}
