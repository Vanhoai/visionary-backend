package entities

type EnumProvider int8

const (
	GOOGLE EnumProvider = iota
	GITHUB
	EMAIL
	FACE
)

var providerNames map[EnumProvider]string = map[EnumProvider]string{
	GOOGLE: "Google",
	GITHUB: "GitHub",
	EMAIL:  "Email",
	FACE:   "Face",
}

func (p EnumProvider) String() string { return providerNames[p] }

type Provider struct {
	Base
	Provider   string `json:"provider"`
	AccountID  string `json:"account_id"`
	Identifier string `json:"identifier"`
	Extra      string `json:"extra"`
}

func NewProvider(provider EnumProvider, accountID, identifier, extra string) *Provider {
	return &Provider{
		Provider:   provider.String(),
		AccountID:  accountID,
		Identifier: identifier,
		Extra:      extra,
	}
}
