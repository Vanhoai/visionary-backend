package entities

type Account struct {
	Base
	Name     string `json:"name"`
	Email    string `json:"email"`
	Password string `json:"password"`
}
