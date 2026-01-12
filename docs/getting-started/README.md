# Getting Started

> Démarrage rapide avec Time Manager

---

## Prérequis

| Outil | Version | Installation |
|-------|---------|--------------|
| Docker | 24+ | [docs.docker.com](https://docs.docker.com/get-docker/) |
| Docker Compose | v2+ | Inclus avec Docker Desktop |
| Git | 2.x | [git-scm.com](https://git-scm.com/) |

---

## Quickstart (5 minutes)

```bash
# 1. Cloner le repo
git clone <repository-url>
cd time-manager

# 2. Copier l'environnement
cp .env.dev.example .env

# 3. Lancer les services
docker compose --profile dev up -d

# 4. Attendre le démarrage (~30s)
docker compose logs -f backend

# 5. Accéder à l'app
open http://localhost:8000
```

---

## URLs locales

| Service | URL | Description |
|---------|-----|-------------|
| Application | http://localhost:8000 | Frontend + API |
| Traefik Dashboard | http://localhost:8081 | Monitoring proxy |
| Mailpit | http://localhost:8025 | Emails de dev |
| pgAdmin | http://localhost:5050 | Administration DB |
| Grafana | http://localhost:3001 | Dashboards (monitoring) |

---

## Comptes de test

| Role | Email | Password |
|------|-------|----------|
| Super Admin | `superadmin@timemanager.local` | `SuperAdmin123!` |
| Admin | `admin@acme.local` | `Admin123!` |
| Manager | `manager@acme.local` | `Manager123!` |
| Employee | `employee@acme.local` | `Employee123!` |

> ⚠️ Ces comptes sont créés par le seeder uniquement en mode développement.

---

## Documentation

| Section | Description |
|---------|-------------|
| [Installation](./installation.md) | Configuration détaillée |
| [Configuration](./configuration.md) | Variables d'environnement |
| [Quickstart](./quickstart.md) | Guide pas à pas |

---

## Prochaines étapes

1. **Explorer l'API** → [Swagger Documentation](../api/)
2. **Comprendre l'architecture** → [Architecture](../architecture/)
3. **Configurer le monitoring** → [Monitoring](../devops/monitoring.md)
