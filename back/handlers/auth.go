package handlers

import (
	"net/http"
	"time"

	"github.com/gin-gonic/gin"

	"back/config"
	"back/models"
	"back/services"
	"back/utils"
)

var (
	authService = services.NewAuthService()
)

type registerReq struct {
	Email       string      `json:"email" binding:"required,email"`
	Password    string      `json:"password" binding:"required,min=8"`
	FirstName   string      `json:"first_name" binding:"required,min=2"`
	LastName    string      `json:"last_name" binding:"required,min=2"`
	PhoneNumber string      `json:"phone_number"`
	Role        models.Role `json:"role"`
	TeamID      *uint       `json:"team_id"`
}

type loginReq struct {
	Email    string `json:"email" binding:"required,email"`
	Password string `json:"password" binding:"required"`
}

type refreshReq struct {
	RefreshToken string `json:"refresh_token" binding:"required"`
}

func Register(c *gin.Context) {
	var body registerReq
	if err := c.ShouldBindJSON(&body); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	var createdByID *uint
	if uidVal, exists := c.Get("uid"); exists {
		uid := uidVal.(uint)
		createdByID = &uid
	}

	user, err := authService.Register(services.RegisterData{
		Email:       body.Email,
		Password:    body.Password,
		FirstName:   body.FirstName,
		LastName:    body.LastName,
		PhoneNumber: body.PhoneNumber,
		Role:        body.Role,
		CreatedByID: createdByID,
		TeamID:      body.TeamID,
	})

	if err != nil {
		if err.Error() == "rôle invalide" || err.Error() == "seul un admin peut créer des utilisateurs" {
			c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
			return
		}
		c.JSON(http.StatusConflict, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusCreated, gin.H{
		"user": gin.H{
			"id":         user.ID,
			"email":      user.Email,
			"first_name": user.FirstName,
			"last_name":  user.LastName,
			"role":       user.Role,
		},
	})
}

func Login(cfg *config.Config) gin.HandlerFunc {
	return func(c *gin.Context) {
		var body loginReq
		if err := c.ShouldBindJSON(&body); err != nil {
			c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
			return
		}

		user, err := authService.Login(body.Email, body.Password)
		if err != nil {
			c.JSON(http.StatusUnauthorized, gin.H{"error": err.Error()})
			return
		}

		// Generate access token
		token, err := utils.GenerateJWT(cfg.JWTSecret, cfg.JWTTTL, user.ID)
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": "erreur lors de la génération du token"})
			return
		}

		// Generate refresh token
		refreshToken, err := utils.GenerateRefreshToken(cfg.JWTSecret, cfg.RefreshTokenTTL, user.ID)
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": "erreur lors de la génération du refresh token"})
			return
		}

		// Store refresh token in database
		refreshExpiresAt := time.Now().Add(cfg.RefreshTokenTTL)
		if err := authService.StoreRefreshToken(user.ID, refreshToken, refreshExpiresAt); err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": "erreur lors du stockage du refresh token"})
			return
		}

		c.JSON(http.StatusOK, gin.H{
			"token":         token,
			"refresh_token": refreshToken,
			"user": gin.H{
				"id":         user.ID,
				"email":      user.Email,
				"first_name": user.FirstName,
				"last_name":  user.LastName,
				"role":       user.Role,
				"team_id":    user.TeamID,
			},
		})
	}
}

func Refresh(cfg *config.Config) gin.HandlerFunc {
	return func(c *gin.Context) {
		var body refreshReq
		if err := c.ShouldBindJSON(&body); err != nil {
			c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
			return
		}

		// Parse and validate refresh token
		claims, err := utils.ParseRefreshToken(body.RefreshToken, cfg.JWTSecret)
		if err != nil {
			c.JSON(http.StatusUnauthorized, gin.H{"error": "refresh token invalide ou expiré"})
			return
		}

		// Validate refresh token against database
		valid, err := authService.ValidateRefreshToken(claims.UserID, body.RefreshToken)
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": "erreur lors de la validation du refresh token"})
			return
		}
		if !valid {
			c.JSON(http.StatusUnauthorized, gin.H{"error": "refresh token invalide ou révoqué"})
			return
		}

		// Generate new access token
		newToken, err := utils.GenerateJWT(cfg.JWTSecret, cfg.JWTTTL, claims.UserID)
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": "erreur lors de la génération du token"})
			return
		}

		// Generate new refresh token (token rotation for security)
		newRefreshToken, err := utils.GenerateRefreshToken(cfg.JWTSecret, cfg.RefreshTokenTTL, claims.UserID)
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": "erreur lors de la génération du refresh token"})
			return
		}

		// Store new refresh token in database
		refreshExpiresAt := time.Now().Add(cfg.RefreshTokenTTL)
		if err := authService.StoreRefreshToken(claims.UserID, newRefreshToken, refreshExpiresAt); err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": "erreur lors du stockage du refresh token"})
			return
		}

		c.JSON(http.StatusOK, gin.H{
			"token":         newToken,
			"refresh_token": newRefreshToken,
		})
	}
}
