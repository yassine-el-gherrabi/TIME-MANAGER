package handlers

import (
	"net/http"

	"github.com/gin-gonic/gin"

	"back/models"
	"back/services"
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

func Login(jwtGen func(userID uint) (string, error)) gin.HandlerFunc {
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

		token, err := jwtGen(user.ID)
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": "erreur lors de la génération du token"})
			return
		}

		c.JSON(http.StatusOK, gin.H{
			"token": token,
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
