// package tests

// import (
// 	"bytes"
// 	"encoding/json"
// 	"fmt"
// 	"net/http"
// 	"net/http/httptest"
// 	"testing"

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

// func createEmployeeUser(t *testing.T, adminID uint, teamID *uint) (*models.User, string) {
// 	service := services.NewAuthService()
// 	employee, err := service.Register(services.RegisterData{
// 		Email:       "employee@test.com",
// 		Password:    "password123",
// 		FirstName:   "Employee",
// 		LastName:    "User",
// 		Role:        models.RoleEmployee,
// 		CreatedByID: &adminID,
// 		TeamID:      teamID,
// 	})
// 	assert.NoError(t, err)

// 	cfg := &config.Config{
// 		JWTSecret: "test-secret",
// 		JWTTTL:    3600,
// 	}
// 	token, err := utils.GenerateJWT(cfg.JWTSecret, cfg.JWTTTL, employee.ID)
// 	assert.NoError(t, err)

// 	return employee, token
// }

// func getTestConfig() *config.Config {
// 	return &config.Config{
// 		JWTSecret: "test-secret",
// 		JWTTTL:    3600,
// 	}
// }

// func TestCreateTeam_Success(t *testing.T) {
// 	setupTestDB(t)
// 	defer cleanupTestDB(t)

// 	admin, token := createAdminUser(t)
// 	assert.NotNil(t, admin)

// 	cfg := getTestConfig()

// 	router := setupTestRouter()
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

// 	assert.Equal(t, http.StatusCreated, w.Code, "Response body: %s", w.Body.String())

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

// 	cfg := getTestConfig()

// 	router := setupTestRouter()
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

// 	assert.Equal(t, http.StatusForbidden, w.Code, "Response body: %s", w.Body.String())
// }

// func TestGetTeams_AdminSeesAll(t *testing.T) {
// 	setupTestDB(t)
// 	defer cleanupTestDB(t)

// 	admin, token := createAdminUser(t)

// 	// Créer 2 teams
// 	teamService := services.NewTeamService()
// 	team1, err := teamService.Create(services.CreateTeamData{
// 		Name:        "Team 1",
// 		Description: "First team",
// 		CreatedByID: admin.ID,
// 	})
// 	assert.NoError(t, err)

// 	team2, err := teamService.Create(services.CreateTeamData{
// 		Name:        "Team 2",
// 		Description: "Second team",
// 		CreatedByID: admin.ID,
// 	})
// 	assert.NoError(t, err)
// 	assert.NotNil(t, team1)
// 	assert.NotNil(t, team2)

// 	cfg := getTestConfig()

// 	router := setupTestRouter()
// 	protected := router.Group("/", middleware.AuthRequired(cfg))
// 	protected.GET("/teams", handlers.GetTeams)

// 	req := httptest.NewRequest(http.MethodGet, "/teams", nil)
// 	req.Header.Set("Authorization", "Bearer "+token)
// 	w := httptest.NewRecorder()

// 	router.ServeHTTP(w, req)

// 	assert.Equal(t, http.StatusOK, w.Code, "Response body: %s", w.Body.String())

// 	var teams []map[string]interface{}
// 	json.Unmarshal(w.Body.Bytes(), &teams)
// 	assert.Len(t, teams, 2)
// }

// func TestGetTeams_ManagerSeesOnlyHisTeams(t *testing.T) {
// 	setupTestDB(t)
// 	defer cleanupTestDB(t)

// 	admin, _ := createAdminUser(t)
// 	manager, managerToken := createManagerUser(t, admin.ID)

// 	// Créer 2 teams
// 	teamService := services.NewTeamService()
// 	team1, err := teamService.Create(services.CreateTeamData{
// 		Name:        "Team 1",
// 		Description: "Manager's team",
// 		CreatedByID: admin.ID,
// 	})
// 	assert.NoError(t, err)

