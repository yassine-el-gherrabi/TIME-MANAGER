package main

import (
	"log"

	"back/config"
	"back/database"
	"back/handlers"
	"back/middleware"
	"back/models"
	"back/utils"

	"github.com/gin-gonic/gin"
)

// #
func main() {
	cfg, err := config.Load()
	if err != nil {
		log.Fatal(err)
	}

	if err := database.Connect(cfg.DBURL); err != nil {
		log.Fatal(err)
	}

	if err := database.DB.AutoMigrate(&models.User{}); err != nil {
		log.Fatal(err)
	}

	r := gin.Default()

	// Route de santé - publique
	r.GET("/health", func(c *gin.Context) { c.JSON(200, gin.H{"ok": true}) })

	// Routes publiques (pas d'authentification nécessaire)
	r.POST("/register", handlers.Register)
	r.POST("/login", handlers.Login(func(uid uint) (string, error) {
		return utils.GenerateJWT(cfg.JWTSecret, cfg.JWTTTL, uid)
	}))

	// Routes protégées - nécessitent authentification
	protected := r.Group("/", middleware.AuthRequired(cfg))
	{
		// Profile - accessible à tous les utilisateurs authentifiés (employee, manager, admin)
		protected.GET("/me", handlers.GetProfile)

		// Routes /users - nécessitent d'être manager ou admin
		users := protected.Group("/users", middleware.ManagerOrAdmin())
		{
			users.GET("", handlers.GetUsers)          // Liste tous les utilisateurs
			users.GET("/:id", handlers.GetUser)       // Voir un utilisateur
			users.PUT("/:id", handlers.UpdateUser)    // Modifier un utilisateur
			users.DELETE("/:id", handlers.DeleteUser) // Supprimer un utilisateur
		}
	}

	log.Printf("API sur : http://localhost:%s", cfg.AppPort)
	if err := r.Run(":" + cfg.AppPort); err != nil {
		log.Fatal(err)
	}
}
