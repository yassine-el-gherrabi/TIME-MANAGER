# CD Pipeline

> Déploiement continu vers Hetzner avec GitHub Actions

---

## Vue d'ensemble

```mermaid
graph LR
    subgraph Trigger["🎯 Trigger"]
        Push["Push to main/master"]
        Manual["workflow_dispatch"]
    end

    subgraph Build["🔨 Build"]
        Backend["Build Backend<br/><small>Docker image</small>"]
        Frontend["Build Frontend<br/><small>Docker image</small>"]
    end

    subgraph Registry["📦 Registry"]
        GHCR["GitHub Container Registry<br/><small>ghcr.io</small>"]
    end

    subgraph Deploy["🚀 Deploy"]
        SSH["SSH to Hetzner"]
        Pull["Docker pull"]
        Up["Docker compose up"]
    end

    Push --> Backend
    Push --> Frontend
    Manual --> Backend
    Manual --> Frontend
    Backend --> GHCR
    Frontend --> GHCR
    GHCR --> SSH
    SSH --> Pull
    Pull --> Up
```

---

## Pipeline détaillé

```mermaid
sequenceDiagram
    participant GH as GitHub
    participant GA as GitHub Actions
    participant GHCR as Container Registry
    participant H as Hetzner Server

    Note over GH,GA: Push to main/master

    GH->>GA: Trigger CD workflow

    par Build Images
        GA->>GA: Build Backend (Dockerfile.prod)
        GA->>GA: Build Frontend (Dockerfile.prod)
    end

    GA->>GHCR: Push backend:latest
    GA->>GHCR: Push frontend:latest

    Note over GA,H: Deploy job starts

    GA->>H: SCP docker-compose.prod.yml
    GA->>H: SCP infrastructure/
    GA->>H: SSH connect

    H->>H: Create .env file
    H->>GHCR: docker login
    H->>GHCR: docker compose pull
    H->>H: docker compose up -d
    H->>H: docker image prune

    H-->>GA: Deployment complete
```

---

## Jobs

### Build Backend

```mermaid
graph TD
    Checkout["📥 Checkout"] --> Buildx["🔧 Setup Buildx"]
    Buildx --> Login["🔐 Login GHCR"]
    Login --> Meta["📋 Extract metadata"]
    Meta --> Build["🔨 Build & Push"]

    Build --> Tags["Tags:<br/>- sha-abc123<br/>- latest"]
```

**Détails :**
- Context : `./backend`
- Dockerfile : `Dockerfile.prod`
- Cache : GitHub Actions cache (`type=gha`)
- Tags : SHA commit + `latest`

### Build Frontend

```mermaid
graph TD
    Checkout["📥 Checkout"] --> Buildx["🔧 Setup Buildx"]
    Buildx --> Login["🔐 Login GHCR"]
    Login --> Meta["📋 Extract metadata"]
    Meta --> Build["🔨 Build & Push"]

    Build --> Args["Build args:<br/>VITE_API_BASE_URL"]
```

**Build arg :**
```yaml
VITE_API_BASE_URL: https://time-manager.app/api
```

### Deploy

```mermaid
graph TD
    subgraph Copy["📂 Copy Files"]
        SCP["SCP to server"]
        Files["docker-compose.prod.yml<br/>infrastructure/"]
    end

    subgraph SSH["🔌 SSH Commands"]
        Env["Create .env"]
        Login["docker login ghcr.io"]
        Pull["docker compose pull"]
        Up["docker compose up -d"]
        Prune["docker image prune"]
    end

    Copy --> SSH
```

---

## Configuration Production

### Variables d'environnement

| Variable | Source | Description |
|----------|--------|-------------|
| `DOMAIN` | Hardcoded | time-manager.app |
| `POSTGRES_PASSWORD` | Secret | Mot de passe DB |
| `ACME_EMAIL` | Variable | Email Let's Encrypt |
| `SMTP_HOST` | Variable | Serveur SMTP |
| `SMTP_PASSWORD` | Secret | Password SMTP |
| `GRAFANA_PASSWORD` | Secret | Admin Grafana |

### Secrets GitHub

