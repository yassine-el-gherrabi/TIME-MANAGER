package middleware

import (
	"net/http"
	"strings"

	"back/config"
	"back/database"
	"back/models"
	"back/utils"

	"github.com/gin-gonic/gin"
)

func AuthRequired(cfg *config.Config) gin.HandlerFunc {
	return func(c *gin.Context) {
		auth := c.GetHeader("Authorization")
		if auth == "" || !strings.HasPrefix(auth, "Bearer ") {
			c.AbortWithStatusJSON(http.StatusUnauthorized, gin.H{"error": "token manquant"})
			return
		}
		token := strings.TrimPrefix(auth, "Bearer ")
		claims, err := utils.ParseJWT(token, cfg.JWTSecret)
		if err != nil {
			c.AbortWithStatusJSON(http.StatusUnauthorized, gin.H{"error": "token invalide"})
			return
		}

		// Récupérer le rôle de l'utilisateur
		var user models.User
		if err := database.DB.First(&user, claims.UserID).Error; err != nil {
			c.AbortWithStatusJSON(http.StatusUnauthorized, gin.H{"error": "utilisateur non trouvé"})
			return
		}

		c.Set("uid", claims.UserID)
		c.Set("role", user.Role)
		c.Next()
	}
}

func AdminOnly() gin.HandlerFunc {
	return func(c *gin.Context) {
		role, exists := c.Get("role")
		if !exists {
			c.AbortWithStatusJSON(http.StatusForbidden, gin.H{"error": "accès refusé"})
			return
		}

		if role != models.RoleAdmin {
			c.AbortWithStatusJSON(http.StatusForbidden, gin.H{"error": "accès réservé aux administrateurs"})
			return
		}

		c.Next()
	}
}

func ManagerOrAdmin() gin.HandlerFunc {
	return func(c *gin.Context) {
		role, exists := c.Get("role")
		if !exists {
			c.AbortWithStatusJSON(http.StatusForbidden, gin.H{"error": "accès refusé"})
			return
		}

		userRole := role.(models.Role)
		if userRole != models.RoleAdmin && userRole != models.RoleManager {
			c.AbortWithStatusJSON(http.StatusForbidden, gin.H{"error": "accès réservé aux managers et administrateurs"})
			return
		}

		c.Next()
	}
}
