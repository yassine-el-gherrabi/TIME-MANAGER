# TM-E13 : Tests & Qualité

## Informations Epic

| Champ | Valeur |
|-------|--------|
| **ID** | TM-E13 |
| **Titre** | Tests & Qualité |
| **Priorité** | P1 - Haute (Continue) |
| **Estimation globale** | 24 SP |
| **Sprint cible** | Tous les sprints |
| **Dépendances** | TM-E01 (Infrastructure) |

---

## Description

### Contexte

La qualité du code et la couverture de tests sont des critères d'évaluation importants du projet Epitech. Cette Epic définit la stratégie de tests, les outils de qualité, et les seuils à atteindre. Les tests sont développés en parallèle des fonctionnalités, pas à la fin.

### Objectif Business

Garantir la fiabilité et la maintenabilité de l'application à travers une suite de tests automatisés complète, permettant des déploiements en confiance et une régression minimale.

### Valeur Apportée

- **Pour les développeurs** : Confiance dans les modifications, documentation vivante
- **Pour le projet** : Réduction des bugs en production, déploiements sereins
- **Pour Epitech** : Critère tests_coverage validé (>80% backend, >60% frontend)

---

## Scope

### Inclus

- Tests unitaires backend (Rust) avec cargo-nextest
- Tests d'intégration backend (API) avec testcontainers
- Tests unitaires frontend (React)
- Tests de composants frontend
- Configuration CI avec checks automatiques
- Linting et formatting automatisés
- Couverture de code mesurée
- Audit de sécurité automatisé (cargo-audit, cargo-deny)
- Analyse des dépendances (cargo-machete)

### Exclus

- Tests E2E navigateur (Cypress/Playwright) - v2
- Tests de performance/charge
- Tests de sécurité automatisés (pentesting)
- Mutation testing

---

## Critères de Succès de l'Epic

- [ ] Couverture backend ≥ 80%
- [ ] Couverture frontend ≥ 60%
- [ ] CI vérifie tous les tests avant merge
- [ ] Linting passe sans erreur
- [ ] Aucun test flaky (instable)
- [ ] Temps d'exécution CI < 10 minutes

---

## User Stories

---

### TM-87 : Configuration tests backend

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur backend,
**Je veux** un environnement de tests configuré,
**Afin de** écrire et exécuter des tests facilement.

#### Contexte Détaillé

Rust utilise son système de tests intégré. Configuration nécessaire :
- Base de données de test (PostgreSQL dédié ou TestContainers)
- Fixtures et factories pour les données
- Mocks pour services externes
- Organisation des tests (unit vs integration)

#### Critères d'Acceptation

- [ ] `cargo nextest run` fonctionne (nextest installé et configuré)
- [ ] Base de données de test isolée via testcontainers
- [ ] Variables d'environnement de test (.env.test)
- [ ] Module de fixtures créé (users, teams, etc.)
- [ ] Helper pour setup/teardown de la base
- [ ] Tests parallèles activés avec isolation
- [ ] Couverture mesurable avec `cargo-tarpaulin`
- [ ] Timeout par test: 60s max
- [ ] Retry tests flaky: 2 tentatives automatiques
- [ ] Output formaté avec couleurs et résumé

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-87.1 | Installer et configurer cargo-nextest | 0.5h |
| TM-87.2 | Configurer testcontainers pour PostgreSQL | 1h |
| TM-87.3 | Créer .env.test et configuration | 0.5h |
| TM-87.4 | Créer module fixtures avec factories | 2h |
| TM-87.5 | Configurer couverture avec tarpaulin | 1h |
| TM-87.6 | Configurer timeouts et retries | 0.5h |
| TM-87.7 | Documenter stratégie de tests | 0.5h |

---

### TM-88 : Tests unitaires backend

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 5 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur backend,
**Je veux** des tests unitaires pour la logique métier,
**Afin de** valider le comportement des services isolément.

#### Contexte Détaillé

Les tests unitaires couvrent :
- Services (calculs, validations, transformations)
- Fonctions utilitaires
- Validation des DTOs
- Logique de permissions

Ils mockent les dépendances (repositories, services externes).

#### Critères d'Acceptation

- [ ] Tests pour AuthService (validation tokens, hashing)
- [ ] Tests pour UserService (CRUD, validations)
- [ ] Tests pour ClockService (calcul durées, validations)
- [ ] Tests pour AbsenceService (jours ouvrés, soldes)
- [ ] Tests pour ReportService (calculs KPIs)
- [ ] Tests pour permissions et guards
- [ ] Couverture des services ≥ 80%

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-88.1 | Tests AuthService | 2h |
| TM-88.2 | Tests UserService | 2h |
| TM-88.3 | Tests ClockService | 2h |
| TM-88.4 | Tests AbsenceService | 3h |
| TM-88.5 | Tests ReportService | 2h |
| TM-88.6 | Tests permissions | 1h |
| TM-88.7 | Atteindre couverture 80% | 2h |

