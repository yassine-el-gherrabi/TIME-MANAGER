package config

import (
	"errors"
	"os"
	"path/filepath"
	"time"

	"github.com/joho/godotenv"
)

type Config struct {
	AppPort   string
	DBURL     string
	JWTSecret string
	JWTTTL    time.Duration
}

func Load() (*Config, error) {
	// Try to load .env file (for local development)
	// Ignore error if file not found (e.g., in Docker where env vars are provided)
	_ = loadDotenvFromBackRoot()

	cfg := &Config{
		AppPort:   getEnv("APP_PORT", "8080"),
		DBURL:     getEnv("DATABASE_URL", ""),
		JWTSecret: getEnv("JWT_SECRET", ""),
	}

	if cfg.DBURL == "" {
		return nil, errors.New("DATABASE_URL manquant")
	}
	if cfg.JWTSecret == "" {
		return nil, errors.New("JWT_SECRET manquant")
	}

	ttlStr := getEnv("JWT_TTL", "24h")
	d, err := time.ParseDuration(ttlStr)
	if err != nil {
		d = 24 * time.Hour
	}
	cfg.JWTTTL = d

	return cfg, nil
}

func getEnv(key, def string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return def
}

func loadDotenvFromBackRoot() error {
	wd, err := os.Getwd()
	if err != nil {
		return err
	}

	for filepath.Base(wd) != "back" {
		parent := filepath.Dir(wd)
		if parent == wd {
			return errors.New("dossier 'back' non trouvé")
		}
		wd = parent
	}

	envPath := filepath.Join(wd, ".env")
	if _, err := os.Stat(envPath); err != nil {
		return errors.New(".env non trouvé à la racine du dossier 'back'")
	}

	return godotenv.Load(envPath)
}
