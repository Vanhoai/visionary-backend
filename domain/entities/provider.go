package entities

type EnumProvider int8

const (
	PASSWORD EnumProvider = iota
	GOOGLE
	GITHUB
	FACE
)

var providerNames map[EnumProvider]string = map[EnumProvider]string{
	PASSWORD: "Password",
	GOOGLE:   "Google",
	GITHUB:   "GitHub",
	FACE:     "Face",
}

func (p EnumProvider) String() string { return providerNames[p] }

type Provider struct {
	Base
	AccountId  string `json:"account_id"`
	Provider   string `json:"provider"`
	Identifier string `json:"identifier"`
	Extra      string `json:"extra"`
}
