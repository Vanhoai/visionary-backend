package usecases

// ============================ ManageSessionUseCases ============================
type EmailPasswordReq struct {
	Email    string `json:"email" validate:"required,email"`
	Password string `json:"password" validate:"required,min=8"`
}

type AuthResponse struct {
	AccessToken  string `json:"accessToken"`
	RefreshToken string `json:"refreshToken"`
}

type RefreshTokenReq struct {
	RefreshToken string `json:"refreshToken"`
}

type ManageSessionUseCases interface {
	SignIn(req *EmailPasswordReq) (*AuthResponse, error)
	RefreshToken(req *RefreshTokenReq) (*AuthResponse, error)
}

// ============================ ManageSessionUseCases ============================
