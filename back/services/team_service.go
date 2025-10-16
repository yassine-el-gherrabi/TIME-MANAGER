package services

import (
	"errors"

	"gorm.io/gorm"

	"back/database"
	"back/models"
)

type TeamService struct{}

func NewTeamService() *TeamService {
	return &TeamService{}
}

type CreateTeamData struct {
	Name        string
	Description string
	CreatedByID uint
}

func (s *TeamService) Create(data CreateTeamData) (*models.Team, error) {
	// Vérifier que le créateur est bien un admin
	var creator models.User
	if err := database.DB.First(&creator, data.CreatedByID).Error; err != nil {
		return nil, errors.New("créateur non trouvé")
	}
	if creator.Role != models.RoleAdmin {
		return nil, errors.New("seul un admin peut créer des teams")
	}

	team := models.Team{
		Name:        data.Name,
		Description: data.Description,
		CreatedByID: data.CreatedByID,
	}

	if err := database.DB.Create(&team).Error; err != nil {
		return nil, err
	}

	return &team, nil
}

func (s *TeamService) GetByID(id uint) (*models.Team, error) {
	var team models.Team
	if err := database.DB.Preload("Employees").Preload("Managers").Preload("CreatedBy").First(&team, id).Error; err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, errors.New("team non trouvée")
		}
		return nil, err
	}
	return &team, nil
}

func (s *TeamService) GetAll(currentUserID uint, currentUserRole models.Role) ([]models.Team, error) {
	var teams []models.Team

	query := database.DB.Preload("Employees").Preload("Managers").Preload("CreatedBy")

	switch currentUserRole {
	case models.RoleAdmin:
		if err := query.Find(&teams).Error; err != nil {
			return nil, err
		}

	case models.RoleManager:
		if err := query.Joins("JOIN manager_teams ON manager_teams.team_id = teams.id").
			Where("manager_teams.user_id = ?", currentUserID).
			Find(&teams).Error; err != nil {
			return nil, err
		}

	case models.RoleEmployee:
		var user models.User
		if err := database.DB.First(&user, currentUserID).Error; err != nil {
			return nil, err
		}
		if user.TeamID != nil {
			var team models.Team
			if err := query.First(&team, *user.TeamID).Error; err != nil {
				return nil, err
			}
			teams = []models.Team{team}
		}

	default:
		return nil, errors.New("rôle invalide")
	}

	return teams, nil
}

type UpdateTeamData struct {
	Name        *string
	Description *string
}

func (s *TeamService) Update(id uint, data UpdateTeamData, currentUserRole models.Role) (*models.Team, error) {
	if currentUserRole != models.RoleAdmin {
		return nil, errors.New("seul un admin peut modifier des teams")
	}

	team, err := s.GetByID(id)
	if err != nil {
		return nil, err
	}

	if data.Name != nil {
		team.Name = *data.Name
	}
	if data.Description != nil {
		team.Description = *data.Description
	}

	if err := database.DB.Save(team).Error; err != nil {
		return nil, err
	}

	return team, nil
}

func (s *TeamService) Delete(id uint, currentUserRole models.Role) error {
	if currentUserRole != models.RoleAdmin {
		return errors.New("seul un admin peut supprimer des teams")
	}

	team, err := s.GetByID(id)
	if err != nil {
		return err
	}

	if err := database.DB.Delete(team).Error; err != nil {
		return err
	}

	return nil
}

func (s *TeamService) AddManager(teamID, managerID uint, currentUserRole models.Role) error {
	if currentUserRole != models.RoleAdmin {
		return errors.New("seul un admin peut affecter des managers")
	}

	var manager models.User
	if err := database.DB.First(&manager, managerID).Error; err != nil {
		return errors.New("manager non trouvé")
	}
	if manager.Role != models.RoleManager {
		return errors.New("l'utilisateur n'est pas un manager")
	}

	var team models.Team
	if err := database.DB.First(&team, teamID).Error; err != nil {
		return errors.New("team non trouvée")
	}

	if err := database.DB.Model(&team).Association("Managers").Append(&manager); err != nil {
		return err
	}

	return nil
}

func (s *TeamService) RemoveManager(teamID, managerID uint, currentUserRole models.Role) error {
	if currentUserRole != models.RoleAdmin {
		return errors.New("seul un admin peut retirer des managers")
	}

	var manager models.User
	if err := database.DB.First(&manager, managerID).Error; err != nil {
		return errors.New("manager non trouvé")
	}

	var team models.Team
	if err := database.DB.First(&team, teamID).Error; err != nil {
		return errors.New("team non trouvée")
	}

	if err := database.DB.Model(&team).Association("Managers").Delete(&manager); err != nil {
		return err
	}

	return nil
}
