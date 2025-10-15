package tests

import (
	"testing"

	"github.com/gin-gonic/gin"
	"github.com/stretchr/testify/require"
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"

	"back/database"
	"back/models"
)

func setupTestDB(t *testing.T) {
	db, err := gorm.Open(sqlite.Open(":memory:"), &gorm.Config{})
	require.NoError(t, err, "Erreur lors de la création de la DB de test")

	err = db.AutoMigrate(&models.User{}, &models.Team{})
	require.NoError(t, err, "Erreur lors de la migration")

	database.DB = db
}

func cleanupTestDB(t *testing.T) {
	err := database.DB.Exec("DELETE FROM manager_teams").Error
	require.NoError(t, err)

	err = database.DB.Exec("DELETE FROM users").Error
	require.NoError(t, err)

	err = database.DB.Exec("DELETE FROM teams").Error
	require.NoError(t, err)
}

func setupTestRouter() *gin.Engine {
	gin.SetMode(gin.TestMode)
	return gin.New()
}
