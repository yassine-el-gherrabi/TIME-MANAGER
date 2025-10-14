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
	if err := database.DB.First(&user, id).Error; err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, errors.New("utilisateur non trouvé")
		}
		return nil, err
	}
	return &user, nil
}

func (s *UserService) GetAll() ([]models.User, error) {
	var users []models.User
	if err := database.DB.Find(&users).Error; err != nil {
		return nil, err
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
}

func (s *UserService) Update(id uint, data UpdateUserData) (*models.User, error) {
	user, err := s.GetByID(id)
	if err != nil {
		return nil, err
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

func (s *UserService) Delete(id uint) error {
	user, err := s.GetByID(id)
	if err != nil {
		return err
	}

	if err := database.DB.Delete(user).Error; err != nil {
		return err
	}

	return nil
}
