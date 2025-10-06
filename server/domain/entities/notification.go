package entities

type Notification struct {
	Base
	AccountID string `json:"account_id"`
	Message   string `json:"message"`
	IsRead    bool   `json:"is_read"`
}