// 	team2, err := teamService.Create(services.CreateTeamData{
// 		Name:        "Team 2",
// 		Description: "Other team",
// 		CreatedByID: admin.ID,
// 	})
// 	assert.NoError(t, err)

// 	// Affecter le manager à team1 seulement
// 	err = teamService.AddManager(team1.ID, manager.ID, models.RoleAdmin)
// 	assert.NoError(t, err)

// 	cfg := getTestConfig()

// 	router := setupTestRouter()
// 	protected := router.Group("/", middleware.AuthRequired(cfg))
// 	protected.GET("/teams", handlers.GetTeams)

// 	req := httptest.NewRequest(http.MethodGet, "/teams", nil)
// 	req.Header.Set("Authorization", "Bearer "+managerToken)
// 	w := httptest.NewRecorder()

// 	router.ServeHTTP(w, req)

// 	assert.Equal(t, http.StatusOK, w.Code, "Response body: %s", w.Body.String())

// 	var teams []map[string]interface{}
// 	json.Unmarshal(w.Body.Bytes(), &teams)

// 	// Manager devrait voir seulement team1
// 	if assert.Len(t, teams, 1, "Manager should see only 1 team") {
// 		assert.Equal(t, "Team 1", teams[0]["name"])
// 	}

// 	_ = team2
// }

// func TestGetTeams_EmployeeSeesOnlyHisTeam(t *testing.T) {
// 	setupTestDB(t)
// 	defer cleanupTestDB(t)

// 	admin, _ := createAdminUser(t)

// 	// Créer 2 teams
// 	teamService := services.NewTeamService()
// 	team1, err := teamService.Create(services.CreateTeamData{
// 		Name:        "Team 1",
// 		Description: "Employee's team",
// 		CreatedByID: admin.ID,
// 	})
// 	assert.NoError(t, err)

// 	team2, err := teamService.Create(services.CreateTeamData{
// 		Name:        "Team 2",
// 		Description: "Other team",
// 		CreatedByID: admin.ID,
// 	})
// 	assert.NoError(t, err)

// 	// Créer un employee dans team1
// 	employee, employeeToken := createEmployeeUser(t, admin.ID, &team1.ID)
// 	assert.NotNil(t, employee)

// 	cfg := getTestConfig()

// 	router := setupTestRouter()
// 	protected := router.Group("/", middleware.AuthRequired(cfg))
// 	protected.GET("/teams", handlers.GetTeams)

// 	req := httptest.NewRequest(http.MethodGet, "/teams", nil)
// 	req.Header.Set("Authorization", "Bearer "+employeeToken)
// 	w := httptest.NewRecorder()

// 	router.ServeHTTP(w, req)

// 	assert.Equal(t, http.StatusOK, w.Code, "Response body: %s", w.Body.String())

// 	var teams []map[string]interface{}
// 	json.Unmarshal(w.Body.Bytes(), &teams)

// 	// Employee devrait voir seulement team1
// 	if assert.Len(t, teams, 1, "Employee should see only 1 team") {
// 		assert.Equal(t, "Team 1", teams[0]["name"])
// 	}

// 	_ = team2
// }

// func TestAddManagerToTeam_Success(t *testing.T) {
// 	setupTestDB(t)
// 	defer cleanupTestDB(t)

// 	admin, token := createAdminUser(t)
// 	manager, _ := createManagerUser(t, admin.ID)

// 	// Créer une team
// 	teamService := services.NewTeamService()
// 	team, err := teamService.Create(services.CreateTeamData{
// 		Name:        "Team 1",
// 		Description: "Test team",
// 		CreatedByID: admin.ID,
// 	})
// 	assert.NoError(t, err)

// 	cfg := getTestConfig()

// 	router := setupTestRouter()
// 	protected := router.Group("/", middleware.AuthRequired(cfg))
// 	protected.POST("/teams/:id/managers", middleware.AdminOnly(), handlers.AddManagerToTeam)

// 	reqBody := map[string]uint{
// 		"manager_id": manager.ID,
// 	}

