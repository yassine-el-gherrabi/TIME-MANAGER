package services

import (
	"errors"

	"gorm.io/gorm"

	"back/database"
	"back/models"
	"back/utils"
)

type AuthService struct{}

func NewAuthService() *AuthService {
	return &AuthService{}
}

type RegisterData struct {
	Email       string
	Password    string
	FirstName   string
	LastName    string
	PhoneNumber string
	Role        models.Role
}

func (s *AuthService) Register(data RegisterData) (*models.User, error) {
	if data.Role == "" {
		data.Role = models.RoleEmployee
	}
	if data.Role != models.RoleEmployee && data.Role != models.RoleManager && data.Role != models.RoleAdmin {
		return nil, errors.New("rôle invalide")
	}
	hash, err := utils.HashPassword(data.Password)
	if err != nil {
		return nil, errors.New("erreur lors du hashage du mot de passe")
	}

	user := models.User{
		Email:        data.Email,
		PasswordHash: hash,
		FirstName:    data.FirstName,
		LastName:     data.LastName,
		PhoneNumber:  data.PhoneNumber,
		Role:         data.Role,
	}

	if err := database.DB.Create(&user).Error; err != nil {
		return nil, errors.New("email déjà utilisé")
	}

	return &user, nil
}

func (s *AuthService) Login(email, password string) (*models.User, error) {
	var user models.User
	if err := database.DB.Where("email = ?", email).First(&user).Error; err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, errors.New("identifiants invalides")
		}
		return nil, err
	}

	if !utils.CheckPassword(user.PasswordHash, password) {
		return nil, errors.New("identifiants invalides")
	}

	return &user, nil
}
