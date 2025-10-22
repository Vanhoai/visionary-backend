package services

import "visionary-backend/domain/apis"

type AuthService struct {
	authApi *apis.AuthApi
}

func NewAuthService(authApi *apis.AuthApi) *AuthService {
	return &AuthService{authApi: authApi}
}
