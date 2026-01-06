# TM-E01 : Infrastructure & Setup

## Informations Epic

| Champ | Valeur |
|-------|--------|
| **ID** | TM-E01 |
| **Titre** | Infrastructure & Setup |
| **Priorité** | P0 - Bloquant |
| **Estimation globale** | 26 SP |
| **Sprint cible** | Sprint 1 |
| **Dépendances** | Aucune (Epic fondateur) |
| **Etiquettes** | backend, frontend, devops, docker, ci/cd, observabilité |

---

## Description

### Contexte

Le projet Time Manager est une application SaaS de gestion du temps pour entreprises. Avant tout développement fonctionnel, l'équipe a besoin d'un environnement de développement stable, reproductible et conforme aux standards Epitech.

Cette Epic pose les fondations techniques sur lesquelles toutes les autres Epics s'appuieront. Un retard ou des problèmes sur cette Epic impacteront l'ensemble du projet.

### Objectif Business

Permettre à l'équipe de 5 développeurs de travailler en parallèle sur un environnement local identique, avec des outils de qualité (lint, format) et une base de données fonctionnelle.

### Valeur Apportée

- **Pour les développeurs** : Environnement prêt à l'emploi en une commande (`docker-compose up`)
- **Pour le projet** : Base solide, reproductible, conforme aux critères Epitech
- **Pour la qualité** : CI de base garantissant un code propre dès le départ

---

## Scope

### Inclus

- Initialisation projet Rust/Axum avec structure de dossiers
- Initialisation projet React/TypeScript/Vite avec Shadcn/UI
- Configuration Docker Compose pour environnement de dev
- Configuration Diesel ORM et migrations initiales (organizations, users)
- Stack observabilité (Loki, Prometheus, Grafana)
- Pipeline CI GitHub Actions (lint + build)
- Configuration Traefik comme reverse proxy

### Exclus

- Logique métier (authentification, pointage, etc.)
- Tests d'intégration (Epic E13)
- Configuration production (Epic E14)
- Déploiement sur serveur

---

## Critères de Succès de l'Epic

- [ ] `docker-compose up` démarre tous les services sans erreur
- [ ] Endpoint `/health` du backend répond 200 OK
- [ ] Frontend accessible sur `http://localhost:3000`
- [ ] Base PostgreSQL accessible et migrations appliquées
- [ ] Grafana accessible avec dashboard de base
- [ ] CI GitHub Actions passe sur une PR

---

## User Stories

---

### TM-1 : Setup projet backend Rust/Axum

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 5 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur backend,
**Je veux** un projet Rust/Axum initialisé avec toutes les dépendances configurées,
**Afin de** pouvoir commencer à développer les endpoints API.

#### Contexte Détaillé

Le backend utilise Rust avec le framework Axum pour ses performances et sa sécurité mémoire. Le projet doit suivre une architecture modulaire (Clean Architecture) facilitant les tests et la maintenance.

Les dépendances principales sont :
- **Axum 0.7+** : Framework HTTP
- **Diesel 2.1+** : ORM avec migrations
- **Tokio** : Runtime async
- **Serde** : Sérialisation JSON
- **Tracing** : Logs structurés
- **jsonwebtoken** : JWT
- **argon2** : Hash passwords

#### Critères d'Acceptation

- [ ] Cargo.toml avec toutes les dépendances listées et versions fixées
- [ ] Structure de dossiers créée :
  ```
  backend/
  ├── src/
  │   ├── main.rs
  │   ├── lib.rs
  │   ├── config/
  │   ├── api/
  │   ├── domain/
  │   ├── models/
  │   ├── repositories/
  │   ├── services/
  │   ├── middleware/
  │   ├── extractors/
  │   ├── error/
  │   └── utils/
  ├── migrations/
  └── tests/
  ```
- [ ] Endpoint `GET /health` retourne `{"status": "ok"}`
- [ ] Configuration via variables d'environnement (dotenv)
- [ ] `.env.example` avec toutes les variables documentées
- [ ] `cargo build` compile sans erreur
- [ ] `cargo clippy` passe sans warning

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-1.1 | Créer le projet avec `cargo new backend` | 0.5h |
| TM-1.2 | Configurer Cargo.toml avec toutes les dépendances | 1h |
| TM-1.3 | Créer la structure de dossiers src/ | 1h |
| TM-1.4 | Implémenter le module config (lecture .env) | 2h |
| TM-1.5 | Implémenter le endpoint /health | 1h |
| TM-1.6 | Configurer tracing pour les logs | 1h |
| TM-1.7 | Créer .env.example documenté | 0.5h |