// 	body, _ := json.Marshal(reqBody)
// 	req := httptest.NewRequest(http.MethodPost, fmt.Sprintf("/teams/%d/managers", team.ID), bytes.NewBuffer(body))
// 	req.Header.Set("Content-Type", "application/json")
// 	req.Header.Set("Authorization", "Bearer "+token)
// 	w := httptest.NewRecorder()

// 	router.ServeHTTP(w, req)

// 	assert.Equal(t, http.StatusOK, w.Code, "Response body: %s", w.Body.String())
// }

// func TestDeleteTeam_AdminOnly(t *testing.T) {
// 	setupTestDB(t)
// 	defer cleanupTestDB(t)

// 	admin, adminToken := createAdminUser(t)
// 	_, managerToken := createManagerUser(t, admin.ID)

// 	// Créer une team
// 	teamService := services.NewTeamService()
// 	team, err := teamService.Create(services.CreateTeamData{
// 		Name:        "Team to delete",
// 		Description: "Test",
// 		CreatedByID: admin.ID,
// 	})
// 	assert.NoError(t, err)

// 	cfg := getTestConfig()

// 	router := setupTestRouter()
// 	protected := router.Group("/", middleware.AuthRequired(cfg))
// 	protected.DELETE("/teams/:id", middleware.AdminOnly(), handlers.DeleteTeam)

// 	// Manager essaie de supprimer (devrait échouer)
// 	req1 := httptest.NewRequest(http.MethodDelete, fmt.Sprintf("/teams/%d", team.ID), nil)
// 	req1.Header.Set("Authorization", "Bearer "+managerToken)
// 	w1 := httptest.NewRecorder()
// 	router.ServeHTTP(w1, req1)
// 	assert.Equal(t, http.StatusForbidden, w1.Code, "Manager should be forbidden")

// 	// Admin supprime (devrait réussir)
// 	req2 := httptest.NewRequest(http.MethodDelete, fmt.Sprintf("/teams/%d", team.ID), nil)
// 	req2.Header.Set("Authorization", "Bearer "+adminToken)
// 	w2 := httptest.NewRecorder()
// 	router.ServeHTTP(w2, req2)
// 	assert.Equal(t, http.StatusOK, w2.Code, "Admin should be able to delete: %s", w2.Body.String())
// }

// func TestUpdateTeam_AdminOnly(t *testing.T) {
// 	setupTestDB(t)
// 	defer cleanupTestDB(t)

// 	admin, adminToken := createAdminUser(t)

// 	// Créer une team
// 	teamService := services.NewTeamService()
// 	team, err := teamService.Create(services.CreateTeamData{
// 		Name:        "Old Name",
// 		Description: "Old description",
// 		CreatedByID: admin.ID,
// 	})
// 	assert.NoError(t, err)

// 	cfg := getTestConfig()

// 	router := setupTestRouter()
// 	protected := router.Group("/", middleware.AuthRequired(cfg))
// 	protected.PUT("/teams/:id", middleware.AdminOnly(), handlers.UpdateTeam)

// 	reqBody := map[string]string{
// 		"name":        "New Name",
// 		"description": "New description",
// 	}

// 	body, _ := json.Marshal(reqBody)
// 	req := httptest.NewRequest(http.MethodPut, fmt.Sprintf("/teams/%d", team.ID), bytes.NewBuffer(body))
// 	req.Header.Set("Content-Type", "application/json")
// 	req.Header.Set("Authorization", "Bearer "+adminToken)
// 	w := httptest.NewRecorder()

// 	router.ServeHTTP(w, req)

// 	assert.Equal(t, http.StatusOK, w.Code, "Response body: %s", w.Body.String())

// 	var response map[string]interface{}
// 	json.Unmarshal(w.Body.Bytes(), &response)
// 	assert.Equal(t, "New Name", response["name"])
// 	assert.Equal(t, "New description", response["description"])
// }
