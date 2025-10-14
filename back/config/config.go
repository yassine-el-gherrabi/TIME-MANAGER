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
	_ = loadDotenvUpwards()

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

func loadDotenvUpwards() error {
	wd, _ := os.Getwd()
	dir := wd
	for i := 0; i < 6; i++ {
		p := filepath.Join(dir, ".env")
		if _, err := os.Stat(p); err == nil {
			return godotenv.Load(p)
		}
		parent := filepath.Dir(dir)
		if parent == dir {
			break
		}
		dir = parent
	}
	return nil
}
