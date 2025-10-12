package apis

import "go-hexagonal-architecture/domain/apis"

type authApiImpl struct{}

func NewAuthApi() apis.AuthApi {
	return &authApiImpl{}
}

func (api *authApiImpl) OAuthGoogle(idToken string, rawNonce string) (string, error) {
	return "IdToken", nil
}
