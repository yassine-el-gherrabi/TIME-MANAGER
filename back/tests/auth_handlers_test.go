// package tests

// import (
// 	"bytes"
// 	"encoding/json"
// 	"net/http"
// 	"net/http/httptest"
// 	"testing"

// 	"github.com/gin-gonic/gin"
// 	"github.com/stretchr/testify/assert"

// 	"back/config"
// 	"back/handlers"
// 	"back/middleware"
// 	"back/models"
// 	"back/services"
// 	"back/utils"
// )

// type createTeamReq struct {
// 	Name        string `json:"name"`
// 	Description string `json:"description"`
// }

// func createAdminUser(t *testing.T) (*models.User, string) {
// 	service := services.NewAuthService()
// 	admin, err := service.Register(services.RegisterData{
// 		Email:     "admin@test.com",
// 		Password:  "password123",
// 		FirstName: "Admin",
// 		LastName:  "User",
// 		Role:      models.RoleAdmin,
// 	})
// 	assert.NoError(t, err)

// 	cfg := &config.Config{
// 		JWTSecret: "test-secret",
// 		JWTTTL:    3600,
// 	}
// 	token, err := utils.GenerateJWT(cfg.JWTSecret, cfg.JWTTTL, admin.ID)
// 	assert.NoError(t, err)

// 	return admin, token
// }

// func createManagerUser(t *testing.T, adminID uint) (*models.User, string) {
// 	service := services.NewAuthService()
// 	manager, err := service.Register(services.RegisterData{
// 		Email:       "manager@test.com",
// 		Password:    "password123",
// 		FirstName:   "Manager",
// 		LastName:    "User",
// 		Role:        models.RoleManager,
// 		CreatedByID: &adminID,
// 	})
// 	assert.NoError(t, err)

// 	cfg := &config.Config{
// 		JWTSecret: "test-secret",
// 		JWTTTL:    3600,
// 	}
// 	token, err := utils.GenerateJWT(cfg.JWTSecret, cfg.JWTTTL, manager.ID)
// 	assert.NoError(t, err)

// 	return manager, token
// }

// func setupAuthRouter(cfg *config.Config) *gin.Engine {
// 	router := setupTestRouter()
// 	router.Use(func(c *gin.Context) {
// 		c.Set("config", cfg)
// 		c.Next()
// 	})
// 	return router
// }

// func TestCreateTeam_Success(t *testing.T) {
// 	setupTestDB(t)
// 	defer cleanupTestDB(t)

// 	admin, token := createAdminUser(t)
// 	assert.NotNil(t, admin)

// 	cfg := &config.Config{
// 		JWTSecret: "test-secret",
// 		JWTTTL:    3600,
// 	}

// 	router := setupAuthRouter(cfg)
// 	protected := router.Group("/", middleware.AuthRequired(cfg))
// 	protected.POST("/teams", middleware.AdminOnly(), handlers.CreateTeam)

// 	reqBody := createTeamReq{
// 		Name:        "Team Alpha",
// 		Description: "Test team",
// 	}

// 	body, _ := json.Marshal(reqBody)
// 	req := httptest.NewRequest(http.MethodPost, "/teams", bytes.NewBuffer(body))
// 	req.Header.Set("Content-Type", "application/json")
// 	req.Header.Set("Authorization", "Bearer "+token)
// 	w := httptest.NewRecorder()

// 	router.ServeHTTP(w, req)

// 	assert.Equal(t, http.StatusCreated, w.Code)

// 	var response map[string]interface{}
// 	json.Unmarshal(w.Body.Bytes(), &response)
// 	assert.Equal(t, "Team Alpha", response["name"])
// 	assert.Equal(t, "Test team", response["description"])
// 	assert.NotNil(t, response["id"])
// }

// func TestCreateTeam_ManagerForbidden(t *testing.T) {
// 	setupTestDB(t)
// 	defer cleanupTestDB(t)

// 	admin, _ := createAdminUser(t)
// 	_, managerToken := createManagerUser(t, admin.ID)

// 	cfg := &config.Config{
// 		JWTSecret: "test-secret",
// 		JWTTTL:    3600,
// 	}

// 	router := setupAuthRouter(cfg)
// 	protected := router.Group("/", middleware.AuthRequired(cfg))
// 	protected.POST("/teams", middleware.AdminOnly(), handlers.CreateTeam)

// 	reqBody := createTeamReq{
// 		Name:        "Team Hack",
// 		Description: "Should fail",
// 	}

// 	body, _ := json.Marshal(reqBody)
// 	req := httptest.NewRequest(http.MethodPost, "/teams", bytes.NewBuffer(body))
// 	req.Header.Set("Content-Type", "application/json")
// 	req.Header.Set("Authorization", "Bearer "+managerToken)
// 	w := httptest.NewRecorder()

// 	router.ServeHTTP(w, req)

// 	assert.Equal(t, http.StatusForbidden, w.Code)
// }

// func TestGetTeams_AdminSeesAll(t *testing.T) {
// 	setupTestDB(t)
// 	defer cleanupTestDB(t)

// 	admin, token := createAdminUser(t)

// 	teamService := services.NewTeamService()
// 	team1, _ := teamService.Create(services.CreateTeamData{
// 		Name:        "Team 1",
// 		Description: "First team",
// 		CreatedByID: admin.ID,
// 	})
// 	team2, _ := teamService.Create(services.CreateTeamData{
// 		Name:        "Team 2",
// 		Description: "Second team",
// 		CreatedByID: admin.ID,
// 	})
// 	assert.NotNil(t, team1)
// 	assert.NotNil(t, team2)

