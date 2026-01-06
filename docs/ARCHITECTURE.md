# Time Manager - Architecture Technique

> **Projet Epitech MSc** | Équipe Trinity | 5 développeurs

## 1. Vue d'Ensemble

Time Manager est une application SaaS de gestion du temps pour entreprises, permettant le suivi des présences, la gestion des absences et le reporting RH.

### Caractéristiques Principales

| Aspect | Choix | Justification |
|--------|-------|---------------|
| **Architecture** | Monolithe modulaire | Simplicité initiale, prêt microservices |
| **Multi-tenant** | Isolation par organisation | Scalabilité, sécurité données |
| **API** | REST versionnée | Standard industrie, tooling mature |
| **Auth** | JWT + Refresh Token | Stateless, sécurisé |

---

## 2. Diagramme d'Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         INTERNET                                 │
└───────────────────────────┬─────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                    TRAEFIK (Reverse Proxy)                       │
│         SSL/TLS • Rate Limiting • Load Balancing                 │
│                    Ports: 80, 443                                │
└───────────────────────────┬─────────────────────────────────────┘
                            │
            ┌───────────────┴───────────────┐
            ▼                               ▼
┌───────────────────────┐       ┌───────────────────────┐
│   FRONTEND (React)    │       │   BACKEND (Rust)      │
│   Port: 3000          │       │   Port: 8080          │
│                       │       │                       │
│ • React 18 + TS       │       │ • Axum Framework      │
│ • TanStack Query      │       │ • Diesel ORM          │
│ • Shadcn/UI           │       │ • JWT Auth            │
│ • Recharts            │       │ • Multi-tenant        │
└───────────────────────┘       └───────────┬───────────┘
                                            │
                                            ▼
                                ┌───────────────────────┐
                                │   PostgreSQL 16       │
                                │   Port: 5432          │
                                └───────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                    OBSERVABILITÉ (Stack OTEL)                    │
