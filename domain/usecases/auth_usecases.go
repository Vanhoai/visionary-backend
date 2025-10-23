package usecases

type EmailPasswordReq struct {
	AuthType string `json:"authType"`
	Email    string `json:"email"`
	Password string `json:"password"`
}

type AuthResponse struct {
	AccessToken  string `json:"accessToken"`
	RefreshToken string `json:"refreshToken"`
}

type RefreshTokenReq struct {
	RefreshToken string `json:"refreshToken"`
}

type ManageSessionUseCases interface {
	SignInWithEmail(req *EmailPasswordReq) (*AuthResponse, error)
	SignUpWithEmail(req *EmailPasswordReq) (*AuthResponse, error)
	RefreshToken(req *RefreshTokenReq) (*AuthResponse, error)
}
