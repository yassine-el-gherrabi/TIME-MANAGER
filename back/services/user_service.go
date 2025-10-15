package services

import (
	"errors"

	"gorm.io/gorm"

	"back/database"
	"back/models"
	"back/utils"
)

type UserService struct{}

func NewUserService() *UserService {
	return &UserService{}
}

func (s *UserService) GetByID(id uint) (*models.User, error) {
	var user models.User
	if err := database.DB.Preload("Team").Preload("Teams").Preload("CreatedBy").First(&user, id).Error; err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, errors.New("utilisateur non trouvé")
		}
		return nil, err
	}
	return &user, nil
}

func (s *UserService) GetAll(currentUserID uint, currentUserRole models.Role) ([]models.User, error) {
	var users []models.User
	var currentUser models.User

	if err := database.DB.Preload("Teams").First(&currentUser, currentUserID).Error; err != nil {
		return nil, err
	}

	query := database.DB.Preload("Team").Preload("Teams").Preload("CreatedBy")

	switch currentUserRole {
	case models.RoleAdmin:
		if err := query.Find(&users).Error; err != nil {
			return nil, err
		}

	case models.RoleManager:
		teamIDs := make([]uint, len(currentUser.Teams))
		for i, team := range currentUser.Teams {
			teamIDs[i] = team.ID
		}
		if err := query.Where("role = ? AND team_id IN ?", models.RoleEmployee, teamIDs).
			Or("id = ?", currentUserID).
			Find(&users).Error; err != nil {
			return nil, err
		}

	case models.RoleEmployee:
		if currentUser.TeamID == nil {
			return []models.User{}, nil
		}
		if err := query.Where("role = ? AND team_id = ?", models.RoleEmployee, *currentUser.TeamID).
			Find(&users).Error; err != nil {
			return nil, err
		}

	default:
		return nil, errors.New("rôle invalide")
	}

	return users, nil
}

type UpdateUserData struct {
	FirstName   *string
	LastName    *string
	PhoneNumber *string
	Email       *string
	Password    *string
	Role        *models.Role
	TeamID      *uint
}

func (s *UserService) Update(id uint, data UpdateUserData, currentUserRole models.Role) (*models.User, error) {
	user, err := s.GetByID(id)
	if err != nil {
		return nil, err
	}

	if currentUserRole != models.RoleAdmin {
		return nil, errors.New("seul un admin peut modifier des utilisateurs")
	}

	if data.FirstName != nil {
		user.FirstName = *data.FirstName
	}
	if data.LastName != nil {
		user.LastName = *data.LastName
	}
	if data.PhoneNumber != nil {
		user.PhoneNumber = *data.PhoneNumber
	}
	if data.Email != nil {
		user.Email = *data.Email
	}
	if data.Role != nil {
		if *data.Role != models.RoleEmployee && *data.Role != models.RoleManager && *data.Role != models.RoleAdmin {
			return nil, errors.New("rôle invalide")
		}
		user.Role = *data.Role
	}
	if data.TeamID != nil {
		user.TeamID = data.TeamID
	}
	if data.Password != nil && *data.Password != "" {
		hash, err := utils.HashPassword(*data.Password)
		if err != nil {
			return nil, errors.New("erreur lors du hashage du mot de passe")
		}
		user.PasswordHash = hash
	}

	if err := database.DB.Save(user).Error; err != nil {
		return nil, err
	}

	return user, nil
}

func (s *UserService) Delete(id uint, currentUserRole models.Role) error {
	if currentUserRole != models.RoleAdmin {
		return errors.New("seul un admin peut supprimer des utilisateurs")
	}

	user, err := s.GetByID(id)
	if err != nil {
		return err
	}

	if err := database.DB.Delete(user).Error; err != nil {
		return err
	}

	return nil
}