| Secret | Usage |
|--------|-------|
| `HETZNER_HOST` | IP du serveur |
| `HETZNER_SSH_KEY` | Clé SSH privée |
| `GHCR_TOKEN` | Token registry |
| `POSTGRES_PASSWORD` | Password PostgreSQL |
| `SMTP_PASSWORD` | Password SMTP |
| `GRAFANA_PASSWORD` | Password Grafana |

---

## Docker Images

### Tags générés

```
ghcr.io/yassine-el-gherrabi/time-manager/backend:latest
ghcr.io/yassine-el-gherrabi/time-manager/backend:abc123f

ghcr.io/yassine-el-gherrabi/time-manager/frontend:latest
ghcr.io/yassine-el-gherrabi/time-manager/frontend:abc123f
```

### Cache Docker

```yaml
cache-from: type=gha
cache-to: type=gha,mode=max
```

**Avantages :**
- Réutilisation des layers entre builds
- Temps de build réduit (~50%)
- Stocké dans GitHub Actions cache

---

## Infrastructure serveur

### Structure sur Hetzner

```
/opt/timemanager/
├── docker-compose.prod.yml
├── .env
└── infrastructure/
    ├── traefik/
    │   ├── traefik.yml
    │   └── dynamic.yml
    ├── prometheus/
    │   └── prometheus.yml
    ├── grafana/
    │   ├── provisioning/
    │   └── dashboards/
    └── ...
```

### Services déployés

```mermaid
graph TB
    subgraph Production["🏭 Production Stack"]
        Traefik["Traefik<br/><small>:80, :443</small>"]
        Backend["Backend<br/><small>:8080</small>"]
        Frontend["Frontend<br/><small>:3000</small>"]
        Postgres["PostgreSQL<br/><small>:5432</small>"]
    end

    subgraph Monitoring["📊 Monitoring Stack"]
        Prometheus["Prometheus"]
        Loki["Loki"]
        Tempo["Tempo"]
        Grafana["Grafana<br/><small>:3001</small>"]
    end

    Internet["🌐"] --> Traefik
    Traefik --> Backend
    Traefik --> Frontend
    Backend --> Postgres
    Backend -.-> Prometheus
    Prometheus --> Grafana
    Loki --> Grafana
    Tempo --> Grafana
```

---

## Rollback

### Procédure manuelle

```bash
# SSH sur le serveur
ssh root@<HETZNER_HOST>

# Lister les images disponibles
docker images | grep time-manager

# Revenir à une version précédente
docker compose -f docker-compose.prod.yml pull backend:sha-<commit>
docker compose -f docker-compose.prod.yml up -d
```

### Via GitHub Actions

1. Aller sur Actions > CD - Deploy to Hetzner
2. Click "Run workflow"
3. Sélectionner un commit précédent ou tag

---

## Health Checks

### Vérification post-déploiement

```bash
# Status des containers
docker compose -f docker-compose.prod.yml ps

# Logs backend
docker logs timemanager-backend --tail 100

# Health check API
curl https://time-manager.app/api/health
```

### Monitoring automatique

- **Prometheus** : Scrape `/metrics` toutes les 15s
- **Traefik** : Health checks configurés
- **Grafana** : Alertes sur métriques critiques

---

## Troubleshooting

### Erreur de build

```bash
# Vérifier les logs du workflow
gh run view <run-id> --log

# Build localement
docker build -f backend/Dockerfile.prod ./backend
```

### Erreur de déploiement

```bash
# SSH sur le serveur
ssh root@<HETZNER_HOST>

# Voir les logs
docker compose -f docker-compose.prod.yml logs -f

# Restart un service
docker compose -f docker-compose.prod.yml restart backend
```

### Container qui ne démarre pas

```bash
# Voir les events
docker events --filter container=timemanager-backend

# Inspecter le container
docker inspect timemanager-backend

# Logs détaillés
docker logs timemanager-backend --tail 200
```

---

## Liens connexes

- [CI Pipeline](./ci-pipeline.md)
- [Monitoring](./monitoring.md)
- [Infrastructure Docker](./docker.md)