---

### TM-2 : Setup projet frontend React/TypeScript

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 5 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur frontend,
**Je veux** un projet React 18 avec TypeScript, Vite, Tailwind et Shadcn/UI configurés,
**Afin de** pouvoir développer l'interface utilisateur avec des composants modernes.

#### Contexte Détaillé

Le frontend utilise une stack moderne optimisée pour la DX (Developer Experience) :
- **Vite** : Build ultra-rapide, HMR instantané
- **React 18** : Concurrent features, Suspense
- **TypeScript 5** : Type safety stricte
- **Tailwind CSS** : Utility-first CSS
- **Shadcn/UI** : Composants accessibles et customisables

#### Critères d'Acceptation

- [ ] Projet initialisé avec Vite + React 18 + TypeScript
- [ ] Tailwind CSS configuré et fonctionnel
- [ ] Shadcn/UI initialisé avec composants de base (Button, Input, Card)
- [ ] ESLint configuré avec règles strictes
- [ ] Prettier configuré pour formatage cohérent
- [ ] Structure de dossiers créée :
  ```
  frontend/
  ├── src/
  │   ├── main.tsx
  │   ├── App.tsx
  │   ├── index.css
  │   ├── config/
  │   ├── types/
  │   ├── api/
  │   ├── hooks/
  │   ├── context/
  │   ├── lib/
  │   ├── components/
  │   │   ├── ui/         (shadcn)
  │   │   ├── layout/
  │   │   └── shared/
  │   ├── pages/
  │   └── routes/
  └── tests/
  ```
- [ ] `npm run dev` démarre sans erreur
- [ ] `npm run lint` passe sans erreur
- [ ] `npm run build` compile sans erreur

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-2.1 | Créer le projet avec `npm create vite@latest` | 0.5h |
| TM-2.2 | Configurer TypeScript (tsconfig strict) | 1h |
| TM-2.3 | Installer et configurer Tailwind CSS | 1h |
| TM-2.4 | Initialiser Shadcn/UI et ajouter composants de base | 2h |
| TM-2.5 | Configurer ESLint avec règles React/TS | 1h |
| TM-2.6 | Configurer Prettier | 0.5h |
| TM-2.7 | Créer la structure de dossiers src/ | 1h |
| TM-2.8 | Créer .env.example | 0.5h |

---

### TM-3 : Configuration Docker Compose (développement)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 5 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur,
**Je veux** lancer tout l'environnement avec une seule commande `docker-compose up`,
**Afin de** avoir un setup reproductible et identique pour toute l'équipe.

#### Contexte Détaillé

Docker Compose orchestre tous les services nécessaires au développement :
- Backend Rust (hot reload avec cargo-watch)
- Frontend React (HMR Vite)
- PostgreSQL 16
- Traefik (reverse proxy)
- Mailpit (emails de dev)

Les volumes persistants garantissent la conservation des données entre les redémarrages.

#### Critères d'Acceptation

- [ ] Fichier `docker-compose.yml` à la racine du projet
- [ ] Service **backend** :
  - Port 8080 exposé
  - Volume pour hot reload
  - Dépend de postgres
- [ ] Service **frontend** :
  - Port 3000 exposé
  - Volume pour HMR
- [ ] Service **postgres** :
  - PostgreSQL 16
  - Port 5432 exposé
  - Volume persistant pour les données
  - Script d'init pour créer la base
- [ ] Service **traefik** :
  - Ports 80 et 8080 (dashboard)
  - Routing vers backend et frontend
  - Labels Docker pour auto-discovery
- [ ] Service **mailpit** :
  - Port 1025 (SMTP)
  - Port 8025 (UI web)
- [ ] Fichier `.env.example` avec toutes les variables
- [ ] `docker-compose up` démarre tous les services
- [ ] `docker-compose down -v` nettoie tout proprement

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-3.1 | Créer Dockerfile backend (multi-stage) | 2h |
| TM-3.2 | Créer Dockerfile frontend (multi-stage) | 1h |
| TM-3.3 | Configurer service postgres avec init script | 1h |
| TM-3.4 | Configurer service traefik avec routing | 2h |
| TM-3.5 | Configurer service mailpit | 0.5h |
| TM-3.6 | Créer docker-compose.yml complet | 2h |
| TM-3.7 | Créer .env.example avec documentation | 0.5h |
| TM-3.8 | Tester le setup complet et documenter | 1h |