// 	cfg := &config.Config{
// 		JWTSecret: "test-secret",
// 		JWTTTL:    3600,
// 	}

// 	router := setupAuthRouter(cfg)
// 	protected := router.Group("/", middleware.AuthRequired(cfg))
// 	protected.GET("/teams", handlers.GetTeams)

// 	req := httptest.NewRequest(http.MethodGet, "/teams", nil)
// 	req.Header.Set("Authorization", "Bearer "+token)
// 	w := httptest.NewRecorder()

// 	router.ServeHTTP(w, req)

// 	assert.Equal(t, http.StatusOK, w.Code)

// 	var teams []map[string]interface{}
// 	json.Unmarshal(w.Body.Bytes(), &teams)
// 	assert.Len(t, teams, 2)
// }

// func TestGetTeams_ManagerSeesOnlyHisTeams(t *testing.T) {
// 	setupTestDB(t)
// 	defer cleanupTestDB(t)

// 	admin, _ := createAdminUser(t)
// 	manager, managerToken := createManagerUser(t, admin.ID)

// 	teamService := services.NewTeamService()
// 	team1, _ := teamService.Create(services.CreateTeamData{
// 		Name:        "Team 1",
// 		Description: "Manager's team",
// 		CreatedByID: admin.ID,
// 	})
// 	team2, _ := teamService.Create(services.CreateTeamData{
// 		Name:        "Team 2",
// 		Description: "Other team",
// 		CreatedByID: admin.ID,
// 	})

// 	teamService.AddManager(team1.ID, manager.ID, models.RoleAdmin)

// 	cfg := &config.Config{
// 		JWTSecret: "test-secret",
// 		JWTTTL:    3600,
// 	}

// 	router := setupAuthRouter(cfg)
// 	protected := router.Group("/", middleware.AuthRequired(cfg))
// 	protected.GET("/teams", handlers.GetTeams)

// 	req := httptest.NewRequest(http.MethodGet, "/teams", nil)
// 	req.Header.Set("Authorization", "Bearer "+managerToken)
// 	w := httptest.NewRecorder()

// 	router.ServeHTTP(w, req)

// 	assert.Equal(t, http.StatusOK, w.Code)

// 	var teams []map[string]interface{}
// 	json.Unmarshal(w.Body.Bytes(), &teams)

// 	assert.Len(t, teams, 1)
// 	assert.Equal(t, "Team 1", teams[0]["name"])

// 	_ = team2
// }

// func TestAddManagerToTeam_Success(t *testing.T) {
// 	setupTestDB(t)
// 	defer cleanupTestDB(t)

// 	admin, token := createAdminUser(t)
// 	manager, _ := createManagerUser(t, admin.ID)

// 	teamService := services.NewTeamService()
// 	team, _ := teamService.Create(services.CreateTeamData{
// 		Name:        "Team 1",
// 		Description: "Test team",
// 		CreatedByID: admin.ID,
// 	})

// 	cfg := &config.Config{
// 		JWTSecret: "test-secret",
// 		JWTTTL:    3600,
// 	}

// 	router := setupAuthRouter(cfg)
// 	protected := router.Group("/", middleware.AuthRequired(cfg))
// 	protected.POST("/teams/:id/managers", middleware.AdminOnly(), handlers.AddManagerToTeam)

// 	reqBody := map[string]uint{
// 		"manager_id": manager.ID,
// 	}

// 	body, _ := json.Marshal(reqBody)
// 	req := httptest.NewRequest(http.MethodPost, "/teams/"+string(rune(team.ID))+"/managers", bytes.NewBuffer(body))
// 	req.Header.Set("Content-Type", "application/json")
// 	req.Header.Set("Authorization", "Bearer "+token)
// 	w := httptest.NewRecorder()

// 	router.ServeHTTP(w, req)

// }

// func TestDeleteTeam_AdminOnly(t *testing.T) {
// 	setupTestDB(t)
// 	defer cleanupTestDB(t)

// 	admin, adminToken := createAdminUser(t)
// 	_, managerToken := createManagerUser(t, admin.ID)

// 	teamService := services.NewTeamService()
// 	team, _ := teamService.Create(services.CreateTeamData{
// 		Name:        "Team to delete",
// 		Description: "Test",
// 		CreatedByID: admin.ID,
// 	})

// 	cfg := &config.Config{
// 		JWTSecret: "test-secret",
// 		JWTTTL:    3600,
// 	}

// 	router := setupAuthRouter(cfg)
// 	protected := router.Group("/", middleware.AuthRequired(cfg))
// 	protected.DELETE("/teams/:id", middleware.AdminOnly(), handlers.DeleteTeam)

// 	req1 := httptest.NewRequest(http.MethodDelete, "/teams/"+string(rune(team.ID)), nil)
// 	req1.Header.Set("Authorization", "Bearer "+managerToken)
// 	w1 := httptest.NewRecorder()
// 	router.ServeHTTP(w1, req1)
// 	assert.Equal(t, http.StatusForbidden, w1.Code)

// 	req2 := httptest.NewRequest(http.MethodDelete, "/teams/"+string(rune(team.ID)), nil)
// 	req2.Header.Set("Authorization", "Bearer "+adminToken)
// 	w2 := httptest.NewRecorder()
// 	router.ServeHTTP(w2, req2)
// }
