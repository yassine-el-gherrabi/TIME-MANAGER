# Time Manager - Document de Référence Projet

## Vue d'ensemble

**Application web multi-tenant de gestion du temps de travail.**

- Pointage des employés
- Gestion des absences avec workflow de validation
- Horaires configurables par équipe/utilisateur
- Dashboards et KPIs
- 4 rôles avec droits cascadés

---

## Stack Technique

### Backend
| Composant | Technologie |
|-----------|-------------|
| Langage | Rust |
| Framework | Axum |
| ORM | Diesel |
| Base de données | PostgreSQL 16 |
| Auth | JWT (access 15min + refresh 7j) |
| Password | Argon2 |
| Validation | crate `validator` |
| Errors | `thiserror` |

### Frontend
| Composant | Technologie |
|-----------|-------------|
| Framework | React 18 + TypeScript |
| State | TanStack Query + React Context |
| UI | Shadcn/UI + Tailwind CSS |
| Charts | Recharts |
| HTTP | Axios |
| Routing | React Router v6 |

### Infrastructure
| Composant | Technologie |
|-----------|-------------|
| Reverse Proxy | Traefik |
| Containers | Docker + Docker Compose |
| CI/CD | GitHub Actions |
| Hosting | Hetzner VPS |
| SMTP | Brevo (prod) / Mailpit (dev) |

### Observabilité
| Composant | Technologie |
|-----------|-------------|
| Collector | OpenTelemetry |
| Logs | Loki |
| Metrics | Prometheus |
| Traces | Tempo |
| Dashboard | Grafana |

---

## Personas

### Employé
- Pointe son arrivée/départ
- Consulte ses heures et soldes de congés
- Soumet des demandes d'absence
- Demande des corrections de pointage

### Manager
- Tout ce que fait l'Employé
- Voit les pointages et absences de son équipe
- Valide/refuse les demandes de son équipe
- Consulte les KPIs de son équipe

