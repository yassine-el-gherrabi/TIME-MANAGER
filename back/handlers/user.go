package handlers

import (
	"net/http"

	"github.com/gin-gonic/gin"
	"back/database"
	"back/models"
)

func GetProfile(c *gin.Context) {
	uidVal, _ := c.Get("uid")
	uid := uidVal.(uint)

	var u models.User
	if err := database.DB.First(&u, uid).Error; err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "utilisateur non trouvé"})
		return
	}
	c.JSON(http.StatusOK, gin.H{
		"id":         u.ID,
		"email":      u.Email,
		"created_at": u.CreatedAt,
	})
}
