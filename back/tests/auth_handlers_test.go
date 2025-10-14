package tests

import (
	"bytes"
	"encoding/json"
	"errors"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/stretchr/testify/assert"

	"back/handlers"
	"back/models"
	"back/services"
)

type registerReq struct {
	Email       string      `json:"email"`
	Password    string      `json:"password"`
	FirstName   string      `json:"first_name"`
	LastName    string      `json:"last_name"`
	PhoneNumber string      `json:"phone_number"`
	Role        models.Role `json:"role"`
}

type loginReq struct {
	Email    string `json:"email"`
	Password string `json:"password"`
}

func TestRegister_Success(t *testing.T) {
	setupTestDB(t)
	defer cleanupTestDB(t)

	router := setupTestRouter()
	router.POST("/register", handlers.Register)

	reqBody := registerReq{
		Email:       "test@test.test",
		Password:    "testpassword",
		FirstName:   "Carlo",
		LastName:    "Santos",
		PhoneNumber: "0123456789",
		Role:        models.RoleEmployee,
	}

	body, _ := json.Marshal(reqBody)
	req := httptest.NewRequest(http.MethodPost, "/register", bytes.NewBuffer(body))
	req.Header.Set("Content-Type", "application/json")
	w := httptest.NewRecorder()

	router.ServeHTTP(w, req)

	assert.Equal(t, http.StatusCreated, w.Code)

	var response map[string]interface{}
	if err := json.Unmarshal(w.Body.Bytes(), &response); err != nil {
		t.Fatalf("failed to unmarshal response: %v", err)
	}

	assert.Equal(t, reqBody.Email, response["email"])
	assert.Equal(t, reqBody.FirstName, response["first_name"])
	assert.Equal(t, reqBody.LastName, response["last_name"])
	assert.Equal(t, string(models.RoleEmployee), response["role"])
	assert.NotNil(t, response["id"])
}

func TestRegister_InvalidEmail(t *testing.T) {
	setupTestDB(t)
	defer cleanupTestDB(t)

	router := setupTestRouter()
	router.POST("/register", handlers.Register)

	reqBody := map[string]string{
		"email":      "invalid-email",
		"password":   "password123",
		"first_name": "Test",
		"last_name":  "User",
	}

	body, _ := json.Marshal(reqBody)
	req := httptest.NewRequest(http.MethodPost, "/register", bytes.NewBuffer(body))
	req.Header.Set("Content-Type", "application/json")
	w := httptest.NewRecorder()

	router.ServeHTTP(w, req)

	assert.Equal(t, http.StatusBadRequest, w.Code)
}

func TestRegister_ShortPassword(t *testing.T) {
	setupTestDB(t)
	defer cleanupTestDB(t)

	router := setupTestRouter()
	router.POST("/register", handlers.Register)

	reqBody := registerReq{
		Email:     "test@example.com",
		Password:  "short",
		FirstName: "Test",
		LastName:  "User",
	}

	body, _ := json.Marshal(reqBody)
	req := httptest.NewRequest(http.MethodPost, "/register", bytes.NewBuffer(body))
	req.Header.Set("Content-Type", "application/json")
	w := httptest.NewRecorder()

	router.ServeHTTP(w, req)

	assert.Equal(t, http.StatusBadRequest, w.Code)
}

func TestRegister_DuplicateEmail(t *testing.T) {
	setupTestDB(t)
	defer cleanupTestDB(t)

	router := setupTestRouter()
	router.POST("/register", handlers.Register)

	reqBody := registerReq{
		Email:     "duplicate@example.com",
		Password:  "password123",
		FirstName: "Test",
		LastName:  "User",
	}

	body, _ := json.Marshal(reqBody)
	req1 := httptest.NewRequest(http.MethodPost, "/register", bytes.NewBuffer(body))
	req1.Header.Set("Content-Type", "application/json")
	w1 := httptest.NewRecorder()
	router.ServeHTTP(w1, req1)
	assert.Equal(t, http.StatusCreated, w1.Code)

	body, _ = json.Marshal(reqBody)
	req2 := httptest.NewRequest(http.MethodPost, "/register", bytes.NewBuffer(body))
	req2.Header.Set("Content-Type", "application/json")
	w2 := httptest.NewRecorder()
	router.ServeHTTP(w2, req2)
	assert.Equal(t, http.StatusConflict, w2.Code)
}

func TestRegister_InvalidRole(t *testing.T) {
	setupTestDB(t)
	defer cleanupTestDB(t)

	router := setupTestRouter()
	router.POST("/register", handlers.Register)

	reqBody := registerReq{
		Email:     "test@example.com",
		Password:  "password123",
		FirstName: "Test",
		LastName:  "User",
		Role:      "invalid",
	}

	body, _ := json.Marshal(reqBody)
	req := httptest.NewRequest(http.MethodPost, "/register", bytes.NewBuffer(body))
	req.Header.Set("Content-Type", "application/json")
	w := httptest.NewRecorder()

	router.ServeHTTP(w, req)

	assert.Equal(t, http.StatusBadRequest, w.Code)
}

