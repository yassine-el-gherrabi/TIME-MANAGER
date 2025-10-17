package utils

import (
	"time"

	"github.com/golang-jwt/jwt/v5"
)

type Claims struct {
	UserID uint `json:"uid"`
	jwt.RegisteredClaims
}

func GenerateJWT(secret string, ttl time.Duration, userID uint) (string, error) {
	now := time.Now()
	claims := Claims{
		UserID: userID,
		RegisteredClaims: jwt.RegisteredClaims{
			IssuedAt:  jwt.NewNumericDate(now),
			ExpiresAt: jwt.NewNumericDate(now.Add(ttl)),
		},
	}
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	return token.SignedString([]byte(secret))
}

func ParseJWT(tokenStr, secret string) (*Claims, error) {
	token, err := jwt.ParseWithClaims(tokenStr, &Claims{}, func(t *jwt.Token) (interface{}, error) {
		return []byte(secret), nil
	})
	if err != nil {
		return nil, err
	}
	if c, ok := token.Claims.(*Claims); ok && token.Valid {
		return c, nil
	}
	return nil, jwt.ErrTokenInvalidClaims
}

// GenerateRefreshToken generates a refresh token with longer TTL
// Refresh tokens are used to obtain new access tokens without re-authentication
func GenerateRefreshToken(secret string, ttl time.Duration, userID uint) (string, error) {
	// Refresh tokens use the same structure as access tokens but with longer TTL
	return GenerateJWT(secret, ttl, userID)
}

// ParseRefreshToken parses and validates a refresh token
// Returns the claims if valid, error otherwise
func ParseRefreshToken(tokenStr, secret string) (*Claims, error) {
	// Refresh tokens use the same structure and validation as access tokens
	return ParseJWT(tokenStr, secret)
}
