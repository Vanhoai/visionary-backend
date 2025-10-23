package secure

import (
	"time"
	"visionary-backend/core/safe"

	"github.com/golang-jwt/jwt/v5"
)

type TokenType string

const (
	ACCESS_TOKEN  TokenType = "ACCESS_TOKEN"
	REFRESH_TOKEN TokenType = "REFRESH_TOKEN"
)

type Claims struct {
	jwt.RegisteredClaims
	AccountId string `json:"account_id"`
	Email     string `json:"email"`
}

type JwtSecure struct {
	cryptography    Cryptography
	accessDuration  time.Duration
	refreshDuration time.Duration
}

func NewJwt(cryptography Cryptography, accessDuration, refreshDuration time.Duration) *JwtSecure {
	return &JwtSecure{
		cryptography:    cryptography,
		accessDuration:  accessDuration,
		refreshDuration: refreshDuration,
	}
}

func (j *JwtSecure) GetDuration(tokenType TokenType) time.Duration {
	switch tokenType {
	case ACCESS_TOKEN:
		return j.accessDuration
	default:
		return j.refreshDuration
	}
}

func (j *JwtSecure) GenerateToken(accountId, email string, tokenType TokenType) (string, error) {
	return safe.Try(func() (string, error) {
		duration := j.GetDuration(tokenType)

		claims := &Claims{
			AccountId: accountId,
			Email:     email,
			RegisteredClaims: jwt.RegisteredClaims{
				ExpiresAt: jwt.NewNumericDate(time.Now().Add(duration)),
				IssuedAt:  jwt.NewNumericDate(time.Now()),
				NotBefore: jwt.NewNumericDate(time.Now()),
			},
		}

		encodeKey := safe.Must(j.cryptography.GetEncodeKey())
		token := jwt.NewWithClaims(jwt.SigningMethodHS512, claims)
		tokenString := safe.Must(token.SignedString(encodeKey))

		return tokenString, nil
	})
}

func (j *JwtSecure) DecodeToken(tokenString string) (*Claims, error) {
	return safe.Try(func() (*Claims, error) {
		decodeKey := safe.Must(j.cryptography.GetDecodeKey())
		token := safe.Must(jwt.ParseWithClaims(tokenString, &Claims{}, func(token *jwt.Token) (interface{}, error) {
			return decodeKey, nil
		}))

		if claims, ok := token.Claims.(*Claims); ok && token.Valid {
			return claims, nil
		} else {
			return nil, jwt.ErrTokenInvalidClaims
		}
	})
}