func TestLogin_Success(t *testing.T) {
	setupTestDB(t)
	defer cleanupTestDB(t)

	router := setupTestRouter()

	mockJWTGen := func(userID uint) (string, error) {
		return "mock-jwt-token", nil
	}

	router.POST("/login", handlers.Login(mockJWTGen))

	service := services.NewAuthService()
	if _, err := service.Register(services.RegisterData{
		Email:     "login@example.com",
		Password:  "password123",
		FirstName: "Test",
		LastName:  "User",
	}); err != nil {
		t.Fatalf("failed to seed user: %v", err)
	}

	reqBody := loginReq{
		Email:    "login@example.com",
		Password: "password123",
	}

	body, _ := json.Marshal(reqBody)
	req := httptest.NewRequest(http.MethodPost, "/login", bytes.NewBuffer(body))
	req.Header.Set("Content-Type", "application/json")
	w := httptest.NewRecorder()

	router.ServeHTTP(w, req)

	assert.Equal(t, http.StatusOK, w.Code)

	var response map[string]interface{}
	if err := json.Unmarshal(w.Body.Bytes(), &response); err != nil {
		t.Fatalf("failed to unmarshal response: %v", err)
	}

	assert.Equal(t, "mock-jwt-token", response["token"])
	assert.NotNil(t, response["user"])

	user := response["user"].(map[string]interface{})
	assert.Equal(t, "login@example.com", user["email"])
	assert.Equal(t, "Test", user["first_name"])
}

func TestLogin_WrongPassword(t *testing.T) {
	setupTestDB(t)
	defer cleanupTestDB(t)

	router := setupTestRouter()

	mockJWTGen := func(userID uint) (string, error) {
		return "mock-jwt-token", nil
	}

	router.POST("/login", handlers.Login(mockJWTGen))

	service := services.NewAuthService()
	if _, err := service.Register(services.RegisterData{
		Email:     "login@example.com",
		Password:  "password123",
		FirstName: "Test",
		LastName:  "User",
	}); err != nil {
		t.Fatalf("failed to seed user: %v", err)
	}

	reqBody := loginReq{
		Email:    "login@example.com",
		Password: "wrongpassword",
	}

	body, _ := json.Marshal(reqBody)
	req := httptest.NewRequest(http.MethodPost, "/login", bytes.NewBuffer(body))
	req.Header.Set("Content-Type", "application/json")
	w := httptest.NewRecorder()

	router.ServeHTTP(w, req)

	assert.Equal(t, http.StatusUnauthorized, w.Code)
}

func TestLogin_UserNotFound(t *testing.T) {
	setupTestDB(t)
	defer cleanupTestDB(t)

	router := setupTestRouter()

	mockJWTGen := func(userID uint) (string, error) {
		return "mock-jwt-token", nil
	}

	router.POST("/login", handlers.Login(mockJWTGen))

	reqBody := loginReq{
		Email:    "nonexistent@example.com",
		Password: "password123",
	}

	body, _ := json.Marshal(reqBody)
	req := httptest.NewRequest(http.MethodPost, "/login", bytes.NewBuffer(body))
	req.Header.Set("Content-Type", "application/json")
	w := httptest.NewRecorder()

	router.ServeHTTP(w, req)

	assert.Equal(t, http.StatusUnauthorized, w.Code)
}

func TestLogin_JWTGenerationError(t *testing.T) {
	setupTestDB(t)
	defer cleanupTestDB(t)

	router := setupTestRouter()

	mockJWTGen := func(userID uint) (string, error) {
		return "", errors.New("jwt generation failed")
	}

	router.POST("/login", handlers.Login(mockJWTGen))

	service := services.NewAuthService()
	if _, err := service.Register(services.RegisterData{
		Email:     "login@example.com",
		Password:  "password123",
		FirstName: "Test",
		LastName:  "User",
	}); err != nil {
		t.Fatalf("failed to seed user: %v", err)
	}

	reqBody := loginReq{
		Email:    "login@example.com",
		Password: "password123",
	}

	body, _ := json.Marshal(reqBody)
	req := httptest.NewRequest(http.MethodPost, "/login", bytes.NewBuffer(body))
	req.Header.Set("Content-Type", "application/json")
	w := httptest.NewRecorder()

	router.ServeHTTP(w, req)

	assert.Equal(t, http.StatusInternalServerError, w.Code)
}