│                                                                  │
│  ┌─────────┐  ┌──────────┐  ┌───────┐  ┌─────────┐             │
│  │  Loki   │  │Prometheus│  │ Tempo │  │ Grafana │             │
│  │ (logs)  │  │(metrics) │  │(traces│  │(dashboard)            │
│  └─────────┘  └──────────┘  └───────┘  └─────────┘             │
│                      ▲                                          │
│              ┌───────┴───────┐                                  │
│              │ OTEL Collector │                                  │
│              └───────────────┘                                  │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                         SERVICES                                 │
│                                                                  │
│  ┌──────────────┐   ┌──────────────┐                            │
│  │   Mailpit    │   │    Brevo     │                            │
│  │   (dev)      │   │   (prod)     │                            │
│  │  Port: 1025  │   │  SMTP API    │                            │
│  └──────────────┘   └──────────────┘                            │
└─────────────────────────────────────────────────────────────────┘
```

---

## 3. Stack Technique

### 3.1 Backend (Rust)

| Composant | Technologie | Version | Rôle |
|-----------|-------------|---------|------|
| Runtime | Rust | 1.75+ | Langage principal |
| Framework | Axum | 0.7+ | HTTP server, routing |
| ORM | Diesel | 2.1+ | Database access, migrations |
| Database | PostgreSQL | 16 | Persistance |
| Auth | jsonwebtoken | 9+ | JWT encode/decode |
| Password | argon2 | 0.5+ | Password hashing |
| Validation | validator | 0.16+ | Input validation |
| Sérialisation | serde | 1.0+ | JSON handling |
| Erreurs | thiserror | 1.0+ | Error types |
| Async | Tokio | 1.0+ | Async runtime |
| Telemetry | tracing | 0.1+ | Logs, traces |

**Justification Rust :**
- Performance : Pas de GC, latence prédictible
- Sécurité : Memory safety à la compilation
- Fiabilité : "If it compiles, it works"
- Binaire : 10-15MB vs 200MB+ Node.js

### 3.2 Frontend (React)

| Composant | Technologie | Version | Rôle |
|-----------|-------------|---------|------|
| Framework | React | 18+ | UI library |
| Langage | TypeScript | 5+ | Type safety |
| Build | Vite | 5+ | Dev server, bundling |
| State (Server) | TanStack Query | 5+ | API state |
| State (UI) | React Context | - | UI state |
| Routing | React Router | 6+ | Navigation |
| UI | Shadcn/UI | - | Components |
| Styling | Tailwind CSS | 3+ | Utility CSS |
| Charts | Recharts | 2+ | Data visualization |
| HTTP | Axios | 1+ | API calls |
| Icons | Lucide React | - | Icon library |
| Forms | React Hook Form | 7+ | Form handling |
| Validation | Zod | 3+ | Schema validation |

### 3.3 Infrastructure

| Composant | Technologie | Rôle |
|-----------|-------------|------|
| Reverse Proxy | Traefik v3 | SSL, routing, load balancing |
| Containers | Docker | Isolation |
| Orchestration | Docker Compose | Multi-container |
| CI/CD | GitHub Actions | Build, test, deploy |
| Hosting | Hetzner VPS | Production server |

---

## 4. Schéma Base de Données

### 4.1 Entity Relationship Diagram (ERD)

```
┌──────────────────┐
│  organizations   │
├──────────────────┤
│ PK id            │
│    name          │
│    slug (unique) │
│    timezone      │
│    created_at    │
│    updated_at    │
└────────┬─────────┘
         │
         │ 1:N
         ▼
┌──────────────────┐         ┌──────────────────┐
│      users       │         │      teams       │
├──────────────────┤         ├──────────────────┤
│ PK id            │         │ PK id            │
│ FK organization_id────────►│ FK organization_id
│ FK team_id───────┼────────►│    name          │
│    email         │         │    description   │
│    password_hash │         │ FK manager_id────┤
│    first_name    │         │ FK work_schedule_id
│    last_name     │         │    created_at    │
│    phone         │         │    updated_at    │
│    role (ENUM)   │         └──────────────────┘
│ FK work_schedule_id
│    created_at    │         ┌──────────────────┐
│    updated_at    │         │  work_schedules  │
│    deleted_at    │         ├──────────────────┤
└────────┬─────────┘         │ PK id            │
         │                   │ FK organization_id
         │ 1:N               │    name          │
         ▼                   │    weekly_hours  │
┌──────────────────┐         │    tolerance_min │
│  refresh_tokens  │         │    is_default    │
├──────────────────┤         │    created_at    │
│ PK id            │         │    updated_at    │
│ FK user_id       │         └────────┬─────────┘
│    token_hash    │                  │
│    expires_at    │                  │ 1:N
│    revoked_at    │                  ▼
│    created_at    │         ┌──────────────────┐
└──────────────────┘         │work_schedule_days│
                             ├──────────────────┤
┌──────────────────┐         │ PK id            │
│  clock_entries   │         │ FK schedule_id   │
├──────────────────┤         │    day_of_week   │
│ PK id            │         │    start_time    │
│ FK organization_id         │    end_time      │
│ FK user_id       │         │    break_minutes │
│    date          │         └──────────────────┘
│    clock_in      │
│    clock_out     │         ┌──────────────────┐
│    duration_min  │         │  absence_types   │
│    is_manual     │         ├──────────────────┤
│    status (ENUM) │         │ PK id            │
│ FK approved_by   │         │ FK organization_id
│    notes         │         │    name          │
│    created_at    │         │    code (unique) │
│    updated_at    │         │    color         │
└──────────────────┘         │    requires_appr │
                             │    affects_bal   │
┌──────────────────┐         │    is_paid       │
│    absences      │         │    created_at    │
├──────────────────┤         └──────────────────┘
│ PK id            │
│ FK organization_id         ┌──────────────────┐
│ FK user_id       │         │  leave_balances  │
│ FK type_id───────┼────────►├──────────────────┤
│    start_date    │         │ PK id            │
│    end_date      │         │ FK organization_id
│    days_count    │         │ FK user_id       │
│    status (ENUM) │         │ FK absence_type_id
│    reason        │         │    year          │
│    rejection_reason        │    initial_bal   │
│ FK approved_by   │         │    used          │
│    approved_at   │         │    adjustment    │
│    created_at    │         │    remaining     │
│    updated_at    │         │    created_at    │
└──────────────────┘         │    updated_at    │
                             └──────────────────┘
┌──────────────────┐
│     holidays     │         ┌──────────────────┐
├──────────────────┤         │  notifications   │
│ PK id            │         ├──────────────────┤
│ FK organization_id         │ PK id            │
│    name          │         │ FK organization_id
│    date          │         │ FK user_id       │
│    is_recurring  │         │    type          │
│    created_at    │         │    title         │
└──────────────────┘         │    message       │
                             │    data (JSONB)  │
┌──────────────────┐         │    read_at       │
│   audit_logs     │         │    created_at    │
├──────────────────┤         └──────────────────┘
│ PK id            │
│ FK organization_id
│ FK user_id       │
│    action        │
│    entity_type   │
│    entity_id     │
│    old_values    │
│    new_values    │
│    ip_address    │
│    user_agent    │
│    created_at    │
└──────────────────┘
```

### 4.2 Enums

```sql
-- Rôles utilisateur (cascadés)
CREATE TYPE user_role AS ENUM ('employee', 'manager', 'admin', 'super_admin');

-- Statut pointage
CREATE TYPE clock_status AS ENUM ('pending', 'approved', 'rejected');

-- Statut absence
CREATE TYPE absence_status AS ENUM ('pending', 'approved', 'rejected', 'cancelled');
```

### 4.3 Règles Multi-tenant

- **Toutes les tables** ont `organization_id` (sauf `audit_logs` pour super_admin)
- **Toutes les requêtes** filtrent par `organization_id` du JWT
- **Un user = une organisation** (pas de cross-tenant)
- **Super Admin** peut accéder à toutes les organisations

---

## 5. Rôles et Permissions

### 5.1 Hiérarchie des Rôles (Cascade)

```
SUPER_ADMIN
    │ hérite tout de ↓
    ▼
  ADMIN
    │ hérite tout de ↓
    ▼
 MANAGER
    │ hérite tout de ↓
    ▼
EMPLOYEE
```

### 5.2 Matrice des Permissions

| Fonctionnalité | EMPLOYEE | MANAGER | ADMIN | SUPER_ADMIN |
|----------------|:--------:|:-------:|:-----:|:-----------:|
| **Profil personnel** | ✅ | ✅ | ✅ | ✅ |
| **Pointer** | ✅ | ✅ | ✅ | ✅ |
| **Voir ses pointages** | ✅ | ✅ | ✅ | ✅ |
| **Demander correction** | ✅ | ✅ | ✅ | ✅ |
| **Soumettre absence** | ✅ | ✅ | ✅ | ✅ |
| **Dashboard personnel** | ✅ | ✅ | ✅ | ✅ |
| Voir pointages équipe | ❌ | ✅* | ✅ | ✅ |
| Valider corrections | ❌ | ✅* | ✅ | ✅ |
| Valider absences | ❌ | ✅* | ✅ | ✅ |
| Dashboard équipe | ❌ | ✅* | ✅ | ✅ |
| Export équipe | ❌ | ✅* | ✅ | ✅ |
| CRUD utilisateurs | ❌ | ❌ | ✅ | ✅ |
| CRUD équipes | ❌ | ❌ | ✅ | ✅ |
| Config horaires | ❌ | ❌ | ✅ | ✅ |
| Config absences types | ❌ | ❌ | ✅ | ✅ |
| Config jours fériés | ❌ | ❌ | ✅ | ✅ |
| Dashboard global | ❌ | ❌ | ✅ | ✅ |
| Export paie | ❌ | ❌ | ✅ | ✅ |
| Config organisation | ❌ | ❌ | ❌ | ✅ |
| Audit logs | ❌ | ❌ | ❌ | ✅ |
| CRUD organisations | ❌ | ❌ | ❌ | ✅ |

*\* = uniquement pour sa propre équipe*

---

## 6. API REST

### 6.1 Convention

- **Base URL** : `/api/v1`
- **Format** : JSON
- **Auth** : Bearer token (header `Authorization`)
- **CSRF** : Double Submit Cookie (header `X-CSRF-Token`)

### 6.2 Réponses Standard

```json
// Succès
{
  "data": { ... },
  "meta": { "timestamp": "...", "request_id": "..." }
}

// Erreur
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Email invalide",
    "details": [{ "field": "email", "message": "Format invalide" }]
  }
}