---

### TM-4 : Configuration Diesel ORM et migrations initiales

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur backend,
**Je veux** Diesel ORM configuré avec les migrations pour les tables de base,
**Afin de** pouvoir persister les données utilisateurs et organisations.

#### Contexte Détaillé

Diesel est l'ORM choisi pour Rust car il offre :
- Vérification des requêtes à la compilation
- Migrations versionnées
- Support natif PostgreSQL (UUID, JSONB, ENUM)

Les tables initiales sont `organizations` et `users` car elles sont nécessaires pour l'authentification (Epic E02).

#### Critères d'Acceptation

- [ ] `diesel.toml` configuré
- [ ] CLI Diesel installée (`cargo install diesel_cli`)
- [ ] Migration `create_organizations` :
  ```sql
  - id (UUID, PK)
  - name (VARCHAR)
  - slug (VARCHAR, UNIQUE)
  - timezone (VARCHAR, default 'Europe/Paris')
  - created_at, updated_at (TIMESTAMP)
  ```
- [ ] Migration `create_users` :
  ```sql
  - id (UUID, PK)
  - organization_id (UUID, FK)
  - email (VARCHAR, UNIQUE per org)
  - password_hash (VARCHAR)
  - first_name, last_name (VARCHAR)
  - role (ENUM: employee, manager, admin, super_admin)
  - created_at, updated_at, deleted_at (TIMESTAMP)
  ```
- [ ] `schema.rs` généré automatiquement
- [ ] `diesel migration run` s'exécute sans erreur
- [ ] `diesel migration redo` fonctionne (rollback + run)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-4.1 | Installer diesel_cli avec features postgres | 0.5h |
| TM-4.2 | Créer diesel.toml | 0.5h |
| TM-4.3 | Créer migration create_user_role_enum | 1h |
| TM-4.4 | Créer migration create_organizations | 1h |
| TM-4.5 | Créer migration create_users | 1h |
| TM-4.6 | Configurer la connexion pool (r2d2) | 1h |
| TM-4.7 | Tester migrations (run, redo, revert) | 1h |

---

### TM-5 : Stack Observabilité (Loki, Prometheus, Grafana)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur,
**Je veux** une stack d'observabilité complète,
**Afin de** pouvoir débugger et monitorer l'application.

#### Contexte Détaillé

La stack OTEL (OpenTelemetry) permet de centraliser :
- **Logs** : Loki (agrégation des logs de tous les services)
- **Métriques** : Prometheus (CPU, mémoire, requêtes/sec)
- **Visualisation** : Grafana (dashboards)

Cette stack est un critère Epitech et facilite grandement le debugging en équipe.

#### Critères d'Acceptation

- [ ] Service **Loki** ajouté au docker-compose
- [ ] Service **Prometheus** configuré pour scraper le backend
- [ ] Service **Grafana** accessible sur port 3001
- [ ] Dashboard Grafana de base créé :
  - Logs du backend
  - Métriques de santé (up/down)
  - Requêtes par endpoint
- [ ] Backend expose endpoint `/metrics` pour Prometheus
- [ ] Documentation d'accès dans README

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-5.1 | Ajouter service Loki au docker-compose | 1h |
| TM-5.2 | Configurer Prometheus (prometheus.yml) | 1h |
| TM-5.3 | Ajouter service Grafana avec provisioning | 1h |
| TM-5.4 | Exposer /metrics dans le backend Axum | 1h |
| TM-5.5 | Créer dashboard Grafana de base | 2h |
| TM-5.6 | Documenter l'accès et l'utilisation | 0.5h |

---

### TM-6 : Pipeline CI GitHub Actions

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur,
**Je veux** un pipeline CI qui valide automatiquement le code sur chaque PR,
**Afin de** garantir la qualité du code avant merge.

#### Contexte Détaillé

Le pipeline CI de base vérifie :
- Le formatage du code (rustfmt, prettier)
- Les erreurs de lint (clippy, eslint)
- La compilation (cargo build, npm build)

Les tests seront ajoutés dans l'Epic E13.

#### Critères d'Acceptation

- [ ] Fichier `.github/workflows/ci.yml` créé
- [ ] Job **backend-lint** :
  - `cargo fmt --check`
  - `cargo clippy -- -D warnings`
