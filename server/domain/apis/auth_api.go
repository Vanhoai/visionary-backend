package apis

type AuthApi interface {
	OAuthGoogle(idToken string, rawNonce string) (string, error)
}
