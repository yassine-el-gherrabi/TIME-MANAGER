# TM-E14 : Documentation & Déploiement

## Informations Epic

| Champ | Valeur |
|-------|--------|
| **ID** | TM-E14 |
| **Titre** | Documentation & Déploiement |
| **Priorité** | P3 - Basse |
| **Estimation globale** | 13 SP |
| **Sprint cible** | Sprint 6 (finalisation) |
| **Dépendances** | Toutes les autres Epics |

---

## Description

### Contexte

La documentation et le déploiement sont les étapes finales avant livraison. Une bonne documentation permet aux développeurs futurs de reprendre le projet, et aux utilisateurs de comprendre l'application. Le déploiement doit être automatisé et reproductible.

### Objectif Business

Assurer la pérennité du projet avec une documentation complète et un processus de déploiement fiable, permettant une mise en production sans intervention manuelle complexe.

### Valeur Apportée

- **Pour les développeurs** : Onboarding rapide, maintenance facilitée
- **Pour les utilisateurs** : Guide d'utilisation clair
- **Pour l'équipe projet** : Déploiement en confiance
- **Pour Epitech** : Critères documentation et déploiement validés

---

## Scope

### Inclus

- Documentation technique (README, API, architecture)
- Documentation utilisateur (guide de démarrage)
- Déploiement Docker Compose (production-ready)
- Scripts de déploiement automatisés
- Healthchecks et monitoring basique

### Exclus

- Documentation vidéo/tutoriels
- Déploiement Kubernetes
- Multi-environnement (staging)
- Blue-green deployment
- Rollback automatique

---

## Critères de Succès de l'Epic

- [ ] README complet avec instructions de setup
- [ ] Documentation API générée (OpenAPI/Swagger)
- [ ] Guide utilisateur avec screenshots
- [ ] `docker-compose up` démarre l'application en production
- [ ] Healthchecks fonctionnels pour tous les services
- [ ] Variables d'environnement documentées

---

## User Stories

---

### TM-95 : README principal

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur qui rejoint le projet,
**Je veux** un README clair et complet,
**Afin de** comprendre et lancer le projet rapidement.

#### Critères d'Acceptation

- [ ] Description du projet (objectif, stack technique)
- [ ] Prérequis (Docker, Node, Rust, etc.)
- [ ] Instructions d'installation pas à pas
- [ ] Commandes pour lancer en développement
- [ ] Commandes pour lancer les tests
- [ ] Structure du projet expliquée
- [ ] Liens vers documentation détaillée
- [ ] Badges CI, couverture, version

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-95.1 | Rédiger section présentation | 0.5h |
| TM-95.2 | Rédiger section installation | 1h |
| TM-95.3 | Rédiger section développement | 1h |
| TM-95.4 | Ajouter badges et liens | 0.5h |
| TM-95.5 | Relecture et validation | 0.5h |

---

### TM-96 : Documentation API

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur frontend ou intégrateur,
**Je veux** une documentation API complète,
**Afin de** comprendre les endpoints disponibles.

#### Contexte Détaillé

Génération automatique via OpenAPI (Swagger) :
- Description de chaque endpoint
- Paramètres requis et optionnels
- Exemples de requêtes et réponses
- Codes d'erreur possibles
- Authentification requise

#### Critères d'Acceptation

- [ ] Spec OpenAPI générée depuis le code (utoipa)
- [ ] UI Swagger accessible en développement
- [ ] Documentation de tous les endpoints
- [ ] Exemples pour les cas principaux
- [ ] Export JSON/YAML de la spec
- [ ] Versionning de l'API documenté

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-96.1 | Configurer utoipa pour génération OpenAPI | 1h |
| TM-96.2 | Annoter tous les handlers | 2h |
| TM-96.3 | Configurer Swagger UI | 0.5h |
| TM-96.4 | Vérifier exhaustivité | 1h |
| TM-96.5 | Documenter authentification | 0.5h |

---

### TM-97 : Documentation architecture

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur ou architecte,
**Je veux** une documentation de l'architecture,
**Afin de** comprendre les choix techniques et la structure.

#### Critères d'Acceptation

- [ ] Diagramme d'architecture système
- [ ] Schéma de la base de données
- [ ] Description des composants backend
- [ ] Description des composants frontend
- [ ] Flux d'authentification
- [ ] Décisions d'architecture (ADR style)
- [ ] Stack technique justifiée

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-97.1 | Créer diagramme architecture (Mermaid) | 1h |
| TM-97.2 | Créer schéma BDD | 1h |
| TM-97.3 | Documenter composants backend | 1h |
| TM-97.4 | Documenter composants frontend | 1h |
| TM-97.5 | Rédiger décisions architecture | 1h |

---

### TM-98 : Guide utilisateur

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P2 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur final,
**Je veux** un guide d'utilisation,
**Afin de** comprendre comment utiliser l'application.

#### Critères d'Acceptation

- [ ] Guide de démarrage (première connexion)
- [ ] Guide par rôle (employee, manager, admin)
- [ ] Screenshots des écrans principaux
- [ ] FAQ des questions courantes
- [ ] Format : Markdown dans /docs ou wiki GitHub

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-98.1 | Rédiger guide démarrage | 1h |
| TM-98.2 | Rédiger guide employee | 1h |
| TM-98.3 | Rédiger guide manager | 1h |
| TM-98.4 | Rédiger guide admin | 1h |
| TM-98.5 | Capturer screenshots | 1h |

---

### TM-99 : Docker Compose production

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant qu'** opérateur de déploiement,
**Je veux** un docker-compose de production,
**Afin de** déployer l'application sur un serveur.

