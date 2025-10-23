package entities

type Account struct {
	Base
	Username      string `json:"username"`
	Avatar        string `json:"avatar"`
	Email         string `json:"email"`
	EmailVerified bool   `json:"email_verified"`
	Bio           string `json:"bio"`
	IsActive      bool   `json:"is_active"`
}
