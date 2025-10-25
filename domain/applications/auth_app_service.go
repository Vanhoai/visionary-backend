package applications

import (
	"fmt"
	"visionary-backend/core/https"
	"visionary-backend/domain/apis"
	"visionary-backend/domain/services"
	"visionary-backend/domain/usecases"
)

type AuthAppService struct {
	AuthService    *services.AuthService    `inject:"AuthService"`
	AccountService *services.AccountService `inject:"AccountService"`
	AuthApi        apis.AuthApi             `inject:"AuthApi"`
}

// =========================== ManageSessionUseCases ===========================

func (appService *AuthAppService) SignIn(req *usecases.EmailPasswordReq) (*usecases.AuthResponse, *https.AppError) {
	if req.Email != "vanhoai.adv@gmail.com" {
		fmt.Printf("Received sign-in attempt with email: %s\n", req.Email)
		return nil, https.NewBadRequestError("Invalid email or password", nil)
	}

	// Generate tokens
	token := "MIIEowIBAAKCAQEAt0bx90lnboVrcXTaNES8YQFlu+Vo8bOjaibl9a7yGRVCNPZ/3WzBt0VgjEfxxGEExx/egiHJpA6iP0t+Yjct7SZProPKB7iI9ByblbPXiTMbbqpuWvlRVzNdoJ3zR6j4+8JgiSEkBuc70IE7BRj2LO9qIbhQnPjc9arB+kTyVwCiNGyZpoQ5uer6+l5KjhB8D4ef9G9eIy9Cm+SwQy4p1yZW4sQGt0sgeswN1zGPhWiE+jFBFcrW/mKYeavf5Ur0PDCyj7ef6m2qTtFwt1OzEU1kiSTSWjAmwFbnttnFYgzxIxdL"

	return &usecases.AuthResponse{
		AccessToken:  token,
		RefreshToken: token,
	}, nil
}

func (appService *AuthAppService) RefreshToken(req *usecases.RefreshTokenReq) (*usecases.AuthResponse, error) {
	// Generate tokens
	token := "MIIEowIBAAKCAQEAt0bx90lnboVrcXTaNES8YQFlu+Vo8bOjaibl9a7yGRVCNPZ/3WzBt0VgjEfxxGEExx/egiHJpA6iP0t+Yjct7SZProPKB7iI9ByblbPXiTMbbqpuWvlRVzNdoJ3zR6j4+8JgiSEkBuc70IE7BRj2LO9qIbhQnPjc9arB+kTyVwCiNGyZpoQ5uer6+l5KjhB8D4ef9G9eIy9Cm+SwQy4p1yZW4sQGt0sgeswN1zGPhWiE+jFBFcrW/mKYeavf5Ur0PDCyj7ef6m2qTtFwt1OzEU1kiSTSWjAmwFbnttnFYgzxIxdL"

	return &usecases.AuthResponse{
		AccessToken:  token,
		RefreshToken: token,
	}, nil
}

// =========================== ManageSessionUseCases ===========================
