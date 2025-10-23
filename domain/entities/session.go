package entities

type Session struct {
	Base
	AccountId    string `json:"account_id"`
	SessionToken string `json:"session_token"`
	ExpiresAt    int64  `json:"expires_at"`
	IpAddress    string `json:"ip_address"`
	UserAgent    string `json:"user_agent"`
	DeviceType   string `json:"device_type"`
}
