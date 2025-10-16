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

	if err := database.DB.AutoMigrate(&models.User{}, &models.Team{}); err != nil {
		log.Fatal(err)
	}

	r := gin.Default()

	r.GET("/health", func(c *gin.Context) { c.JSON(200, gin.H{"ok": true}) })

	r.POST("/register", handlers.Register)
	r.POST("/login", handlers.Login(func(uid uint) (string, error) {
		return utils.GenerateJWT(cfg.JWTSecret, cfg.JWTTTL, uid)
	}))

	protected := r.Group("/", middleware.AuthRequired(cfg))
	{
		protected.GET("/me", handlers.GetProfile)

		protected.GET("/users", handlers.GetUsers)
		protected.GET("/users/:id", handlers.GetUser)

		adminUsers := protected.Group("/users", middleware.AdminOnly())
		{
			adminUsers.POST("", handlers.Register)
			adminUsers.PUT("/:id", handlers.UpdateUser)
			adminUsers.DELETE("/:id", handlers.DeleteUser)
		}

		protected.GET("/teams", handlers.GetTeams)
		protected.GET("/teams/:id", handlers.GetTeam)

		adminTeams := protected.Group("/teams", middleware.AdminOnly())
		{
			adminTeams.POST("", handlers.CreateTeam)
			adminTeams.PUT("/:id", handlers.UpdateTeam)
			adminTeams.DELETE("/:id", handlers.DeleteTeam)
			adminTeams.POST("/:id/managers", handlers.AddManagerToTeam)
			adminTeams.DELETE("/:id/managers/:manager_id", handlers.RemoveManagerFromTeam)
		}
	}

	log.Printf("API sur : http://localhost:%s", cfg.AppPort)
	if err := r.Run(":" + cfg.AppPort); err != nil {
		log.Fatal(err)
	}
}
