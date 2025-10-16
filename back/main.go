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

func main() {
	cfg, err := config.Load()
	if err != nil {
		log.Fatal(err)
	}

	if err := database.Connect(cfg.DBURL); err != nil {
		log.Fatal(err)
	}

	// Migration des tables
	if err := database.DB.AutoMigrate(&models.User{}, &models.Team{}); err != nil {
		log.Fatal(err)
	}

	r := gin.Default()

	// Route de santé - publique
	r.GET("/health", func(c *gin.Context) { c.JSON(200, gin.H{"ok": true}) })

	// Routes publiques - Premier admin peut s'inscrire sans authentification
	r.POST("/register", handlers.Register)
	r.POST("/login", handlers.Login(func(uid uint) (string, error) {
		return utils.GenerateJWT(cfg.JWTSecret, cfg.JWTTTL, uid)
	}))

	// Routes protégées - nécessitent authentification
	protected := r.Group("/", middleware.AuthRequired(cfg))
	{
		// Profile - accessible à tous les utilisateurs authentifiés
		protected.GET("/me", handlers.GetProfile)

		// ==================== ROUTES USERS ====================
		// Lecture : tous les rôles peuvent lire (avec filtres selon rôle)
		protected.GET("/users", handlers.GetUsers)
		protected.GET("/users/:id", handlers.GetUser)

		// Écriture : seuls les admins (via AdminOnly middleware)
		adminUsers := protected.Group("/users", middleware.AdminOnly())
		{
			adminUsers.POST("", handlers.Register)         // Admin créé des users
			adminUsers.PUT("/:id", handlers.UpdateUser)    // Admin modifie des users
			adminUsers.DELETE("/:id", handlers.DeleteUser) // Admin supprime des users
		}

		// ==================== ROUTES TEAMS ====================
		// Lecture : tous peuvent voir leurs teams (avec filtres selon rôle)
		protected.GET("/teams", handlers.GetTeams)
		protected.GET("/teams/:id", handlers.GetTeam)

		// Écriture : seuls les admins
		adminTeams := protected.Group("/teams", middleware.AdminOnly())
		{
			adminTeams.POST("", handlers.CreateTeam)                                       // Admin créé des teams
			adminTeams.PUT("/:id", handlers.UpdateTeam)                                    // Admin modifie des teams
			adminTeams.DELETE("/:id", handlers.DeleteTeam)                                 // Admin supprime des teams
			adminTeams.POST("/:id/managers", handlers.AddManagerToTeam)                    // Admin affecte un manager à une team
			adminTeams.DELETE("/:id/managers/:manager_id", handlers.RemoveManagerFromTeam) // Admin retire un manager d'une team
		}
	}

	log.Printf("API sur : http://localhost:%s", cfg.AppPort)
	if err := r.Run(":" + cfg.AppPort); err != nil {
		log.Fatal(err)
	}
}