---

### TM-89 : Tests d'intégration backend

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 5 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur backend,
**Je veux** des tests d'intégration pour les endpoints API,
**Afin de** valider le comportement complet de l'API.

#### Contexte Détaillé

Les tests d'intégration :
- Appellent les vrais endpoints HTTP
- Utilisent une vraie base de données (de test)
- Vérifient les codes de réponse, headers, body
- Testent l'authentification et les permissions
- Testent l'isolation multi-tenant

#### Critères d'Acceptation

- [ ] Tests endpoints auth (register, login, refresh, logout)
- [ ] Tests endpoints users CRUD
- [ ] Tests endpoints teams CRUD
- [ ] Tests endpoints clock (in, out, history)
- [ ] Tests endpoints absences (workflow complet)
- [ ] Tests isolation tenant (cross-org = 404)
- [ ] Tests permissions (403 si non autorisé)
- [ ] Chaque endpoint a au moins un test succès et un test erreur

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-89.1 | Créer helper client HTTP de test | 1h |
| TM-89.2 | Tests intégration auth | 2h |
| TM-89.3 | Tests intégration users | 2h |
| TM-89.4 | Tests intégration teams | 1.5h |
| TM-89.5 | Tests intégration clock | 2h |
| TM-89.6 | Tests intégration absences | 2.5h |
| TM-89.7 | Tests isolation multi-tenant | 1.5h |
| TM-89.8 | Tests permissions par rôle | 1.5h |

---

### TM-90 : Configuration tests frontend

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur frontend,
**Je veux** un environnement de tests configuré,
**Afin de** écrire et exécuter des tests React.

#### Contexte Détaillé

Stack de tests frontend :
- Vitest (compatible Vite, rapide)
- React Testing Library (tests orientés utilisateur)
- MSW (Mock Service Worker) pour mocker l'API
- Testing Library User Event pour interactions

#### Critères d'Acceptation

- [ ] Vitest configuré et fonctionnel
- [ ] React Testing Library installé
- [ ] MSW configuré avec handlers de base
- [ ] Setup file avec providers (QueryClient, Router)
- [ ] `npm test` fonctionne
- [ ] `npm run test:coverage` génère un rapport
- [ ] Watch mode disponible

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-90.1 | Installer et configurer Vitest | 1h |
| TM-90.2 | Configurer React Testing Library | 0.5h |
| TM-90.3 | Configurer MSW avec handlers API | 2h |
| TM-90.4 | Créer setup file avec providers | 1h |
| TM-90.5 | Configurer couverture de code | 0.5h |

---

### TM-91 : Tests composants frontend

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 5 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur frontend,
**Je veux** des tests pour les composants React,
**Afin de** valider leur rendu et comportement.

#### Contexte Détaillé

Approche : tester le comportement utilisateur, pas l'implémentation.
- Rendu correct avec différentes props
- Interactions (clics, saisie)
- États (loading, error, success)
- Accessibilité basique

#### Critères d'Acceptation

- [ ] Tests composants layout (Header, Sidebar, MainLayout)
- [ ] Tests composants formulaires
- [ ] Tests composants DataTable
- [ ] Tests composants modales
- [ ] Tests pages principales (Dashboard, Login)
- [ ] Tests hooks custom (useAuth, useToast)
- [ ] Couverture composants ≥ 60%

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-91.1 | Tests composants layout | 2h |
| TM-91.2 | Tests composants formulaires | 2h |
| TM-91.3 | Tests DataTable | 1.5h |
| TM-91.4 | Tests modales | 1h |
| TM-91.5 | Tests pages auth | 2h |
| TM-91.6 | Tests Dashboard | 2h |
| TM-91.7 | Tests hooks | 1.5h |
| TM-91.8 | Atteindre couverture 60% | 2h |

---

### TM-92 : Linting, formatting et audit sécurité

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur,
**Je veux** des outils de linting, formatting et audit automatiques,
**Afin de** maintenir un code cohérent et sécurisé.

#### Critères d'Acceptation