// Pagination
{
  "data": [...],
  "meta": { "page": 1, "per_page": 20, "total": 150 }
}
```

### 6.3 Endpoints

#### Authentication (`/api/v1/auth`)

| Méthode | Endpoint | Description | Auth |
|---------|----------|-------------|:----:|
| POST | `/register` | Créer organisation + admin | Non |
| POST | `/login` | Obtenir tokens | Non |
| POST | `/refresh` | Renouveler access token | Cookie |
| POST | `/logout` | Révoquer token | Oui |
| POST | `/logout-all` | Révoquer tous les tokens | Oui |
| POST | `/forgot-password` | Demander reset | Non |
| POST | `/reset-password` | Reset avec token | Non |
| GET | `/me` | Profil utilisateur courant | Oui |
| PUT | `/me` | Modifier profil | Oui |
| PUT | `/me/password` | Changer mot de passe | Oui |

#### Users (`/api/v1/users`)

| Méthode | Endpoint | Description | Permission |
|---------|----------|-------------|------------|
| GET | `/` | Liste utilisateurs | Admin, Manager* |
| GET | `/:id` | Détail utilisateur | Admin, Manager*, Self |
| POST | `/` | Créer utilisateur | Admin |
| PUT | `/:id` | Modifier utilisateur | Admin |
| DELETE | `/:id` | Supprimer (soft) | Admin |
| PUT | `/:id/restore` | Restaurer | Admin |
| PUT | `/:id/role` | Changer rôle | Admin |
| PUT | `/:id/team` | Assigner équipe | Admin |

#### Teams (`/api/v1/teams`)

| Méthode | Endpoint | Description | Permission |
|---------|----------|-------------|------------|
| GET | `/` | Liste équipes | All |
| GET | `/:id` | Détail équipe | All |
| POST | `/` | Créer équipe | Admin |
| PUT | `/:id` | Modifier équipe | Admin |
| DELETE | `/:id` | Supprimer équipe | Admin |
| GET | `/:id/members` | Membres équipe | Admin, Manager* |
| PUT | `/:id/manager` | Assigner manager | Admin |

#### Organizations (`/api/v1/organizations`)

| Méthode | Endpoint | Description | Permission |
|---------|----------|-------------|------------|
| GET | `/current` | Organisation courante | All |
| PUT | `/current` | Modifier organisation | Admin |
| GET | `/current/settings` | Paramètres | Admin |
| PUT | `/current/settings` | Modifier paramètres | Admin |
| GET | `/` | Liste toutes | SuperAdmin |
| POST | `/` | Créer | SuperAdmin |

#### Schedules (`/api/v1/schedules`)

| Méthode | Endpoint | Description | Permission |
|---------|----------|-------------|------------|
| GET | `/` | Liste horaires | Admin, Manager |
| GET | `/templates` | Modèles horaires | Admin, Manager |
| GET | `/:id` | Détail horaire | All |
| POST | `/` | Créer horaire | Admin |
| PUT | `/:id` | Modifier horaire | Admin |
| DELETE | `/:id` | Supprimer | Admin |
| POST | `/:id/assign-team` | Assigner à équipe | Admin |
| POST | `/:id/assign-user` | Assigner à user | Admin, Manager |
| GET | `/users/:id/schedule` | Horaire effectif user | All*, Admin, Manager* |

#### Clock (`/api/v1/clock`)

| Méthode | Endpoint | Description | Permission |
|---------|----------|-------------|------------|
| GET | `/` | Historique pointages | All*, Admin, Manager* |
| GET | `/current` | Pointage actif | All |
| POST | `/in` | Pointer arrivée | All |
| POST | `/out` | Pointer départ | All |
| GET | `/:id` | Détail pointage | All*, Admin, Manager* |
| POST | `/:id/request-correction` | Demander correction | All* |
| POST | `/:id/approve` | Approuver correction | Admin, Manager* |
| POST | `/:id/reject` | Rejeter correction | Admin, Manager* |
| GET | `/pending-corrections` | Corrections en attente | Admin, Manager* |

#### Absences (`/api/v1/absences`)

| Méthode | Endpoint | Description | Permission |
|---------|----------|-------------|------------|
| GET | `/` | Liste absences | All*, Admin, Manager* |
| GET | `/:id` | Détail absence | All*, Admin, Manager* |
| POST | `/` | Créer demande | All |
| PUT | `/:id` | Modifier demande | All* |
| DELETE | `/:id` | Annuler demande | All*, Admin |
| POST | `/:id/approve` | Approuver | Admin, Manager* |
| POST | `/:id/reject` | Rejeter | Admin, Manager* |
| GET | `/pending` | En attente | Admin, Manager* |

#### Absence Types (`/api/v1/absence-types`)

| Méthode | Endpoint | Description | Permission |
|---------|----------|-------------|------------|
| GET | `/` | Liste types | All |
| POST | `/` | Créer type | Admin |
| PUT | `/:id` | Modifier type | Admin |
| DELETE | `/:id` | Supprimer type | Admin |

#### Leave Balances (`/api/v1/leave-balances`)

| Méthode | Endpoint | Description | Permission |
|---------|----------|-------------|------------|
| GET | `/` | Soldes utilisateur | All*, Admin, Manager* |
| PUT | `/:id` | Ajuster solde | Admin |

#### Holidays (`/api/v1/holidays`)

| Méthode | Endpoint | Description | Permission |
|---------|----------|-------------|------------|
| GET | `/` | Liste jours fériés | All |
| GET | `/:year` | Jours fériés année | All |
| POST | `/` | Créer jour férié | Admin |
| PUT | `/:id` | Modifier | Admin |
| DELETE | `/:id` | Supprimer | Admin |
| POST | `/import` | Importer liste | Admin |

#### Reports (`/api/v1/reports`)

| Méthode | Endpoint | Description | Permission |
|---------|----------|-------------|------------|
| GET | `/dashboard` | KPIs dashboard | All |
| GET | `/time-summary` | Résumé heures | All*, Admin, Manager* |
| GET | `/attendance` | Rapport présence | Admin, Manager* |
| GET | `/absences` | Rapport absences | Admin, Manager* |
| GET | `/overtime` | Rapport heures sup | Admin, Manager* |
| GET | `/punctuality` | Rapport ponctualité | Admin, Manager* |
| GET | `/team-overview` | Vue équipe | Admin, Manager* |
| POST | `/export` | Export CSV/PDF | Admin, Manager* |

#### Notifications (`/api/v1/notifications`)

| Méthode | Endpoint | Description | Permission |
|---------|----------|-------------|------------|
| GET | `/` | Liste notifications | All |
| GET | `/unread-count` | Nombre non lues | All |
| PUT | `/:id/read` | Marquer lue | All |
| PUT | `/read-all` | Tout marquer lu | All |
| DELETE | `/:id` | Supprimer | All |
| GET | `/preferences` | Préférences | All |
| PUT | `/preferences` | Modifier préf. | All |

#### Audit (`/api/v1/audit-logs`)

| Méthode | Endpoint | Description | Permission |
|---------|----------|-------------|------------|
| GET | `/` | Liste logs | SuperAdmin |
| GET | `/:id` | Détail log | SuperAdmin |
| GET | `/entity/:type/:id` | Historique entité | SuperAdmin |

*\* = uniquement ses propres données ou son équipe*

---

## 7. Authentification

### 7.1 Flow JWT

```
┌────────────┐     POST /auth/login      ┌─────────────┐
│   Client   │ ─────────────────────────►│   Backend   │
│            │     {email, password}      │             │
│            │                            │             │
│            │◄─────────────────────────  │             │
│            │     {access_token}         │             │
│            │     Set-Cookie: refresh_token (HttpOnly)
│            │     Set-Cookie: csrf_token │             │
└────────────┘                            └─────────────┘

