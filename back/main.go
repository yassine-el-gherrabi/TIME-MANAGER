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

	if err := database.DB.AutoMigrate(&models.User{}); err != nil {
		log.Fatal(err)
	}

	r := gin.Default()

	r.GET("/health", func(c *gin.Context) { c.JSON(200, gin.H{"ok": true}) })

	r.POST("/register", handlers.Register)
	r.POST("/login", handlers.Login(func(uid uint) (string, error) {
		return utils.GenerateJWT(cfg.JWTSecret, cfg.JWTTTL, uid)
	}))

	protected := r.Group("/", middleware.AuthRequired(cfg))

	protected.GET("/me", handlers.GetProfile)

	users := protected.Group("/users")
	users.GET("", handlers.GetUsers)
	users.GET("/:id", handlers.GetUser)
	users.PUT("/:id", handlers.UpdateUser)
	users.DELETE("/:id", handlers.DeleteUser)

	log.Printf("API sur : http://localhost:%s", cfg.AppPort)
	if err := r.Run(":" + cfg.AppPort); err != nil {
		log.Fatal(err)
	}
}
