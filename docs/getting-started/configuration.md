# Configuration

> Variables d'environnement et configuration de Time Manager

---

## Variables d'environnement

### Application

| Variable | Défaut | Description |
|----------|--------|-------------|
| `APP_HOST` | `0.0.0.0` | Host de binding |
| `APP_PORT` | `8080` | Port interne backend |
| `RUST_LOG` | `info` | Niveau de log |

### Base de données

| Variable | Défaut | Description |
|----------|--------|-------------|
| `DATABASE_URL` | - | URL PostgreSQL complète |
| `POSTGRES_PASSWORD` | - | Mot de passe DB |

### JWT

| Variable | Défaut | Description |
|----------|--------|-------------|
| `JWT_KEYS_PATH` | `/app/keys` | Chemin des clés RSA |
| `JWT_ACCESS_TOKEN_EXPIRY_SECONDS` | `900` | Durée access token (15 min) |
| `JWT_REFRESH_TOKEN_EXPIRY_SECONDS` | `604800` | Durée refresh token (7 jours) |

### CORS

| Variable | Défaut | Description |
|----------|--------|-------------|
| `CORS_ALLOWED_ORIGINS` | - | Origines autorisées (comma-sep) |

### Email

| Variable | Défaut | Description |
|----------|--------|-------------|
| `EMAIL_ENABLED` | `false` | Activer l'envoi d'emails |
| `SMTP_HOST` | `mailpit` | Serveur SMTP |
| `SMTP_PORT` | `1025` | Port SMTP |
| `SMTP_USERNAME` | - | Username SMTP |
| `SMTP_PASSWORD` | - | Password SMTP |
| `EMAIL_FROM` | `noreply@timemanager.local` | Expéditeur |
| `EMAIL_FROM_NAME` | `Time Manager` | Nom expéditeur |
| `FRONTEND_URL` | - | URL frontend (pour les liens) |

### Frontend

| Variable | Défaut | Description |
|----------|--------|-------------|
| `VITE_API_BASE_URL` | `/api` | URL de base de l'API |

### Monitoring

| Variable | Défaut | Description |
|----------|--------|-------------|
| `GRAFANA_USER` | `admin` | Username Grafana |
| `GRAFANA_PASSWORD` | `admin` | Password Grafana |

---

## Fichier .env exemple

### Développement

```env
# === Application ===
APP_HOST=0.0.0.0
APP_PORT=8080
RUST_LOG=debug

# === Database ===
DATABASE_URL=postgres://timemanager:devpassword@postgres:5432/timemanager
POSTGRES_PASSWORD=devpassword

# === JWT ===
JWT_ACCESS_TOKEN_EXPIRY_SECONDS=900
JWT_REFRESH_TOKEN_EXPIRY_SECONDS=604800

# === CORS ===
CORS_ALLOWED_ORIGINS=http://localhost:8000,http://localhost:5173

# === Email (Mailpit) ===
EMAIL_ENABLED=true
SMTP_HOST=mailpit
SMTP_PORT=1025
EMAIL_FROM=noreply@timemanager.local
EMAIL_FROM_NAME=Time Manager
FRONTEND_URL=http://localhost:8000

# === Frontend ===
VITE_API_BASE_URL=/api
```

### Production

```env
# === Application ===
APP_HOST=0.0.0.0
APP_PORT=8080
RUST_LOG=info

# === Database ===
DATABASE_URL=postgres://timemanager:${POSTGRES_PASSWORD}@postgres:5432/timemanager
POSTGRES_PASSWORD=<strong-password>

# === JWT ===
JWT_ACCESS_TOKEN_EXPIRY_SECONDS=900
JWT_REFRESH_TOKEN_EXPIRY_SECONDS=604800

# === CORS ===
CORS_ALLOWED_ORIGINS=https://time-manager.app

# === Email (Production SMTP) ===
EMAIL_ENABLED=true
SMTP_HOST=smtp.resend.com
SMTP_PORT=465
SMTP_USERNAME=resend
SMTP_PASSWORD=<api-key>
EMAIL_FROM=noreply@time-manager.app
EMAIL_FROM_NAME=Time Manager
FRONTEND_URL=https://time-manager.app

# === Frontend ===
VITE_API_BASE_URL=https://time-manager.app/api

# === Monitoring ===
GRAFANA_USER=admin
GRAFANA_PASSWORD=<strong-password>
```

---

## Configuration par service

### Traefik

```yaml
# infrastructure/traefik/traefik.yml
api:
  dashboard: true

entryPoints:
  web:
    address: ":80"
  websecure:
    address: ":443"

providers:
  docker:
    exposedByDefault: false
  file:
    filename: /etc/traefik/dynamic.yml
```

### Prometheus

```yaml
# infrastructure/prometheus/prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'backend'
    static_configs:
      - targets: ['backend:8080']
    metrics_path: '/metrics'

  - job_name: 'traefik'
    static_configs:
      - targets: ['traefik:8082']

  - job_name: 'cadvisor'
    static_configs:
      - targets: ['cadvisor:8080']
```

### Grafana

```yaml
# infrastructure/grafana/provisioning/datasources/datasources.yml
datasources:
  - name: Prometheus
    type: prometheus
    url: http://prometheus:9090

  - name: Loki
    type: loki
    url: http://loki:3100

  - name: Tempo
    type: tempo
    url: http://tempo:3200
```

---

## Niveaux de log

| Niveau | Description | Usage |
|--------|-------------|-------|
| `error` | Erreurs uniquement | Production critique |
| `warn` | Warnings + errors | Production standard |
| `info` | Info + warn + error | Production verbose |
| `debug` | Debug détaillé | Développement |
| `trace` | Très détaillé | Debugging avancé |

### Configuration par module

```env
# Logs détaillés pour un module spécifique
RUST_LOG=info,time_manager::api=debug,diesel=warn
```

---

## Profils Docker Compose

| Profil | Services | Usage |
|--------|----------|-------|
| (aucun) | postgres, backend, frontend, traefik | Core uniquement |
| `dev` | + mailpit, pgadmin | Développement |
| `monitoring` | + prometheus, loki, tempo, grafana, cadvisor, promtail | Observabilité |

```bash
# Core uniquement
docker compose up -d

# Dev
docker compose --profile dev up -d

# Dev + Monitoring
docker compose --profile dev --profile monitoring up -d
```

---

## Secrets en production

### Recommandations

1. **Ne jamais commiter** les fichiers `.env` de production
2. Utiliser **Docker secrets** ou **Vault**
3. Rotation régulière des credentials
4. Monitoring des accès aux secrets

### Variables sensibles

| Variable | Stockage recommandé |
|----------|---------------------|
| `POSTGRES_PASSWORD` | Secret manager |
| `SMTP_PASSWORD` | Secret manager |
| `GRAFANA_PASSWORD` | Secret manager |
| Clés JWT | Fichiers montés |

---

## Liens connexes

- [Installation](./installation.md)
- [Docker](../devops/docker.md)
- [Monitoring](../devops/monitoring.md)