- [ ] Job **backend-build** :
  - `cargo build --release`
- [ ] Job **frontend-lint** :
  - `npm run lint`
  - `npm run type-check` (tsc --noEmit)
- [ ] Job **frontend-build** :
  - `npm run build`
- [ ] Pipeline déclenché sur PR vers `main` et `develop`
- [ ] Badge status dans le README

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-6.1 | Créer workflow ci.yml de base | 1h |
| TM-6.2 | Configurer job backend-lint (fmt + clippy -D warnings) | 1h |
| TM-6.3 | Configurer job backend-build | 0.5h |
| TM-6.4 | Configurer job frontend-lint | 1h |
| TM-6.5 | Configurer job frontend-build | 0.5h |
| TM-6.6 | Ajouter badge CI au README | 0.5h |
| TM-6.7 | Tester le pipeline sur une PR | 0.5h |
| TM-6.8 | Configurer cargo clippy -D warnings | 0.5h |
| TM-6.9 | Ajouter cargo-audit (vulnérabilités) | 0.5h |
| TM-6.10 | Ajouter cargo-deny (licences) | 0.5h |
| TM-6.11 | Configurer cargo nextest (tests rapides) | 0.5h |
| TM-6.12 | Ajouter cargo-machete (deps inutilisées) | 0.5h |
| TM-6.13 | Ajouter cargo-sort (tri Cargo.toml) | 0.5h |

---

### TM-102 : Configuration Rate Limiting Traefik

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 1 SP |
| **Assigné** | - |

#### Description

**En tant que** responsable sécurité,
**Je veux** un rate limiting configuré sur tous les endpoints sensibles,
**Afin de** protéger l'application contre les abus et attaques DoS.

#### Critères d'Acceptation

- [ ] Rate limit global : 100 req/min par utilisateur authentifié
- [ ] Rate limit login : 5 req/min par IP
- [ ] Rate limit register : 1 req/5min par IP
- [ ] Rate limit password reset : 3 req/heure par email
- [ ] Rate limit export données : 1 req/jour par utilisateur
- [ ] Configuration via labels Docker Compose
- [ ] Headers `X-RateLimit-*` retournés dans les réponses
- [ ] Retour 429 Too Many Requests si limite atteinte

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-102.1 | Configurer middleware rate limit Traefik | 1h |
| TM-102.2 | Définir les règles par endpoint | 1h |
| TM-102.3 | Tester les limites | 0.5h |
| TM-102.4 | Documenter la configuration | 0.5h |

---

### TM-103 : Headers de Sécurité HTTP

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 1 SP |
| **Assigné** | - |

#### Description

**En tant que** responsable sécurité,
**Je veux** des headers de sécurité HTTP sur toutes les réponses,
**Afin de** protéger contre les attaques XSS, clickjacking et autres.

#### Critères d'Acceptation

- [ ] X-Content-Type-Options: nosniff
- [ ] X-Frame-Options: DENY
- [ ] Content-Security-Policy: default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'
- [ ] Strict-Transport-Security: max-age=31536000; includeSubDomains
- [ ] X-XSS-Protection: 1; mode=block
- [ ] Referrer-Policy: strict-origin-when-cross-origin
- [ ] Permissions-Policy: geolocation=(), microphone=(), camera=()
- [ ] Configuré via middleware Traefik

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-103.1 | Créer middleware headers sécurité Traefik | 1h |
| TM-103.2 | Configurer CSP appropriée | 1h |
| TM-103.3 | Tester avec security scanner | 0.5h |
| TM-103.4 | Documenter les headers | 0.5h |

---

### TM-104 : Configuration CORS

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 1 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur backend,
**Je veux** une configuration CORS sécurisée,
**Afin de** permettre uniquement les requêtes depuis les domaines autorisés.

#### Critères d'Acceptation

- [ ] Access-Control-Allow-Origin : domaine(s) spécifique(s) (PAS *)
- [ ] Access-Control-Allow-Credentials : true
- [ ] Access-Control-Allow-Methods : GET, POST, PUT, DELETE, OPTIONS
- [ ] Access-Control-Allow-Headers : Authorization, Content-Type, X-CSRF-Token
- [ ] Access-Control-Max-Age : 3600
- [ ] Configuration différente dev/prod
- [ ] Preflight requests gérées correctement

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-104.1 | Configurer CORS dans Axum | 1h |
| TM-104.2 | Définir whitelist domaines | 0.5h |
| TM-104.3 | Tester requêtes cross-origin | 0.5h |
| TM-104.4 | Documenter configuration | 0.5h |

