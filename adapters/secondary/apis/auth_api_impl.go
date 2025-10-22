package apis

import "visionary-backend/domain/apis"

type authApiImpl struct{}

func NewAuthApi() apis.AuthApi {
	return &authApiImpl{}
}

func (api *authApiImpl) OAuthGoogle(idToken string, rawNonce string) (string, error) {
	return "IdToken", nil
}