Requête authentifiée:
Authorization: Bearer <access_token>
X-CSRF-Token: <csrf_token>
```

### 7.2 Tokens

**Access Token (15 minutes)**
```json
{
  "sub": "user-uuid",
  "org": "organization-uuid",
  "role": "employee",
  "type": "access",
  "iat": 1704067200,
  "exp": 1704068100
}
```

**Refresh Token (7 jours)**
- Stocké en HttpOnly cookie
- Hashé en base de données
- Rotation à chaque utilisation (ancien révoqué)

### 7.3 Protection CSRF

- Pattern : Double Submit Cookie
- Cookie `csrf_token` (SameSite=Strict)
- Header `X-CSRF-Token` doit correspondre
- Requis pour POST, PUT, DELETE

---

## 8. KPIs

### 8.1 Taux de Ponctualité

```
punctuality_rate = (jours_ponctuels / jours_travaillés) × 100
```

- Un jour est "ponctuel" si arrivée <= heure_début + tolérance
- Tolérance configurable (défaut: 10 min)
- Exclut jours fériés et absences

### 8.2 Écart Heures

```
variance = heures_travaillées - heures_théoriques
```

- Positif = heures supplémentaires
- Négatif = heures manquantes
- Prend en compte absences approuvées

---

## 9. Structure du Projet

### 9.1 Backend

```
backend/
├── Cargo.toml
├── Dockerfile
├── diesel.toml
├── .env.example
├── migrations/
│   └── [timestamps]_create_*/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config/
│   │   ├── mod.rs
│   │   ├── app.rs
│   │   └── database.rs
│   ├── schema.rs              # Diesel generated
│   ├── domain/                # Business entities
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   ├── team.rs
│   │   └── ...
│   ├── models/                # Database models
│   │   ├── mod.rs
│   │   └── ...
│   ├── repositories/          # Data access
│   │   ├── mod.rs
│   │   └── ...
│   ├── services/              # Business logic
│   │   ├── mod.rs
│   │   └── ...
│   ├── api/                   # HTTP handlers
│   │   ├── mod.rs
│   │   ├── router.rs
│   │   ├── auth/
│   │   ├── users/
│   │   └── ...
│   ├── middleware/
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   ├── tenant.rs
│   │   └── csrf.rs
│   ├── extractors/
│   │   ├── mod.rs
│   │   └── current_user.rs
│   ├── error/
│   │   ├── mod.rs
│   │   └── app_error.rs
│   └── utils/
│       ├── mod.rs
│       ├── jwt.rs
│       └── password.rs
└── tests/
    ├── common/
    └── integration/