### Admin (RH)
- Tout ce que fait le Manager (sur toute l'organisation)
- CRUD utilisateurs et équipes
- Configure horaires, types d'absences, jours fériés
- Exporte les données pour la paie

### Super Admin
- Tout ce que fait l'Admin
- Configure l'organisation
- Accède aux logs d'audit
- Gère le multi-tenant

---

## Rôles et Permissions

Droits cascadés : chaque rôle hérite des droits du rôle inférieur.

```
SUPER_ADMIN > ADMIN > MANAGER > EMPLOYEE
```

### Matrice

| Action | EMPLOYEE | MANAGER | ADMIN | SUPER_ADMIN |
|--------|:--------:|:-------:|:-----:|:-----------:|
| **Profil** |
| Voir/éditer son profil | ✓ | ✓ | ✓ | ✓ |
| **Pointage** |
| Pointer | ✓ | ✓ | ✓ | ✓ |
| Voir ses pointages | ✓ | ✓ | ✓ | ✓ |
| Demander correction | ✓ | ✓ | ✓ | ✓ |
| Voir pointages équipe | | ✓* | ✓ | ✓ |
| Valider corrections | | ✓* | ✓ | ✓ |
| **Absences** |
| Soumettre demande | ✓ | ✓ | ✓ | ✓ |
| Voir ses absences/soldes | ✓ | ✓ | ✓ | ✓ |
| Voir absences équipe | | ✓* | ✓ | ✓ |
| Valider absences | | ✓* | ✓ | ✓ |
| Ajuster soldes | | | ✓ | ✓ |
| **Users & Teams** |
| Voir son équipe | ✓ | ✓ | ✓ | ✓ |
| Voir tous les users | | | ✓ | ✓ |
| CRUD users | | | ✓ | ✓ |
| CRUD teams | | | ✓ | ✓ |
| **Configuration** |
| Config horaires | | | ✓ | ✓ |
| Config types absences | | | ✓ | ✓ |
| Config jours fériés | | | ✓ | ✓ |
| Config organisation | | | | ✓ |
| **Rapports** |
| Dashboard personnel | ✓ | ✓ | ✓ | ✓ |
| Dashboard équipe | | ✓* | ✓ | ✓ |
| Dashboard global | | | ✓ | ✓ |
| Export CSV | ✓ | ✓* | ✓ | ✓ |
| **Audit** |
| Voir logs audit | | | | ✓ |

*\* = uniquement son équipe*

---

## Fonctionnalités MVP

### 1. Authentification

| Fonctionnalité | Détail |
|----------------|--------|
| Login | Email + password |
| Tokens | Access (15min, mémoire JS) + Refresh (7j, HttpOnly cookie) |
| Logout | Révoque le refresh token |
| Reset password | Email avec lien de réinitialisation |
| Change password | Depuis le profil, requiert ancien password |
| CSRF | Double Submit Cookie |
| XSS | CSP headers, sanitization |

### 2. Multi-tenant (Organizations)

| Fonctionnalité | Détail |
|----------------|--------|
| Isolation | Toutes les données filtrées par `organization_id` |
| CRUD orgs | Super Admin uniquement |
| Paramètres | Nom, timezone par organisation |

### 3. Utilisateurs

| Fonctionnalité | Détail |
|----------------|--------|
| CRUD | Create, Read, Update, Delete (soft) |
| Champs | email, password, first_name, last_name, phone, role, team_id |
| Soft delete | `deleted_at` non null = désactivé |
| Recherche | Par nom, email, rôle, équipe |
| Profil | L'utilisateur peut éditer ses propres infos (sauf rôle) |

### 4. Équipes

| Fonctionnalité | Détail |
|----------------|--------|
| CRUD | Create, Read, Update, Delete |
| Champs | name, description, manager_id, work_schedule_id |
| Contrainte | Suppression uniquement si équipe vide |
| Membres | Géré via `users.team_id` |

### 5. Horaires

| Fonctionnalité | Détail |
|----------------|--------|
| Modèles | Templates nommés (ex: "35h standard") |
| Par jour | start_time, end_time, break_minutes pour chaque jour |
| Tolérance | Minutes de marge avant retard (pour KPI) |
| Assignation | Équipe → modèle par défaut |
| Override | User → modèle personnel (prioritaire sur équipe) |
| Prédéfinis | Seed: 35h, 39h, temps partiel |

### 6. Pointage

| Fonctionnalité | Détail |
|----------------|--------|
| Clock in/out | Toggle en 1 clic |
| Statut | Est-ce que je suis pointé ? Depuis combien de temps ? |
| Historique | Liste des pointages avec filtres (date, période) |
| Correction | Demande si oubli, avec notes |
| Workflow | Correction: pending → approved/rejected par manager |
| Vue manager | Pointages de son équipe, qui est présent |

### 7. Absences

| Fonctionnalité | Détail |
|----------------|--------|
| Types | Configurables par admin (CP, RTT, Maladie, Sans solde...) |
| Propriétés type | requires_approval, affects_balance, is_paid, color |
| Demande | type, start_date, end_date, reason |
| Workflow | pending → approved/rejected/cancelled |
| Soldes | Par user, par type, par année |
| Calcul solde | remaining = initial - used + adjustment |
| Calendrier | Vue équipe: qui est absent quand |

### 8. Jours fériés

| Fonctionnalité | Détail |
|----------------|--------|
| CRUD | Admin gère la liste |
| Champs | name, date, is_recurring |
| Seed | Jours fériés français fixes |
| Usage | Exclus du calcul des heures théoriques |

### 9. Notifications

| Fonctionnalité | Détail |
|----------------|--------|
| In-app | Centre de notifications, badge non-lus |
| Email | Via Brevo (prod) / Mailpit (dev) |
| Types | Absence à valider, absence validée/refusée, correction à valider |
| Actions | Marquer comme lu, marquer tout lu |

### 10. Rapports & KPIs

| KPI | Formule |
|-----|---------|
| Taux de ponctualité | (Jours à l'heure / Jours travaillés) × 100 |
| Écart heures | Heures réelles - Heures théoriques |

| Dashboard | Contenu |
|-----------|---------|
| Employé | Ses stats personnelles |
| Manager | Stats de son équipe |
| Admin | Stats globales organisation |

| Visualisation | Type |
|---------------|------|
| Heures/jour | Bar chart |
| Évolution | Line chart |
| Export | CSV |

### 11. Audit

| Fonctionnalité | Détail |
|----------------|--------|
| Logging | Toutes les modifications (CREATE, UPDATE, DELETE) |
| Données | user_id, action, entity_type, entity_id, old/new values, IP |
| Accès | Super Admin uniquement |
| Table | Append-only, jamais modifiée |

---

## Modèle de Données

### organizations
```
id              UUID PK
name            VARCHAR NOT NULL
slug            VARCHAR UNIQUE NOT NULL
timezone        VARCHAR DEFAULT 'Europe/Paris'
created_at      TIMESTAMPTZ
updated_at      TIMESTAMPTZ
```

### users
```
id                UUID PK
organization_id   UUID FK NOT NULL
email             VARCHAR NOT NULL
password_hash     VARCHAR NOT NULL
first_name        VARCHAR NOT NULL
last_name         VARCHAR NOT NULL
phone             VARCHAR NULL
role              ENUM(employee, manager, admin, super_admin)
team_id           UUID FK NULL
work_schedule_id  UUID FK NULL
created_at        TIMESTAMPTZ
updated_at        TIMESTAMPTZ
deleted_at        TIMESTAMPTZ NULL

UNIQUE(organization_id, email)
```

### teams
```
id                UUID PK
organization_id   UUID FK NOT NULL
name              VARCHAR NOT NULL
description       TEXT NULL
manager_id        UUID FK NULL
work_schedule_id  UUID FK NULL
created_at        TIMESTAMPTZ
updated_at        TIMESTAMPTZ
```

### work_schedules
```
id                 UUID PK
organization_id    UUID FK NOT NULL
name               VARCHAR NOT NULL
weekly_hours       DECIMAL(4,2) NOT NULL
tolerance_minutes  INT DEFAULT 10
is_default         BOOLEAN DEFAULT false
created_at         TIMESTAMPTZ
updated_at         TIMESTAMPTZ
```

### work_schedule_days
```
id             UUID PK
schedule_id    UUID FK NOT NULL
day_of_week    SMALLINT NOT NULL (0=Lun, 6=Dim)
start_time     TIME NOT NULL
end_time       TIME NOT NULL
break_minutes  INT DEFAULT 60

UNIQUE(schedule_id, day_of_week)
```

### clock_entries
```
id               UUID PK
organization_id  UUID FK NOT NULL
user_id          UUID FK NOT NULL
date             DATE NOT NULL
clock_in         TIMESTAMPTZ NOT NULL
clock_out        TIMESTAMPTZ NULL
duration_minutes INT NULL
is_manual        BOOLEAN DEFAULT false
status           ENUM(pending, approved, rejected) DEFAULT 'approved'
approved_by      UUID FK NULL
notes            TEXT NULL
created_at       TIMESTAMPTZ
updated_at       TIMESTAMPTZ
```

### absence_types
```
id                 UUID PK
organization_id    UUID FK NOT NULL
name               VARCHAR NOT NULL
code               VARCHAR NOT NULL
color              VARCHAR DEFAULT '#3B82F6'
requires_approval  BOOLEAN DEFAULT true
affects_balance    BOOLEAN DEFAULT true
is_paid            BOOLEAN DEFAULT true
created_at         TIMESTAMPTZ

UNIQUE(organization_id, code)
```

### absences
```
id                UUID PK
organization_id   UUID FK NOT NULL
user_id           UUID FK NOT NULL
type_id           UUID FK NOT NULL
start_date        DATE NOT NULL
end_date          DATE NOT NULL
days_count        DECIMAL(3,1) NOT NULL
status            ENUM(pending, approved, rejected, cancelled)
reason            TEXT NULL
rejection_reason  TEXT NULL
approved_by       UUID FK NULL
approved_at       TIMESTAMPTZ NULL
created_at        TIMESTAMPTZ
updated_at        TIMESTAMPTZ
```

### leave_balances
```
id               UUID PK
organization_id  UUID FK NOT NULL
user_id          UUID FK NOT NULL
absence_type_id  UUID FK NOT NULL
year             INT NOT NULL
initial_balance  DECIMAL(4,1) NOT NULL
used             DECIMAL(4,1) DEFAULT 0
adjustment       DECIMAL(4,1) DEFAULT 0
remaining        DECIMAL(4,1) GENERATED
created_at       TIMESTAMPTZ
updated_at       TIMESTAMPTZ

UNIQUE(user_id, absence_type_id, year)
```

### holidays
```
id               UUID PK
organization_id  UUID FK NOT NULL
name             VARCHAR NOT NULL
date             DATE NOT NULL
is_recurring     BOOLEAN DEFAULT false
created_at       TIMESTAMPTZ
```

### notifications
```
id               UUID PK
organization_id  UUID FK NOT NULL
user_id          UUID FK NOT NULL
type             VARCHAR NOT NULL
title            VARCHAR NOT NULL
message          TEXT NOT NULL
data             JSONB NULL
read_at          TIMESTAMPTZ NULL
created_at       TIMESTAMPTZ
```

### refresh_tokens
```
id          UUID PK
user_id     UUID FK NOT NULL
token_hash  VARCHAR NOT NULL
expires_at  TIMESTAMPTZ NOT NULL
revoked_at  TIMESTAMPTZ NULL
created_at  TIMESTAMPTZ
```

### audit_logs
```
id               UUID PK
organization_id  UUID FK NULL
user_id          UUID FK NULL
action           VARCHAR NOT NULL
entity_type      VARCHAR NOT NULL
entity_id        UUID NOT NULL
old_values       JSONB NULL
new_values       JSONB NULL
ip_address       VARCHAR NULL
user_agent       VARCHAR NULL
created_at       TIMESTAMPTZ
```

---

## API Endpoints

### Auth
```
POST   /api/v1/auth/login              Login
POST   /api/v1/auth/logout             Logout
POST   /api/v1/auth/refresh            Refresh tokens
GET    /api/v1/auth/me                 Get current user
POST   /api/v1/auth/forgot-password    Request reset
POST   /api/v1/auth/reset-password     Reset password
PUT    /api/v1/auth/change-password    Change password
```

### Organizations (Super Admin)
```
GET    /api/v1/organizations           List orgs
GET    /api/v1/organizations/:id       Get org
POST   /api/v1/organizations           Create org
PUT    /api/v1/organizations/:id       Update org
DELETE /api/v1/organizations/:id       Delete org
```

### Users
```
GET    /api/v1/users                   List users (Admin+)
GET    /api/v1/users/:id               Get user
POST   /api/v1/users                   Create user (Admin+)
PUT    /api/v1/users/:id               Update user
DELETE /api/v1/users/:id               Soft delete user
PUT    /api/v1/users/:id/restore       Restore user (Admin+)
```

### Teams
```
GET    /api/v1/teams                   List teams
GET    /api/v1/teams/:id               Get team
POST   /api/v1/teams                   Create team (Admin+)
PUT    /api/v1/teams/:id               Update team
DELETE /api/v1/teams/:id               Delete team (Admin+)
GET    /api/v1/teams/:id/members       List members
POST   /api/v1/teams/:id/members       Add member (Admin+)
DELETE /api/v1/teams/:id/members/:uid  Remove member (Admin+)
```

### Work Schedules
```
GET    /api/v1/schedules               List schedules
GET    /api/v1/schedules/:id           Get schedule with days
POST   /api/v1/schedules               Create (Admin+)
PUT    /api/v1/schedules/:id           Update (Admin+)
DELETE /api/v1/schedules/:id           Delete (Admin+)
POST   /api/v1/schedules/:id/assign/team/:tid   Assign to team
POST   /api/v1/schedules/:id/assign/user/:uid   Assign to user
```

### Clocks
```
POST   /api/v1/clocks                  Toggle clock in/out
GET    /api/v1/clocks/status           Get current status
GET    /api/v1/clocks                  List clocks (filtres: user_id, dates)
GET    /api/v1/clocks/:id              Get clock entry
POST   /api/v1/clocks/correction       Request correction
PUT    /api/v1/clocks/:id/approve      Approve correction (Manager+)
PUT    /api/v1/clocks/:id/reject       Reject correction (Manager+)
```

### Absence Types
```
GET    /api/v1/absence-types           List types
POST   /api/v1/absence-types           Create (Admin+)
PUT    /api/v1/absence-types/:id       Update (Admin+)
DELETE /api/v1/absence-types/:id       Delete (Admin+)
```

### Absences
```
GET    /api/v1/absences                List absences (filtres: user_id, status, dates)
GET    /api/v1/absences/:id            Get absence
POST   /api/v1/absences                Create absence request
PUT    /api/v1/absences/:id            Update (if pending)
DELETE /api/v1/absences/:id            Cancel (if pending)
PUT    /api/v1/absences/:id/approve    Approve (Manager+)
PUT    /api/v1/absences/:id/reject     Reject (Manager+)
GET    /api/v1/absences/calendar       Team calendar
```

### Leave Balances
```
GET    /api/v1/balances                List balances (filtres: user_id, year)
PUT    /api/v1/balances/:id            Adjust balance (Admin+)
```

### Holidays
```
GET    /api/v1/holidays                List holidays
POST   /api/v1/holidays                Create (Admin+)
PUT    /api/v1/holidays/:id            Update (Admin+)
DELETE /api/v1/holidays/:id            Delete (Admin+)
```

### Reports
```
GET    /api/v1/reports/dashboard/employee   Personal dashboard
GET    /api/v1/reports/dashboard/manager    Team dashboard (Manager+)
GET    /api/v1/reports/dashboard/admin      Global dashboard (Admin+)
GET    /api/v1/reports/kpis/punctuality     KPI taux ponctualité
GET    /api/v1/reports/kpis/hours-balance   KPI écart heures
GET    /api/v1/reports/export               Export CSV
```

### Notifications
```
GET    /api/v1/notifications           List notifications
PUT    /api/v1/notifications/:id/read  Mark as read
PUT    /api/v1/notifications/read-all  Mark all as read
```

### Audit (Super Admin)
```
GET    /api/v1/audit-logs              List logs (filtres: entity, user, dates)
```

---

## Sécurité

### JWT
```json
// Access Token (15 min)
{
  "sub": "user-uuid",
  "org": "organization-uuid",
  "role": "employee",
  "type": "access",
  "exp": 1704068100
}

// Refresh Token (7 jours, HttpOnly cookie)
{
  "sub": "user-uuid",
  "type": "refresh",
  "jti": "unique-id",
  "exp": 1704672000
}
```

### Stockage tokens
- Access token → variable JavaScript (mémoire)
- Refresh token → cookie HttpOnly, Secure, SameSite=Strict

### CSRF
- Cookie `csrf_token` envoyé au login
- Header `X-CSRF-Token` requis sur mutations (POST, PUT, DELETE)
- Validation: cookie == header

### Passwords
- Hash: Argon2id
- Minimum 8 caractères
- Jamais stocké en clair

### Multi-tenant
- Toutes les requêtes filtrent par `organization_id`
- `organization_id` extrait du JWT
- Jamais d'accès cross-organization (sauf Super Admin)

---

## Structure Projet

```
/
├── backend/
│   ├── Cargo.toml
│   ├── Dockerfile
│   ├── diesel.toml
│   ├── migrations/
│   └── src/
│       ├── main.rs
│       ├── config/
│       ├── schema.rs
│       ├── shared/
│       │   ├── error.rs
│       │   ├── response.rs
│       │   ├── pagination.rs
│       │   └── extractors.rs
│       ├── middleware/
│       │   ├── auth.rs
│       │   ├── role_guard.rs
│       │   ├── tenant.rs
│       │   └── csrf.rs
│       ├── modules/
│       │   ├── auth/
│       │   ├── organizations/
│       │   ├── users/
│       │   ├── teams/
│       │   ├── schedules/
│       │   ├── clocks/
│       │   ├── absences/
│       │   ├── reports/
│       │   ├── notifications/
│       │   └── email/
│       └── routes.rs
│
├── frontend/
│   ├── package.json
│   ├── Dockerfile
│   └── src/
│       ├── main.tsx
│       ├── components/
│       │   ├── ui/
│       │   ├── layout/
│       │   └── features/
│       ├── hooks/
│       ├── services/
│       ├── pages/
│       ├── routes/
│       └── types/
│
├── docker/
│   ├── traefik/
│   ├── postgres/
│   ├── grafana/
│   ├── prometheus/
│   ├── loki/
│   └── tempo/
│
├── compose.yml
├── compose.prod.yml
└── docs/
```

---

## Règles Métier

### Pointage
- Un seul pointage ouvert à la fois par user
- `duration_minutes` calculé au clock_out
- Pointage manuel (`is_manual=true`) = demande de correction
- Correction requiert validation si `is_manual=true`

### Absences
- `days_count` = nombre de jours ouvrés entre start et end (exclus fériés et weekends)
- Validation requise si `absence_type.requires_approval=true`
- Balance mise à jour seulement si `status=approved` ET `affects_balance=true`
- Annulation possible seulement si `status=pending`

### Soldes congés
- `remaining = initial_balance - used + adjustment`
- `used` mis à jour automatiquement quand absence approuvée
- `adjustment` modifiable uniquement par Admin (corrections manuelles)

### Horaires
- Résolution horaire user: `user.work_schedule_id` ?? `team.work_schedule_id` ?? schedule par défaut
- Jours non définis dans `work_schedule_days` = non travaillés

### KPI Ponctualité
```
Pour chaque jour travaillé:
  - scheduled_start = work_schedule_days.start_time
  - actual_start = clock_entries.clock_in (heure seulement)
  - tolerance = work_schedules.tolerance_minutes
  - ponctuel = actual_start <= scheduled_start + tolerance

taux = (count ponctuel / count jours) × 100
```

### KPI Écart heures
```
heures_theoriques = Σ (work_schedule_days.end - start - break) 
                   pour jours ouvrés
                   - jours fériés
                   - jours absence approuvée

heures_reelles = Σ clock_entries.duration_minutes / 60

ecart = heures_reelles - heures_theoriques
```

---

## Seed Data

### Organization par défaut
- name: "Demo Company"
- slug: "demo"
- timezone: "Europe/Paris"

### Users par défaut
| Email | Role | Password |
|-------|------|----------|
| superadmin@demo.com | super_admin | Admin123! |
| admin@demo.com | admin | Admin123! |
| manager@demo.com | manager | Admin123! |
| employee@demo.com | employee | Admin123! |

### Types d'absences par défaut
| Code | Nom | Affects Balance | Requires Approval |
|------|-----|-----------------|-------------------|
| CP | Congés payés | true | true |
| RTT | RTT | true | true |
| MALADIE | Maladie | false | false |
| SANS_SOLDE | Sans solde | false | true |

### Jours fériés français (récurrents)
- 1er janvier, 1er mai, 8 mai, 14 juillet, 15 août
- 1er novembre, 11 novembre, 25 décembre

### Horaires par défaut
**35h standard**
- Lun-Ven: 09:00-17:00, pause 60min
- weekly_hours: 35
- tolerance: 10min

---

## Variables d'environnement

### Backend
```env
DATABASE_URL=postgres://user:pass@host:5432/timemanager
JWT_SECRET=random-256-bits-secret
JWT_ACCESS_EXPIRES=900
JWT_REFRESH_EXPIRES=604800
SMTP_HOST=smtp.brevo.com
SMTP_PORT=587
SMTP_USER=xxx
SMTP_PASS=xxx
SMTP_FROM=noreply@timemanager.com
RUST_LOG=info
OTEL_EXPORTER_OTLP_ENDPOINT=http://otel-collector:4317
```

### Frontend
```env
VITE_API_URL=http://localhost:3000/api/v1
```

---

## Tests

### Backend
- **Unit tests**: logique métier (services)
- **Integration tests**: Testcontainers avec PostgreSQL réel
- **Rollback**: chaque test dans une transaction rollback
- **Coverage**: > 70%

### Frontend
- **Unit tests**: Vitest pour composants et hooks
- **Coverage**: > 60%

### CI Pipeline
1. `cargo fmt --check`
2. `cargo clippy`
3. `cargo test`
4. `cargo tarpaulin` (coverage)
5. `npm run lint`
6. `npm run typecheck`
7. `npm run test`
8. Docker build smoke test

---

## Critères Epitech couverts

| Critère | Couverture |
|---------|------------|
| dockerfiles | Dockerfiles multi-stage dev + prod |
| containers | Backend, frontend, DB, proxy isolés |
| persistency | Volumes pour logs et data |
| orchestration | Docker Compose |
| env_specificity | .env.dev / .env.prod |
| secrets | Variables non commitées |
| api_crafting | REST API complète |
| data_persist | 13 tables, schéma normalisé |
| auth_jwt | JWT access + refresh |
| auth_persist | Refresh token 7 jours |
| auth_sec | CSRF + XSS protection |
| roles | 4 rôles cascadés |
| data_viz | Recharts, 2 KPIs |
| uiux_quality | Shadcn/UI, design pro |
| tests_sequence | Unit + intégration |
| tests_coverage | Coverage > 60% |
| tests_automation | CI pipeline |
| ci_pipeline | GitHub Actions |
| ci_quality | Lint + tests = gates |
| versioning_basics | Git flow, conventional commits |
| doc_basic | Architecture doc, README |