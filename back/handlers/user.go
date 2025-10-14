package handlers

import (
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"

	"back/models"
	"back/services"
)

var userService = services.NewUserService()

func GetProfile(c *gin.Context) {
	uidVal, _ := c.Get("uid")
	uid := uidVal.(uint)

	user, err := userService.GetByID(uid)
	if err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{
		"id":           user.ID,
		"email":        user.Email,
		"first_name":   user.FirstName,
		"last_name":    user.LastName,
		"phone_number": user.PhoneNumber,
		"role":         user.Role,
		"created_at":   user.CreatedAt,
	})
}

// GetUsers - accessible par managers et admins
func GetUsers(c *gin.Context) {
	users, err := userService.GetAll()
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "erreur lors de la récupération des utilisateurs"})
		return
	}

	c.JSON(http.StatusOK, users)
}

// GetUser - accessible par managers et admins
func GetUser(c *gin.Context) {
	id, err := strconv.ParseUint(c.Param("id"), 10, 32)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "ID invalide"})
		return
	}

	user, err := userService.GetByID(uint(id))
	if err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, user)
}

type updateUserReq struct {
	FirstName   string      `json:"first_name"`
	LastName    string      `json:"last_name"`
	PhoneNumber string      `json:"phone_number"`
	Email       string      `json:"email" binding:"omitempty,email"`
	Password    string      `json:"password" binding:"omitempty,min=8"`
	Role        models.Role `json:"role"`
}

// UpdateUser - avec vérifications des permissions
func UpdateUser(c *gin.Context) {
	id, err := strconv.ParseUint(c.Param("id"), 10, 32)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "ID invalide"})
		return
	}

	roleVal, _ := c.Get("role")
	currentUserRole := roleVal.(models.Role)

	var body updateUserReq
	if err := c.ShouldBindJSON(&body); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	// Récupérer l'utilisateur cible
	targetUser, err := userService.GetByID(uint(id))
	if err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": err.Error()})
		return
	}

	// Vérifications des permissions
	// Un manager ne peut pas modifier un admin
	if currentUserRole == models.RoleManager && targetUser.Role == models.RoleAdmin {
		c.JSON(http.StatusForbidden, gin.H{"error": "un manager ne peut pas modifier un administrateur"})
		return
	}

	// Un manager ne peut pas promouvoir quelqu'un admin
	if currentUserRole == models.RoleManager && body.Role == models.RoleAdmin {
		c.JSON(http.StatusForbidden, gin.H{"error": "un manager ne peut pas promouvoir quelqu'un administrateur"})
		return
	}

	updateData := services.UpdateUserData{}
	if body.FirstName != "" {
		updateData.FirstName = &body.FirstName
	}
	if body.LastName != "" {
		updateData.LastName = &body.LastName
	}
	if body.PhoneNumber != "" {
		updateData.PhoneNumber = &body.PhoneNumber
	}
	if body.Email != "" {
		updateData.Email = &body.Email
	}
	if body.Password != "" {
		updateData.Password = &body.Password
	}
	if body.Role != "" {
		updateData.Role = &body.Role
	}

	user, err := userService.Update(uint(id), updateData)
	if err != nil {
		if err.Error() == "utilisateur non trouvé" {
			c.JSON(http.StatusNotFound, gin.H{"error": err.Error()})
			return
		}
		if err.Error() == "rôle invalide" {
			c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
			return
		}
		c.JSON(http.StatusInternalServerError, gin.H{"error": "erreur lors de la mise à jour"})
		return
	}

	c.JSON(http.StatusOK, user)
}

func DeleteUser(c *gin.Context) {
	id, err := strconv.ParseUint(c.Param("id"), 10, 32)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "ID invalide"})
		return
	}

	roleVal, _ := c.Get("role")
	currentUserRole := roleVal.(models.Role)

	targetUser, err := userService.GetByID(uint(id))
	if err != nil {
		if err.Error() == "utilisateur non trouvé" {
			c.JSON(http.StatusNotFound, gin.H{"error": err.Error()})
			return
		}
		c.JSON(http.StatusInternalServerError, gin.H{"error": "erreur lors de la récupération de l'utilisateur"})
		return
	}

	if currentUserRole == models.RoleManager && targetUser.Role == models.RoleAdmin {
		c.JSON(http.StatusForbidden, gin.H{"error": "un manager ne peut pas supprimer un administrateur"})
		return
	}

	if err := userService.Delete(uint(id)); err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "erreur lors de la suppression"})
		return
	}

	c.JSON(http.StatusOK, gin.H{"message": "utilisateur supprimé avec succès"})
}