```

### 9.2 Frontend

```
frontend/
├── package.json
├── tsconfig.json
├── vite.config.ts
├── tailwind.config.js
├── Dockerfile
├── .env.example
├── src/
│   ├── main.tsx
│   ├── App.tsx
│   ├── index.css
│   ├── config/
│   │   └── index.ts
│   ├── types/
│   │   ├── index.ts
│   │   ├── models.ts
│   │   └── api.ts
│   ├── api/
│   │   ├── client.ts          # Axios instance
│   │   ├── auth.ts
│   │   ├── users.ts
│   │   └── ...
│   ├── hooks/
│   │   ├── index.ts
│   │   ├── useAuth.ts
│   │   └── ...
│   ├── context/
│   │   └── AuthContext.tsx
│   ├── lib/
│   │   ├── utils.ts           # shadcn utils
│   │   └── validations.ts
│   ├── components/
│   │   ├── ui/                # shadcn components
│   │   ├── layout/
│   │   │   ├── AppLayout.tsx
│   │   │   ├── Sidebar.tsx
│   │   │   └── Header.tsx
│   │   ├── auth/
│   │   ├── users/
│   │   ├── teams/
│   │   ├── clock/
│   │   ├── absences/
│   │   ├── reports/
│   │   └── shared/
│   ├── pages/
│   │   ├── auth/
│   │   ├── dashboard/
│   │   ├── users/
│   │   ├── teams/
│   │   ├── clock/
│   │   ├── absences/
│   │   ├── reports/
│   │   └── settings/
│   └── routes/
│       ├── index.tsx
│       └── ProtectedRoute.tsx
└── tests/
```

### 9.3 Infrastructure

```
time-manager/
├── docker-compose.yml         # Development
├── docker-compose.prod.yml    # Production
├── .env.example
├── infrastructure/
│   ├── traefik/
│   │   └── traefik.yml
│   ├── postgres/
│   │   └── init.sql
│   ├── grafana/
│   │   └── provisioning/
│   ├── prometheus/
│   │   └── prometheus.yml
│   ├── loki/
│   ├── tempo/
│   └── otel-collector/
├── backend/
├── frontend/
└── docs/
```

---

## 10. Tests

### 10.1 Backend

- **Unit tests** : Services, utils
- **Integration tests** : API endpoints avec Testcontainers
- **Couverture cible** : > 80%

```bash
cargo test                    # All tests
cargo test --test integration # Integration only
cargo llvm-cov               # Coverage report
```

### 10.2 Frontend

- **Unit tests** : Components, hooks
- **Integration tests** : Pages
- **Couverture cible** : > 60%

```bash
npm run test         # Vitest
npm run test:coverage
```

---

## 11. CI/CD

### 11.1 Pipeline GitHub Actions

```yaml
# Sur PR et push vers main/develop
jobs:
  backend-lint:    # clippy, rustfmt
  backend-test:    # cargo test
  frontend-lint:   # eslint, tsc
  frontend-test:   # vitest
  docker-build:    # Smoke test images

