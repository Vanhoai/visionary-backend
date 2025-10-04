package entities

// AccountEntity represents an account in the system
type AccountEntity struct {
	BaseEntity
	Name     string `json:"name"`
	Email    string `json:"email"`
	Password string `json:"password"`
}
