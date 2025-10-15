package handlers

import (
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"

	"back/models"
	"back/services"
)

var (
	userService = services.NewUserService()
)

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
		"team_id":      user.TeamID,
		"team":         user.Team,
		"teams":        user.Teams,
		"created_at":   user.CreatedAt,
	})
}

func GetUsers(c *gin.Context) {
	uidVal, _ := c.Get("uid")
	roleVal, _ := c.Get("role")
	uid := uidVal.(uint)
	role := roleVal.(models.Role)

	users, err := userService.GetAll(uid, role)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "erreur lors de la récupération des utilisateurs"})
		return
	}

	c.JSON(http.StatusOK, users)
}

func GetUser(c *gin.Context) {
	id, err := strconv.ParseUint(c.Param("id"), 10, 32)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "ID invalide"})
		return
	}

	roleVal, _ := c.Get("role")
	uidVal, _ := c.Get("uid")
	currentRole := roleVal.(models.Role)
	currentUID := uidVal.(uint)

	user, err := userService.GetByID(uint(id))
	if err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": err.Error()})
		return
	}

	switch currentRole {
	case models.RoleEmployee:
		currentUser, err := userService.GetByID(currentUID)
		if err != nil {
			c.JSON(http.StatusForbidden, gin.H{"error": "accès refusé"})
			return
		}
		if user.Role != models.RoleEmployee ||
			user.TeamID == nil ||
			currentUser.TeamID == nil ||
			*user.TeamID != *currentUser.TeamID {
			c.JSON(http.StatusForbidden, gin.H{"error": "accès refusé"})
			return
		}

	case models.RoleManager:
		if user.Role == models.RoleAdmin {
			c.JSON(http.StatusForbidden, gin.H{"error": "accès refusé"})
			return
		}
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
	TeamID      *uint       `json:"team_id"`
}

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
	if body.TeamID != nil {
		updateData.TeamID = body.TeamID
	}

	user, err := userService.Update(uint(id), updateData, currentUserRole)
	if err != nil {
		if err.Error() == "utilisateur non trouvé" {
			c.JSON(http.StatusNotFound, gin.H{"error": err.Error()})
			return
		}
		if err.Error() == "seul un admin peut modifier des utilisateurs" {
			c.JSON(http.StatusForbidden, gin.H{"error": err.Error()})
			return
		}
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
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

	if err := userService.Delete(uint(id), currentUserRole); err != nil {
		if err.Error() == "utilisateur non trouvé" {
			c.JSON(http.StatusNotFound, gin.H{"error": err.Error()})
			return
		}
		if err.Error() == "seul un admin peut supprimer des utilisateurs" {
			c.JSON(http.StatusForbidden, gin.H{"error": err.Error()})
			return
		}
		c.JSON(http.StatusInternalServerError, gin.H{"error": "erreur lors de la suppression"})
		return
	}

	c.JSON(http.StatusOK, gin.H{"message": "utilisateur supprimé avec succès"})
}
