package handlers

import (
	"back/models"
	"back/services"
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"
)

var (
	teamService = services.NewTeamService()
)

type createTeamReq struct {
	Name        string `json:"name" binding:"required,min=2"`
	Description string `json:"description"`
}

func CreateTeam(c *gin.Context) {
	uidVal, _ := c.Get("uid")
	uid := uidVal.(uint)

	var body createTeamReq
	if err := c.ShouldBindJSON(&body); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	team, err := teamService.Create(services.CreateTeamData{
		Name:        body.Name,
		Description: body.Description,
		CreatedByID: uid,
	})

	if err != nil {
		if err.Error() == "seul un admin peut créer des teams" {
			c.JSON(http.StatusForbidden, gin.H{"error": err.Error()})
			return
		}
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusCreated, team)
}

func GetTeams(c *gin.Context) {
	uidVal, _ := c.Get("uid")
	roleVal, _ := c.Get("role")
	uid := uidVal.(uint)
	role := roleVal.(models.Role)

	teams, err := teamService.GetAll(uid, role)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "erreur lors de la récupération des teams"})
		return
	}

	c.JSON(http.StatusOK, teams)
}

func GetTeam(c *gin.Context) {
	id, err := strconv.ParseUint(c.Param("id"), 10, 32)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "ID invalide"})
		return
	}

	team, err := teamService.GetByID(uint(id))
	if err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, team)
}

type updateTeamReq struct {
	Name        string `json:"name"`
	Description string `json:"description"`
}

func UpdateTeam(c *gin.Context) {
	id, err := strconv.ParseUint(c.Param("id"), 10, 32)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "ID invalide"})
		return
	}

	roleVal, _ := c.Get("role")
	currentUserRole := roleVal.(models.Role)

	var body updateTeamReq
	if err := c.ShouldBindJSON(&body); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	updateData := services.UpdateTeamData{}
	if body.Name != "" {
		updateData.Name = &body.Name
	}
	if body.Description != "" {
		updateData.Description = &body.Description
	}

	team, err := teamService.Update(uint(id), updateData, currentUserRole)
	if err != nil {
		if err.Error() == "team non trouvée" {
			c.JSON(http.StatusNotFound, gin.H{"error": err.Error()})
			return
		}
		if err.Error() == "seul un admin peut modifier des teams" {
			c.JSON(http.StatusForbidden, gin.H{"error": err.Error()})
			return
		}
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, team)
}

func DeleteTeam(c *gin.Context) {
	id, err := strconv.ParseUint(c.Param("id"), 10, 32)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "ID invalide"})
		return
	}

	roleVal, _ := c.Get("role")
	currentUserRole := roleVal.(models.Role)

	if err := teamService.Delete(uint(id), currentUserRole); err != nil {
		if err.Error() == "team non trouvée" {
			c.JSON(http.StatusNotFound, gin.H{"error": err.Error()})
			return
		}
		if err.Error() == "seul un admin peut supprimer des teams" {
			c.JSON(http.StatusForbidden, gin.H{"error": err.Error()})
			return
		}
		c.JSON(http.StatusInternalServerError, gin.H{"error": "erreur lors de la suppression"})
		return
	}

	c.JSON(http.StatusOK, gin.H{"message": "team supprimée avec succès"})
}

type addManagerToTeamReq struct {
	ManagerID uint `json:"manager_id" binding:"required"`
}

func AddManagerToTeam(c *gin.Context) {
	teamID, err := strconv.ParseUint(c.Param("id"), 10, 32)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "ID invalide"})
		return
	}

	roleVal, _ := c.Get("role")
	currentUserRole := roleVal.(models.Role)

	var body addManagerToTeamReq
	if err := c.ShouldBindJSON(&body); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	if err := teamService.AddManager(uint(teamID), body.ManagerID, currentUserRole); err != nil {
		if err.Error() == "seul un admin peut affecter des managers" {
			c.JSON(http.StatusForbidden, gin.H{"error": err.Error()})
			return
		}
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"message": "manager ajouté à la team"})
}

func RemoveManagerFromTeam(c *gin.Context) {
	teamID, err := strconv.ParseUint(c.Param("id"), 10, 32)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "ID invalide"})
		return
	}

	managerID, err := strconv.ParseUint(c.Param("manager_id"), 10, 32)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "ID manager invalide"})
		return
	}

	roleVal, _ := c.Get("role")
	currentUserRole := roleVal.(models.Role)

	if err := teamService.RemoveManager(uint(teamID), uint(managerID), currentUserRole); err != nil {
		if err.Error() == "seul un admin peut retirer des managers" {
			c.JSON(http.StatusForbidden, gin.H{"error": err.Error()})
			return
		}
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"message": "manager retiré de la team"})
}
