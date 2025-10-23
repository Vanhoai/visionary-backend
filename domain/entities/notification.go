package entities

type Notification struct {
	Base
	AccountId string `json:"account_id"`
	Message   string `json:"message"`
	IsRead    bool   `json:"is_read"`
}
