package services

import (
	"errors"
	"time"

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
	CreatedByID *uint
	TeamID      *uint
}

func (s *AuthService) Register(data RegisterData) (*models.User, error) {
	var adminCount int64
	database.DB.Model(&models.User{}).Where("role = ?", models.RoleAdmin).Count(&adminCount)

	if adminCount > 0 && data.CreatedByID == nil {
		return nil, errors.New("seul un admin peut créer des utilisateurs")
	}

	if data.Role == "" {
		data.Role = models.RoleEmployee
	}

	if data.Role != models.RoleEmployee && data.Role != models.RoleManager && data.Role != models.RoleAdmin {
		return nil, errors.New("rôle invalide")
	}

	if data.CreatedByID != nil && (data.Role == models.RoleAdmin || data.Role == models.RoleManager) {
		var creator models.User
		if err := database.DB.First(&creator, data.CreatedByID).Error; err != nil {
			return nil, errors.New("créateur non trouvé")
		}
		if creator.Role != models.RoleAdmin {
			return nil, errors.New("seul un admin peut créer des admins ou managers")
		}
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
		CreatedByID:  data.CreatedByID,
		TeamID:       data.TeamID,
	}

	if err := database.DB.Create(&user).Error; err != nil {
		return nil, errors.New("email déjà utilisé")
	}

	return &user, nil
}

func (s *AuthService) Login(email, password string) (*models.User, error) {
	var user models.User
	if err := database.DB.Preload("Team").Preload("Teams").First(&user, "email = ?", email).Error; err != nil {
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

// StoreRefreshToken stores a refresh token for a user in the database
func (s *AuthService) StoreRefreshToken(userID uint, token string, expiresAt time.Time) error {
	result := database.DB.Model(&models.User{}).
		Where("id = ?", userID).
		Updates(map[string]interface{}{
			"refresh_token":      token,
			"refresh_expires_at": expiresAt,
		})

	if result.Error != nil {
		return errors.New("erreur lors du stockage du refresh token")
	}

	if result.RowsAffected == 0 {
		return errors.New("utilisateur non trouvé")
	}

	return nil
}

// ValidateRefreshToken validates that a refresh token matches the stored token for a user
func (s *AuthService) ValidateRefreshToken(userID uint, token string) (bool, error) {
	var user models.User
	if err := database.DB.First(&user, userID).Error; err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return false, errors.New("utilisateur non trouvé")
		}
		return false, err
	}

	// Check if refresh token exists
	if user.RefreshToken == "" {
		return false, nil
	}

	// Check if refresh token matches
	if user.RefreshToken != token {
		return false, nil
	}

	// Check if refresh token is expired
	if user.RefreshExpiresAt == nil || time.Now().After(*user.RefreshExpiresAt) {
		return false, nil
	}

	return true, nil
}

// RevokeRefreshToken removes the refresh token for a user (e.g., on logout)
func (s *AuthService) RevokeRefreshToken(userID uint) error {
	result := database.DB.Model(&models.User{}).
		Where("id = ?", userID).
		Updates(map[string]interface{}{
			"refresh_token":      "",
			"refresh_expires_at": nil,
		})

	if result.Error != nil {
		return errors.New("erreur lors de la révocation du refresh token")
	}

	return nil
}