- [ ] Backend : `cargo fmt --check` et `cargo clippy -D warnings` configurés
- [ ] Backend : `cargo audit` pour vulnérabilités (RUSTSEC)
- [ ] Backend : `cargo deny` pour licences et dépendances
- [ ] Backend : `cargo machete` pour dépendances inutilisées
- [ ] Backend : `cargo sort` pour tri Cargo.toml
- [ ] Frontend : ESLint + Prettier configurés
- [ ] Frontend : `npm audit` intégré
- [ ] Règles cohérentes avec les bonnes pratiques
- [ ] Scripts npm : `lint`, `lint:fix`, `format`
- [ ] Pre-commit hook optionnel (Husky)
- [ ] CI vérifie le linting ET les audits de sécurité

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-92.1 | Configurer clippy avec `-D warnings` | 0.5h |
| TM-92.2 | Installer et configurer cargo-audit | 0.5h |
| TM-92.3 | Installer et configurer cargo-deny | 0.5h |
| TM-92.4 | Installer et configurer cargo-machete | 0.5h |
| TM-92.5 | Installer et configurer cargo-sort | 0.5h |
| TM-92.6 | Configurer ESLint React/TypeScript | 1h |
| TM-92.7 | Configurer Prettier | 0.5h |
| TM-92.8 | Créer scripts npm | 0.5h |
| TM-92.9 | Configurer pre-commit hooks (optionnel) | 0.5h |

---

### TM-93 : CI Pipeline tests

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur,
**Je veux** une CI qui exécute les tests automatiquement,
**Afin de** valider chaque PR avant merge.

#### Critères d'Acceptation

- [ ] GitHub Actions workflow configuré
- [ ] Jobs : lint-backend, test-backend, lint-frontend, test-frontend
- [ ] Tests backend avec base PostgreSQL (service container)
- [ ] Upload des rapports de couverture
- [ ] Badge de couverture dans README
- [ ] Temps total < 10 minutes
- [ ] Cache des dépendances (Cargo, npm)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-93.1 | Créer workflow test-backend.yml | 1.5h |
| TM-93.2 | Créer workflow test-frontend.yml | 1h |
| TM-93.3 | Configurer service PostgreSQL | 0.5h |
| TM-93.4 | Configurer upload coverage | 1h |
| TM-93.5 | Optimiser avec cache | 0.5h |
| TM-93.6 | Ajouter badges README | 0.5h |

---

### TM-94 : Documentation tests

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P2 |
| **Estimation** | 1 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur,
**Je veux** une documentation sur les tests,
**Afin de** savoir comment écrire et exécuter les tests.

#### Critères d'Acceptation

- [ ] README tests backend : conventions, commandes, fixtures
- [ ] README tests frontend : conventions, commandes, mocks
- [ ] Exemples de tests pour chaque type
- [ ] Guide pour ajouter des mocks MSW
- [ ] Seuils de couverture documentés

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-94.1 | Rédiger docs tests backend | 1h |
| TM-94.2 | Rédiger docs tests frontend | 1h |
| TM-94.3 | Créer exemples annotés | 0.5h |

---

## Récapitulatif des Estimations

| Story | Titre | SP |
|-------|-------|:--:|
| TM-87 | Configuration tests backend | 2 |
| TM-88 | Tests unitaires backend | 5 |
| TM-89 | Tests d'intégration backend | 5 |
| TM-90 | Configuration tests frontend | 2 |
| TM-91 | Tests composants frontend | 5 |
| TM-92 | Linting, formatting et audit sécurité | 2 |
| TM-93 | CI Pipeline tests | 2 |
| TM-94 | Documentation tests | 1 |
| **Total** | | **24 SP** |

---

## Notes Techniques

### Stack Tests Backend (Rust)

```toml
# Cargo.toml
[dev-dependencies]
tokio-test = "0.4"
mockall = "0.11"            # Mocking
fake = "2.5"                # Fake data generation
wiremock = "0.5"            # HTTP mocking
serial_test = "2.0"         # Tests séquentiels si besoin
testcontainers = "0.15"     # Conteneurs Docker pour tests
testcontainers-modules = { version = "0.3", features = ["postgres"] }
```

### Outils CI Rust (installation globale)

```bash
# Installation des outils de qualité
cargo install cargo-nextest       # Tests parallèles rapides
cargo install cargo-tarpaulin     # Couverture de code
cargo install cargo-audit         # Audit vulnérabilités RUSTSEC
cargo install cargo-deny          # Audit licences et dépendances
cargo install cargo-machete       # Détection deps inutilisées
cargo install cargo-sort          # Tri alphabétique Cargo.toml
```

### Configuration Nextest