#### Contexte Détaillé

Différences avec le compose de dev :
- Images buildées (pas de volumes source)
- Variables d'environnement via .env
- Healthchecks sur tous les services
- Restart policies
- Volumes persistants pour BDD et Grafana
- Réseau isolé

#### Critères d'Acceptation

- [ ] `docker-compose.prod.yml` créé
- [ ] Build multi-stage pour images optimisées
- [ ] Variables d'environnement externalisées
- [ ] Healthchecks sur backend, frontend, PostgreSQL
- [ ] Restart: unless-stopped
- [ ] Volumes nommés pour persistance
- [ ] Documentation des variables requises

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-99.1 | Créer docker-compose.prod.yml | 2h |
| TM-99.2 | Optimiser Dockerfiles (multi-stage) | 2h |
| TM-99.3 | Configurer healthchecks | 1h |
| TM-99.4 | Créer .env.example complet | 0.5h |
| TM-99.5 | Tester déploiement complet | 1.5h |
| TM-99.6 | Documenter procédure | 1h |

---

### TM-100 : Scripts de déploiement

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** opérateur de déploiement,
**Je veux** des scripts automatisés,
**Afin de** déployer et mettre à jour facilement.

#### Critères d'Acceptation

- [ ] Script `deploy.sh` pour premier déploiement
- [ ] Script `update.sh` pour mise à jour
- [ ] Script `backup.sh` pour sauvegarde BDD
- [ ] Script `restore.sh` pour restauration
- [ ] Gestion des migrations automatique
- [ ] Vérification des prérequis
- [ ] Logs d'exécution

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-100.1 | Créer script deploy.sh | 1.5h |
| TM-100.2 | Créer script update.sh | 1h |
| TM-100.3 | Créer script backup.sh | 1h |
| TM-100.4 | Créer script restore.sh | 1h |
| TM-100.5 | Tests des scripts | 1h |

---

### TM-101 : Monitoring et healthchecks

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** opérateur,
**Je veux** des healthchecks et du monitoring basique,
**Afin de** savoir si l'application fonctionne correctement.

#### Critères d'Acceptation

- [ ] Endpoint `/health` sur le backend
- [ ] Retourne : status, version, uptime, db_connected
- [ ] Healthcheck Docker utilise cet endpoint
- [ ] Dashboard Grafana avec métriques essentielles
- [ ] Alertes basiques (service down)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-101.1 | Créer endpoint /health | 1h |
| TM-101.2 | Configurer healthcheck Docker | 0.5h |
| TM-101.3 | Créer dashboard Grafana minimal | 2h |
| TM-101.4 | Configurer alertes basiques | 1h |

---

## Récapitulatif des Estimations

| Story | Titre | SP |
|-------|-------|:--:|
| TM-95 | README principal | 2 |
| TM-96 | Documentation API | 2 |
| TM-97 | Documentation architecture | 2 |
| TM-98 | Guide utilisateur | 2 |
| TM-99 | Docker Compose production | 3 |
| TM-100 | Scripts de déploiement | 2 |
| TM-101 | Monitoring et healthchecks | 2 |
| **Total** | | **15 SP** |

---

## Notes Techniques

### Structure Documentation

```
docs/
├── README.md               # Lien vers autres docs
├── ARCHITECTURE.md         # Architecture technique
├── API.md                  # Lien vers Swagger
├── DEPLOYMENT.md           # Guide déploiement
├── USER_GUIDE.md           # Guide utilisateur
├── epics/                  # Backlog (ce dossier)
│   └── TM-E*.md
└── images/                 # Screenshots et diagrammes
    └── ...
```

### Variables d'Environnement

```bash
# .env.example
# === Application ===
APP_ENV=production
APP_URL=https://timemanager.example.com

# === Database ===
DATABASE_URL=postgres://user:pass@db:5432/timemanager
DATABASE_POOL_SIZE=10

# === Auth ===
JWT_SECRET=<generate-secure-secret>
JWT_ACCESS_EXPIRY=15m
JWT_REFRESH_EXPIRY=7d

# === External Services ===
SMTP_HOST=smtp.example.com
SMTP_PORT=587
SMTP_USER=notifications@example.com
SMTP_PASS=<smtp-password>
```

### Docker Compose Production

```yaml
# docker-compose.prod.yml
version: '3.8'

services:
  backend:
    build:
      context: ./back
      dockerfile: Dockerfile
      target: production
    env_file: .env
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3
    restart: unless-stopped
    depends_on:
      db:
        condition: service_healthy

  frontend:
    build:
      context: ./front
      dockerfile: Dockerfile
      target: production
    restart: unless-stopped

  db:
    image: postgres:16-alpine
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 10s
      timeout: 5s
      retries: 5
    restart: unless-stopped

  traefik:
    image: traefik:v3.0
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
      - traefik_certs:/letsencrypt
    restart: unless-stopped

volumes:
  postgres_data:
  traefik_certs:
```

### Endpoint Health

```rust
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub database: String,
    pub timestamp: DateTime<Utc>,
}

async fn health_check(
    State(pool): State<DbPool>,
) -> Json<HealthResponse> {
    let db_status = match pool.get() {
        Ok(_) => "connected",
        Err(_) => "disconnected",
    };

    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: get_uptime(),
        database: db_status.to_string(),
        timestamp: Utc::now(),
    })
}
```

### Critères Epitech Couverts

| Critère | Implémentation |
|---------|----------------|
| dockerfiles | Multi-stage builds optimisés |
| orchestration | docker-compose.prod.yml complet |
| persistency | Volumes nommés pour BDD |
| Documentation | README, API docs, Architecture |
