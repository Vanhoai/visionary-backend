package applications

import "go-hexagonal-architecture/domain/services"

type AuthApplicationService struct {
	authService    *services.AuthService    `inject:"AuthService"`
	accountService *services.AccountService `inject:"AccountService"`
}

func NewAuthApplicationService(authService *services.AuthService, accountService *services.AccountService) *AuthApplicationService {
	return &AuthApplicationService{
		authService:    authService,
		accountService: accountService,
	}
}
