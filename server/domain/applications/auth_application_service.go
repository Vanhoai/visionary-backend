package applications

import "server/domain/services"

type AuthApplicationService struct {
	authService    services.AuthService    `inject:"AuthService"`
	accountService services.AccountService `inject:"AccountService"`
}