```toml
# .config/nextest.toml
[profile.default]
retries = 2                    # Retry tests flaky
slow-timeout = { period = "60s" }
fail-fast = false

[profile.ci]
retries = 2
fail-fast = true
```

### Configuration Cargo Deny

```toml
# deny.toml
[advisories]
vulnerability = "deny"
unmaintained = "warn"

[licenses]
allow = [
    "MIT",
    "Apache-2.0",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "ISC",
    "Zlib",
]
copyleft = "warn"
unlicensed = "deny"

[bans]
multiple-versions = "warn"
wildcards = "deny"
```

### Stack Tests Frontend

```json
// package.json
{
  "devDependencies": {
    "vitest": "^1.0.0",
    "@testing-library/react": "^14.0.0",
    "@testing-library/user-event": "^14.5.0",
    "@testing-library/jest-dom": "^6.1.0",
    "msw": "^2.0.0",
    "@vitest/coverage-v8": "^1.0.0"
  }
}
```

### Structure Tests Backend

```
back/
├── src/
│   ├── services/
│   │   ├── user_service.rs
│   │   └── user_service_test.rs  # Tests unitaires
│   └── ...
└── tests/
    ├── common/
    │   ├── mod.rs
    │   ├── fixtures.rs
    │   └── helpers.rs
    ├── auth_test.rs              # Tests intégration
    ├── users_test.rs
    └── ...
```

### Structure Tests Frontend

```
front/
├── src/
│   ├── components/
│   │   ├── Header.tsx
│   │   └── Header.test.tsx       # Tests unitaires
│   └── ...
└── tests/
    ├── setup.ts                  # Setup global
    ├── mocks/
    │   ├── handlers.ts           # MSW handlers
    │   └── server.ts
    └── utils/
        └── render.tsx            # Custom render
```

### Conventions de Nommage Tests

| Type | Pattern | Exemple |
|------|---------|---------|
| Fichier test Rust | `*_test.rs` | `user_service_test.rs` |
| Fichier test TS | `*.test.tsx` | `Header.test.tsx` |
| Describe | Nom du module/composant | `describe('UserService')` |
| Test | `it should + comportement` | `it('should create user with valid data')` |

### Seuils de Couverture

| Cible | Backend | Frontend |
|-------|---------|----------|
| Minimum requis | 80% | 60% |
| Objectif | 85% | 70% |
| Idéal | 90% | 80% |

### CI Workflows

#### Lint & Build (BLOQUANT)

```yaml
# .github/workflows/ci.yml
name: CI
on: [push, pull_request]

jobs:
  backend-lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - uses: Swatinem/rust-cache@v2
      - name: Check formatting
        run: cargo fmt --all --check
      - name: Clippy
        run: cargo clippy --all --all-targets -- -D warnings
      - name: Check
        run: cargo check --all-features

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
      - run: npm ci
      - run: npm run lint
      - run: npm run type-check

  frontend-build:
    runs-on: ubuntu-latest
    needs: frontend-lint
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'
      - run: npm ci
      - run: npm run build
```

#### Tests (BLOQUANT)

```yaml
# .github/workflows/test.yml
name: Tests
on: [push, pull_request]

jobs:
  test-backend:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:16
        env:
          POSTGRES_PASSWORD: test
        ports:
          - 5432:5432
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - name: Install nextest
        uses: taiki-e/install-action@nextest
      - name: Run tests with nextest
        run: cargo nextest run --all --profile ci
      - name: Coverage
        run: cargo tarpaulin --out Xml --skip-clean
      - uses: codecov/codecov-action@v3
        with:
          fail_ci_if_error: true
          threshold: 80

  test-frontend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'
      - run: npm ci
      - run: npm run test:coverage
      - uses: codecov/codecov-action@v3
        with:
          fail_ci_if_error: true
          threshold: 60
```

#### Security Audits (BLOQUANT)

```yaml
# .github/workflows/security.yml
name: Security Audits
on:
  push:
    branches: [main, master]
  pull_request:
  schedule:
    - cron: '0 6 * * 1'  # Hebdomadaire

jobs:
  audit-cargo:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Audit Cargo dependencies
        uses: rustsec/audit-check@v1
        with:
          token: ${{ secrets.GITHUB_TOKEN }}

  deny-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: EmbarkStudios/cargo-deny-action@v1
        with:
          command: check all

  audit-npm:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
      - run: npm ci
      - run: npm audit --audit-level=moderate
```

### Critères Epitech Couverts

| Critère | Implémentation |
|---------|----------------|
| tests_coverage | Backend ≥ 80%, Frontend ≥ 60% |
| ci_pipeline | GitHub Actions avec tests obligatoires |