# Quality Gates:
- Tests passent à 100%
- Coverage backend > 80%
- Coverage frontend > 60%
- Pas de warnings clippy
- Pas d'erreurs ESLint
```

---

## 12. Critères Epitech

| Critère | Implémentation |
|---------|----------------|
| `dockerfiles` | Multi-stage builds backend/frontend |
| `containers` | 4+ services isolés |
| `persistency` | Volumes PostgreSQL, Grafana |
| `orchestration` | Docker Compose |
| `clean_deploy` | compose.yml + compose.prod.yml |
| `env_specificity` | .env par environnement |
| `secrets` | Variables d'env, pas en clair |
| `api_crafting` | REST versionnée |
| `data_persist` | Diesel + PostgreSQL |
| `framework_back` | Axum (justifié) |
| `auth_jwt` | Access + Refresh tokens |
| `auth_persist` | Refresh en BDD |
| `auth_sec` | CSRF, XSS, Argon2 |
| `roles` | 4 rôles cascadés |
| `data_viz` | Recharts |
| `api_consumption` | TanStack Query |
| `code_orga` | Structure par feature |
| `uiux_quality` | Shadcn/UI + a11y |
| `hmi` | Responsive |
| `framework_front` | React 18 (justifié) |
| `robustness` | Error boundaries |
| `maintainability` | Clean Architecture |
| `tests_sequence` | Tests automatisés |
| `tests_coverage` | Rapports couverture |
| `tests_automation` | CI/CD |
| `ci_pipeline` | GitHub Actions |
| `ci_quality` | Quality gates |
| `versioning_basics` | Git flow |
| `doc_basic` | Ce document |

---

## Annexes

### A. Variables d'Environnement

```env
# Database
DATABASE_URL=postgres://user:pass@host:5432/db

# JWT
JWT_SECRET=your-256-bit-secret
JWT_ACCESS_EXPIRY=900
JWT_REFRESH_EXPIRY=604800

# SMTP
SMTP_HOST=smtp.example.com
SMTP_PORT=587
SMTP_USER=
SMTP_PASS=
SMTP_FROM=noreply@example.com

# App
APP_ENV=development|production
APP_URL=http://localhost:3000
API_URL=http://localhost:8080
```

### B. Codes Erreur API

| Code | Description |
|------|-------------|
| `VALIDATION_ERROR` | Données invalides |
| `UNAUTHORIZED` | Token manquant/invalide |
| `FORBIDDEN` | Permission refusée |
| `NOT_FOUND` | Ressource introuvable |
| `CONFLICT` | Conflit (ex: email existant) |
| `INTERNAL_ERROR` | Erreur serveur |