---

## Récapitulatif des Estimations

| Story | Titre | SP |
|-------|-------|:--:|
| TM-1 | Setup projet backend Rust/Axum | 5 |
| TM-2 | Setup projet frontend React/TypeScript | 5 |
| TM-3 | Configuration Docker Compose (dev) | 5 |
| TM-4 | Configuration Diesel ORM et migrations | 3 |
| TM-5 | Stack Observabilité | 3 |
| TM-6 | Pipeline CI GitHub Actions | 3 |
| TM-102 | Configuration Rate Limiting Traefik | 1 |
| TM-103 | Headers de Sécurité HTTP | 1 |
| TM-104 | Configuration CORS | 1 |
| **Total** | | **27 SP** |

---

## Notes Techniques

### Variables d'Environnement Requises

```env
# Database
DATABASE_URL=postgres://timemanager:password@postgres:5432/timemanager

# Backend
RUST_LOG=debug
APP_HOST=0.0.0.0
APP_PORT=8080

# Frontend
VITE_API_URL=http://localhost:8080/api/v1

# Traefik
TRAEFIK_DASHBOARD=true
```

### Commandes Utiles

```bash
# Démarrer l'environnement
docker-compose up -d

# Voir les logs
docker-compose logs -f backend

# Exécuter les migrations
docker-compose exec backend diesel migration run

# Accès aux services
# - Frontend: http://localhost:3000
# - Backend API: http://localhost:8080
# - Grafana: http://localhost:3001
# - Traefik Dashboard: http://localhost:8080/dashboard
# - Mailpit: http://localhost:8025
```

### Workflows GitHub Actions

```yaml
# .github/workflows/ci.yml - LINT & BUILD (BLOQUANT)
name: CI
on: [push, pull_request]
jobs:
  backend-lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo fmt --check
      - run: cargo clippy --all --all-targets -- -D warnings
      - run: cargo check --all-features

  backend-build:
    runs-on: ubuntu-latest
    needs: backend-lint
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo build --release

  frontend-lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'
          cache-dependency-path: frontend/package-lock.json
      - run: cd frontend && npm ci
      - run: cd frontend && npm run lint
      - run: cd frontend && npm run type-check

  frontend-build:
    runs-on: ubuntu-latest
    needs: frontend-lint
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
      - run: cd frontend && npm ci
      - run: cd frontend && npm run build

# .github/workflows/security.yml - AUDITS (BLOQUANT)
name: Security Audit
on:
  push:
    branches: [main, develop]
  schedule:
    - cron: '0 0 * * *'  # Daily
jobs:
  audit-cargo:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo install cargo-audit cargo-deny
      - run: cargo audit
      - run: cargo deny check

  audit-npm:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cd frontend && npm audit --audit-level=moderate
```

### Outils CI Rust

```bash
# Installation globale des outils CI
cargo install cargo-nextest       # Tests rapides et parallèles
cargo install cargo-tarpaulin     # Couverture de code
cargo install cargo-audit         # Audit vulnérabilités
cargo install cargo-deny          # Audit licences et deps
cargo install cargo-machete       # Détection deps inutilisées
cargo install cargo-sort          # Tri Cargo.toml
```

### Dev Dependencies Cargo.toml

```toml
[dev-dependencies]
tokio-test = "0.4"
mockall = "0.11"            # Mocking
fake = "2.5"                # Fake data generation
wiremock = "0.5"            # HTTP mocking
serial_test = "2.0"         # Tests séquentiels
testcontainers = "0.15"     # Conteneurs pour tests
```

### Configuration Sécurité Traefik

```yaml
# traefik/dynamic.yml
http:
  middlewares:
    security-headers:
      headers:
        frameDeny: true
        contentTypeNosniff: true
        browserXssFilter: true
        referrerPolicy: "strict-origin-when-cross-origin"
        stsSeconds: 31536000
        stsIncludeSubdomains: true
        contentSecurityPolicy: "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'"

    rate-limit-global:
      rateLimit:
        average: 100
        burst: 200
        period: 1m

    rate-limit-auth:
      rateLimit:
        average: 5
        burst: 10
        period: 1m
```
