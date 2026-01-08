# 📋 Checklist Complète de Tests E2E - Time Manager

**Date**: 2026-01-07
**Mode**: Dev (hot reload + monitoring + Traefik)
**Architecture**: Unifiée Dev/Prod

---

## 🚀 Avant de Commencer

### Prérequis
```bash
# Démarrer tous les services
task dev:build

# Vérifier que tous les conteneurs sont UP
docker compose ps

# Tous les services doivent être "Up" (8 services au total)
```

### Variables d'Environnement
Vérifier que `.env.dev` est bien chargé :
- `VITE_API_URL=http://localhost:8000/api/v1`
- `CORS_ALLOWED_ORIGINS=http://localhost:8000,http://localhost`
- `JWT_SECRET` configuré
- `DATABASE_URL` configuré

---

## 🏗️ 1. Tests Infrastructure & Docker

### 1.1 Services Docker
```bash
# Test: Vérifier tous les conteneurs actifs
docker compose ps

# Attendu: 8 services "Up"
✅ timemanager-backend (Up)
✅ timemanager-frontend (Up)
✅ timemanager-postgres (Up, healthy)
✅ timemanager-traefik (Up)
✅ timemanager-grafana (Up)
✅ timemanager-prometheus (Up)
✅ timemanager-loki (Up)
✅ timemanager-mailpit (Up, healthy)
```

### 1.2 Réseau & Connectivité
```bash
# Test: Vérifier le réseau Docker
docker network ls | grep timemanager

# Attendu: timemanager_timemanager-network existe
```

### 1.3 Volumes Persistants
```bash
# Test: Vérifier les volumes
docker volume ls | grep timemanager

# Attendu: 6 volumes
✅ postgres_data
✅ prometheus_data
✅ loki_data
✅ grafana_data
✅ cargo_cache
✅ cargo_target
```

---

## 🔀 2. Tests Traefik (Reverse Proxy)

### 2.1 Dashboard Traefik
```bash
# URL: http://localhost:8081/dashboard/
curl -I http://localhost:8081/dashboard/

# Attendu: HTTP/1.1 200 OK
```

**Test Manuel**:
1. Ouvrir http://localhost:8081/dashboard/ dans le navigateur
2. ✅ Vérifier la présence de 2 routers:
   - `backend@docker`: PathPrefix(`/api`)
   - `frontend@docker`: PathPrefix(`/`)
3. ✅ Vérifier 2 services actifs
4. ✅ Vérifier les middlewares configurés:
   - `rate-limit-global@file`
   - `security-headers@file`
   - `cors-headers@file`
   - `strip-api-prefix@file`

### 2.2 Routing Frontend via Traefik
```bash
# Test: Frontend accessible via Traefik port 8000
curl -I http://localhost:8000

# Attendu: HTTP/1.1 200 OK
# Attendu: Content-Type: text/html
```

**Test Manuel**:
1. Ouvrir http://localhost:8000
2. ✅ Page se charge sans erreur
3. ✅ Pas d'erreur CORS dans la console
4. ✅ Assets (CSS/JS) se chargent correctement

### 2.3 Routing Backend via Traefik
```bash
# Test: Backend accessible via Traefik /api prefix
curl http://localhost:8000/api/health

# Attendu:
{
  "status": "ok",
  "version": "0.1.0",
  "timestamp": <unix_timestamp>
}
```

### 2.4 Strip Prefix Middleware
```bash
# Test: Vérifier que /api est bien enlevé avant backend
curl -v http://localhost:8000/api/health 2>&1 | grep "GET"

# Le backend reçoit: GET /health (pas GET /api/health)
# Vérifier dans les logs backend:
docker compose logs backend --tail 5
```

### 2.5 Rate Limiting
```bash
# Test: Déclencher rate limit (100 req/min)
for i in {1..110}; do curl -s http://localhost:8000/api/health > /dev/null; done

# Attendu: Certaines requêtes reçoivent HTTP 429 Too Many Requests
```

### 2.6 Security Headers
```bash
# Test: Vérifier headers de sécurité
curl -I http://localhost:8000

# Attendu:
X-Content-Type-Options: nosniff
X-Frame-Options: DENY
Referrer-Policy: strict-origin-when-cross-origin
Permissions-Policy: geolocation=(), microphone=(), camera=()
```

### 2.7 CORS Configuration
```bash
# Test: CORS headers présents
curl -I -X OPTIONS http://localhost:8000/api/health \
  -H "Origin: http://localhost:8000"

# Attendu:
Access-Control-Allow-Origin: http://localhost:8000
Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS
Access-Control-Allow-Credentials: true
```

---

## 🗄️ 3. Tests Base de Données (PostgreSQL)

### 3.1 Connexion PostgreSQL
```bash
# Test: Connexion à la DB
docker compose exec postgres psql -U timemanager -d timemanager -c "SELECT version();"

# Attendu: PostgreSQL 16.x
```

### 3.2 Tables Diesel Migrations
```bash
# Test: Vérifier les migrations
docker compose exec postgres psql -U timemanager -d timemanager -c "\dt"

# Attendu: Liste des tables (si migrations exécutées)
```

### 3.3 Healthcheck PostgreSQL
```bash
# Test: Vérifier healthcheck
docker inspect timemanager-postgres | grep -A 5 "Health"

# Attendu: "Status": "healthy"
```

### 3.4 Connexion Backend → PostgreSQL
```bash
# Test: Vérifier les logs backend pour connexion DB
docker compose logs backend | grep -i "database\|postgres"

# Attendu: Pas d'erreur de connexion
```

---

## 🔐 4. Tests Backend API

### 4.1 Health Check
```bash
# Test 1: Via Traefik (port 8000)
curl http://localhost:8000/api/health | jq

# Attendu:
{
  "status": "ok",
  "version": "0.1.0",
  "timestamp": 1767782092
}

# Test 2: Direct backend (port 8080 - non exposé)
docker compose exec backend wget -q -O- http://localhost:8080/health

# Attendu: Même réponse JSON
```

### 4.2 Endpoints Auth (À TESTER quand implémentés)

⚠️ **NOTE**: Ces endpoints existent dans le code mais ne sont **PAS ENCORE MONTÉS** dans le router.
À tester après implémentation dans `backend/src/api/router.rs`

#### 4.2.1 Register (POST /api/v1/auth/register)
```bash
# Test: Créer un nouvel utilisateur
curl -X POST http://localhost:8000/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "Password123!"
  }'

# Attendu:
{
  "access_token": "<jwt_token>",
  "refresh_token": "<refresh_token>",
  "user": {
    "id": 1,
    "username": "testuser",
    "email": "test@example.com"
  }
}
```

**Cas d'erreur à tester**:
- Email déjà existant → 409 Conflict
- Mot de passe faible → 400 Bad Request
- Email invalide → 400 Bad Request
- Champs manquants → 400 Bad Request

#### 4.2.2 Login (POST /api/v1/auth/login)
```bash
# Test: Connexion utilisateur
curl -X POST http://localhost:8000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "Password123!"
  }'

# Attendu: Même structure que register
```

**Cas d'erreur à tester**:
- Email inexistant → 401 Unauthorized
- Mot de passe incorrect → 401 Unauthorized
- Compte désactivé → 403 Forbidden

#### 4.2.3 Me (GET /api/v1/auth/me)
```bash
# Test: Récupérer profil utilisateur connecté
curl http://localhost:8000/api/v1/auth/me \
  -H "Authorization: Bearer <access_token>"

# Attendu:
{
  "id": 1,
  "username": "testuser",
  "email": "test@example.com",
  "role": "user",
  "created_at": "2026-01-07T10:00:00Z"
}
```

**Cas d'erreur à tester**:
- Token manquant → 401 Unauthorized
- Token expiré → 401 Unauthorized
- Token invalide → 401 Unauthorized

#### 4.2.4 Refresh (POST /api/v1/auth/refresh)
```bash
# Test: Rafraîchir access token
curl -X POST http://localhost:8000/api/v1/auth/refresh \
  -H "Content-Type: application/json" \
  -d '{
    "refresh_token": "<refresh_token>"
  }'

# Attendu:
{
  "access_token": "<new_jwt_token>",
  "refresh_token": "<new_refresh_token>"
}
```

#### 4.2.5 Logout (POST /api/v1/auth/logout)
```bash
# Test: Déconnexion (invalide session)
curl -X POST http://localhost:8000/api/v1/auth/logout \
  -H "Authorization: Bearer <access_token>"

# Attendu: 204 No Content
```

#### 4.2.6 Logout All (POST /api/v1/auth/logout-all)
```bash
# Test: Déconnexion de toutes les sessions
curl -X POST http://localhost:8000/api/v1/auth/logout-all \
  -H "Authorization: Bearer <access_token>"

# Attendu: 204 No Content
```

### 4.3 Endpoints Password Reset

#### 4.3.1 Request Reset (POST /api/v1/password/request-reset)
```bash
# Test: Demander reset password
curl -X POST http://localhost:8000/api/v1/password/request-reset \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com"
  }'

# Attendu:
{
  "message": "Si cet email existe, un lien de réinitialisation a été envoyé"
}
```

**À vérifier**:
1. Email reçu dans Mailpit (http://localhost:8025)
2. Token présent dans l'email
3. Lien valide avec token

#### 4.3.2 Reset Password (POST /api/v1/password/reset)
```bash
# Test: Réinitialiser password avec token
curl -X POST http://localhost:8000/api/v1/password/reset \
  -H "Content-Type: application/json" \
  -d '{
    "token": "<reset_token>",
    "new_password": "NewPassword123!"
  }'

# Attendu:
{
  "message": "Mot de passe réinitialisé avec succès"
}
```

### 4.4 Endpoints Sessions (GET /api/v1/sessions)
```bash
# Test: Lister les sessions actives
curl http://localhost:8000/api/v1/sessions \
  -H "Authorization: Bearer <access_token>"

# Attendu:
[
  {
    "id": "session_id",
    "device": "Chrome on Mac",
    "ip": "127.0.0.1",
    "created_at": "2026-01-07T10:00:00Z",
    "last_active": "2026-01-07T10:30:00Z"
  }
]
```

---

## 🎨 5. Tests Frontend (React + Vite)

### 5.1 Build & Hot Reload
```bash
# Test: Vérifier que Vite dev server tourne
docker compose logs frontend --tail 10

# Attendu:
# VITE v5.x.x  ready in X ms
# ➜  Local:   http://localhost:3000/
# ➜  Network: use --host to expose
```

**Test Manuel - Hot Reload**:
1. Ouvrir http://localhost:8000
2. Éditer `frontend/src/App.tsx` (changer un texte)
3. Sauvegarder
4. ✅ Page se recharge automatiquement
5. ✅ Changement visible sans rebuild

### 5.2 Routing Frontend

#### 5.2.1 Page Login (/)
```bash
# Test: Accès page login
curl -I http://localhost:8000/login

# Attendu: HTTP/1.1 200 OK
```

**Test Manuel**:
1. Naviguer vers http://localhost:8000/login
2. ✅ Formulaire login visible
3. ✅ Champs: Email, Password
4. ✅ Bouton "Se connecter"
5. ✅ Lien "Créer un compte" → /register
6. ✅ Lien "Mot de passe oublié?" → /password-reset-request

#### 5.2.2 Page Register (/register)
**Test Manuel**:
1. Naviguer vers http://localhost:8000/register
2. ✅ Formulaire inscription visible
3. ✅ Champs: Username, Email, Password, Confirm Password
4. ✅ Bouton "Créer un compte"
5. ✅ Lien "Déjà un compte?" → /login
6. ✅ Validation côté client (password matching)

#### 5.2.3 Page Password Reset Request (/password-reset-request)
**Test Manuel**:
1. Naviguer vers http://localhost:8000/password-reset-request
2. ✅ Formulaire demande reset visible
3. ✅ Champ: Email
4. ✅ Bouton "Envoyer le lien"
5. ✅ Lien retour vers /login

#### 5.2.4 Page Password Reset (/password-reset?token=xxx)
**Test Manuel**:
1. Naviguer vers http://localhost:8000/password-reset?token=test123
2. ✅ Formulaire reset password visible
3. ✅ Champs: New Password, Confirm Password
4. ✅ Bouton "Réinitialiser"
5. ✅ Validation côté client (password matching)

#### 5.2.5 Page Dashboard (/)
**Test Manuel** (nécessite authentification):
1. Se connecter d'abord via /login
2. Redirection vers http://localhost:8000/
3. ✅ Dashboard visible
4. ✅ Nom utilisateur affiché
5. ✅ Navigation disponible

#### 5.2.6 Page Unauthorized (/unauthorized)
**Test Manuel**:
1. Naviguer vers http://localhost:8000/unauthorized
2. ✅ Message "Non autorisé" visible
3. ✅ Lien retour vers /login

#### 5.2.7 Redirections
**Test Manuel**:
1. Naviguer vers http://localhost:8000/page-inexistante
2. ✅ Redirection vers / (dashboard ou login selon auth)

### 5.3 Protected Routes

**Test Manuel - Utilisateur NON connecté**:
1. Effacer localStorage/cookies
2. Naviguer vers http://localhost:8000/
3. ✅ Redirection vers /login
4. ✅ Message "Vous devez être connecté"

**Test Manuel - Utilisateur connecté**:
1. Se connecter via /login
2. ✅ Accès au dashboard
3. Naviguer vers /login (direct)
4. ✅ Redirection vers / (dashboard)

### 5.4 Gestion d'État (Auth Context)

**Test Manuel**:
1. Ouvrir DevTools → Application → LocalStorage
2. Se connecter
3. ✅ `access_token` présent
4. ✅ `refresh_token` présent
5. ✅ `user` présent (JSON)
6. Se déconnecter
7. ✅ Tokens supprimés du localStorage

### 5.5 API Calls (useAuth hook)

**Test Manuel** (ouvrir DevTools → Network):
1. Se connecter
2. ✅ POST /api/v1/auth/login envoyé
3. ✅ Authorization header ajouté automatiquement
4. ✅ Refresh token automatique si token expiré
5. Se déconnecter
6. ✅ POST /api/v1/auth/logout envoyé

### 5.6 Validation Formulaires

**Test: Login Form**
- Email vide → ✅ Erreur affichée
- Email invalide → ✅ Erreur format
- Password vide → ✅ Erreur affichée
- Login échoué → ✅ Message d'erreur serveur

**Test: Register Form**
- Username vide → ✅ Erreur
- Email invalide → ✅ Erreur
- Password < 8 chars → ✅ Erreur
- Passwords ne matchent pas → ✅ Erreur
- Email existant → ✅ Erreur serveur

---

## 📊 6. Tests Monitoring & Observabilité

### 6.1 Prometheus (http://localhost:9090)

**Test Manuel**:
1. Ouvrir http://localhost:9090
2. ✅ UI Prometheus chargée
3. ✅ Status → Targets → Voir targets configurés
4. Tester query: `up{job="backend"}`
5. ✅ Métriques disponibles

**Test Métriques Backend**:
```bash
# Si endpoint /metrics implémenté
curl http://localhost:8000/api/metrics

# Attendu: Métriques Prometheus format
# http_requests_total{...}
# http_request_duration_seconds{...}
```

### 6.2 Grafana (http://localhost:3001)

**Test Manuel**:
1. Ouvrir http://localhost:3001
2. Login: `admin` / `admin`
3. ✅ Dashboard chargé
4. ✅ Datasource Prometheus configurée
5. ✅ Datasource Loki configurée
6. Aller dans Explore
7. Tester query Prometheus: `rate(http_requests_total[5m])`
8. ✅ Graphique affiché

### 6.3 Loki (http://localhost:3100)

**Test Manuel**:
1. Grafana → Explore → Loki
2. Query: `{container_name="timemanager-backend"}`
3. ✅ Logs backend affichés
4. Query: `{container_name="timemanager-frontend"}`
5. ✅ Logs frontend affichés

```bash
# Test API Loki
curl http://localhost:3100/ready

# Attendu: ready
```

### 6.4 Mailpit (http://localhost:8025)

**Test Manuel**:
1. Ouvrir http://localhost:8025
2. ✅ Interface Mailpit chargée
3. ✅ Liste des emails vide (si aucun envoi)

**Test Envoi Email** (après implémentation password reset):
1. Demander reset password via frontend
2. Aller sur http://localhost:8025
3. ✅ Email reçu dans Mailpit
4. ✅ Contenu email correct (lien reset)
5. ✅ From/To corrects

```bash
# Test SMTP
docker compose exec backend telnet mailpit 1025

# Attendu: 220 mailpit ESMTP Service Ready
```

---

## 🔥 7. Tests Hot Reload

### 7.1 Backend Hot Reload (cargo-watch)

**Test**:
1. Vérifier logs backend:
```bash
docker compose logs backend -f
```

2. Éditer `backend/src/api/handlers/health.rs`:
```rust
// Changer "ok" en "healthy"
status: "healthy".to_string(),
```

3. Sauvegarder
4. ✅ Cargo watch détecte le changement
5. ✅ Recompilation automatique
6. ✅ Server redémarre

7. Tester:
```bash
curl http://localhost:8000/api/health | jq
# Attendu: "status": "healthy"
```

8. Revenir à "ok"
9. ✅ Même processus de reload

### 7.2 Frontend Hot Reload (Vite HMR)

**Test**:
1. Ouvrir http://localhost:8000
2. Ouvrir DevTools → Console
3. Éditer `frontend/src/App.tsx`:
```tsx
// Ajouter un <h1>
<h1>Test Hot Reload</h1>
```

4. Sauvegarder
5. ✅ Page se met à jour SANS refresh complet
6. ✅ Console: "[vite] hot updated"
7. ✅ État de l'application préservé

---

## 🧪 8. Tests Unitaires & Intégration

### 8.1 Tests Backend (Rust)
```bash
# Test: Lancer tous les tests backend
docker compose exec backend cargo test

# Attendu: All tests passed
```

**Tests à vérifier**:
- ✅ `test_health_check` (health.rs)
- ✅ `test_health_route` (router.rs)
- ✅ Tests handlers auth (si implémentés)
- ✅ Tests middlewares

### 8.2 Tests Frontend (Vitest)
```bash
# Test: Lancer tous les tests frontend
docker compose exec frontend npm run ci

# Attendu: All tests passed
```

**Tests à vérifier**:
- ✅ `LoginForm.test.tsx`
- ✅ `RegisterForm.test.tsx`
- ✅ `ProtectedRoute.test.tsx`
- ✅ `useAuth.test.tsx`
- ✅ `routing-integration.test.tsx`
- ✅ `auth-integration.test.tsx`

### 8.3 Coverage
```bash
# Frontend: Vérifier coverage
docker compose exec frontend npm run test:coverage

# Attendu: Coverage > 80%
```

---

## 🔒 9. Tests Sécurité

### 9.1 JWT Tokens

**Test: Expiration Token**
1. Se connecter
2. Copier access_token
3. Attendre expiration (15 min par défaut)
4. Faire requête avec token expiré
5. ✅ 401 Unauthorized
6. ✅ Frontend refresh automatique

**Test: Invalid Token**
```bash
curl http://localhost:8000/api/v1/auth/me \
  -H "Authorization: Bearer invalid_token"

# Attendu: 401 Unauthorized
```

### 9.2 CORS

**Test: Origin invalide**
```bash
curl -H "Origin: http://malicious-site.com" \
  http://localhost:8000/api/health

# Attendu: Pas d'Access-Control-Allow-Origin
```

**Test: Origin valide**
```bash
curl -H "Origin: http://localhost:8000" \
  http://localhost:8000/api/health

# Attendu: Access-Control-Allow-Origin: http://localhost:8000
```

### 9.3 Rate Limiting

**Test: Dépassement limite**
```bash
# Login: 5 req/min max
for i in {1..10}; do
  curl -X POST http://localhost:8000/api/v1/auth/login \
    -H "Content-Type: application/json" \
    -d '{"email":"test@test.com","password":"wrong"}' \
    -w "\nStatus: %{http_code}\n"
done

# Attendu: Premières requêtes 401, puis 429 Too Many Requests
```

### 9.4 CSRF Protection

**Test: Requête sans CSRF token** (si CSRF middleware activé)
```bash
curl -X POST http://localhost:8000/api/v1/auth/logout \
  -H "Authorization: Bearer <token>"
  # Sans X-CSRF-Token header

# Attendu: 403 Forbidden (si CSRF activé)
```

### 9.5 SQL Injection

**Test: Tentative SQL injection dans login**
```bash
curl -X POST http://localhost:8000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@test.com OR 1=1--",
    "password": "anything"
  }'

# Attendu: 400 Bad Request (email invalide)
# OU 401 Unauthorized
# PAS de comportement étrange ni SQL error
```

---

## 🚨 10. Tests Erreurs & Edge Cases

### 10.1 Backend Crash Recovery

**Test**:
```bash
# Tuer le backend
docker compose kill backend

# Attendre 2-3 secondes
sleep 3

# Vérifier qu'il redémarre
docker compose ps backend

# Attendu: Redémarré automatiquement (restart: unless-stopped)
```

### 10.2 Database Disconnection

**Test**:
```bash
# Arrêter PostgreSQL
docker compose stop postgres

# Tenter requête nécessitant DB
curl http://localhost:8000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@test.com","password":"test"}'

# Attendu: 503 Service Unavailable
# OU 500 Internal Server Error avec message explicite

# Redémarrer PostgreSQL
docker compose start postgres

# Attendre healthcheck
sleep 10

# Retester requête
# Attendu: Fonctionne à nouveau
```

### 10.3 Traefik Down

**Test**:
```bash
# Arrêter Traefik
docker compose stop traefik

# Tenter accès via 8000
curl -I http://localhost:8000

# Attendu: Connection refused

# Redémarrer
docker compose start traefik

# Retester
# Attendu: Fonctionne
```

### 10.4 Payload Trop Grand

**Test**:
```bash
# Créer payload 10MB
python3 -c "print('a' * 10000000)" > /tmp/large.json

curl -X POST http://localhost:8000/api/v1/auth/register \
  -H "Content-Type: application/json" \
  --data @/tmp/large.json

# Attendu: 413 Payload Too Large
```

### 10.5 Requêtes Malformées

**Test: JSON invalide**
```bash
curl -X POST http://localhost:8000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d 'invalid json{'

# Attendu: 400 Bad Request
```

**Test: Content-Type manquant**
```bash
curl -X POST http://localhost:8000/api/v1/auth/login \
  -d '{"email":"test","password":"test"}'

# Attendu: 415 Unsupported Media Type
```

---

## 📈 11. Tests Performance

### 11.1 Load Test Simple

**Test: 100 requêtes concurrentes**
```bash
# Installer Apache Bench si nécessaire
# brew install httpd (macOS)

ab -n 1000 -c 100 http://localhost:8000/api/health

# Analyser:
# - Requests per second
# - Time per request (mean)
# - Failed requests (doit être 0)
```

### 11.2 Response Time

**Test: Temps de réponse < 100ms**
```bash
curl -w "\nTime: %{time_total}s\n" http://localhost:8000/api/health

# Attendu: < 0.1s (100ms)
```

---

## 🎯 12. Scénarios E2E Complets

### Scénario 1: Inscription → Connexion → Dashboard
1. ✅ Ouvrir http://localhost:8000/register
2. ✅ Remplir formulaire inscription
3. ✅ Soumettre
4. ✅ Redirection vers dashboard
5. ✅ Token sauvegardé dans localStorage
6. ✅ Nom utilisateur affiché
7. ✅ Rafraîchir page → toujours connecté

### Scénario 2: Reset Password Complet
1. ✅ Ouvrir /password-reset-request
2. ✅ Entrer email
3. ✅ Message "Email envoyé"
4. ✅ Ouvrir Mailpit (http://localhost:8025)
5. ✅ Vérifier email reçu
6. ✅ Copier lien reset
7. ✅ Ouvrir lien dans navigateur
8. ✅ Formulaire reset password affiché
9. ✅ Entrer nouveau mot de passe
10. ✅ Soumettre
11. ✅ Message "Réinitialisé avec succès"
12. ✅ Se connecter avec nouveau mot de passe

### Scénario 3: Session Management
1. ✅ Se connecter
2. ✅ Ouvrir onglet 2 (même navigateur)
3. ✅ Toujours connecté dans onglet 2
4. ✅ Se déconnecter dans onglet 1
5. ✅ Rafraîchir onglet 2 → déconnecté

### Scénario 4: Token Refresh
1. ✅ Se connecter
2. ✅ Attendre expiration access_token (15min)
3. ✅ Faire action nécessitant auth
4. ✅ Refresh automatique (check Network)
5. ✅ Action réussit sans déconnexion

---

## 🎬 13. Tests Logs & Debugging

### 13.1 Logs Backend
```bash
# Temps réel
docker compose logs backend -f

# Filtrer erreurs
docker compose logs backend | grep -i error

# Dernières 100 lignes
docker compose logs backend --tail 100
```

**Vérifier**:
- ✅ Pas d'erreur critique
- ✅ Requêtes HTTP loggées
- ✅ Niveau DEBUG en dev

### 13.2 Logs Frontend
```bash
# Logs Vite dev server
docker compose logs frontend -f

# Vérifier erreurs de build
docker compose logs frontend | grep -i error
```

**Vérifier dans navigateur**:
- ✅ Pas d'erreurs console
- ✅ Pas de warnings React
- ✅ Network requests OK

### 13.3 Logs Traefik
```bash
docker compose logs traefik | grep -i error

# Voir routing
docker compose logs traefik | grep "Adding route"
```

---

## ✅ Checklist Finale

### Infrastructure
- [ ] 8 conteneurs UP et healthy
- [ ] Volumes persistants créés
- [ ] Network Docker fonctionnel

### Traefik
- [ ] Dashboard accessible (8081)
- [ ] Frontend routé via /
- [ ] Backend routé via /api
- [ ] Middlewares actifs (rate limit, CORS, security)
- [ ] Strip prefix fonctionne

### Backend
- [ ] Health check OK
- [ ] Endpoints auth implémentés et testés
- [ ] JWT tokens fonctionnels
- [ ] Connexion DB OK
- [ ] Hot reload cargo-watch OK
- [ ] Tests unitaires passent

### Frontend
- [ ] Toutes les pages accessibles
- [ ] Routing fonctionne
- [ ] Protected routes OK
- [ ] Formulaires validés
- [ ] API calls OK (useAuth)
- [ ] Hot reload Vite OK
- [ ] Tests unitaires passent

### Monitoring
- [ ] Prometheus accessible (9090)
- [ ] Grafana accessible (3001)
- [ ] Loki logs disponibles
- [ ] Mailpit accessible (8025)

### Sécurité
- [ ] JWT expiration testée
- [ ] CORS configuré
- [ ] Rate limiting fonctionne
- [ ] Security headers présents
- [ ] SQL injection protégé

### Performance
- [ ] Response time < 100ms
- [ ] Load test OK (1000 req)
- [ ] Pas de memory leaks

### E2E
- [ ] Scénario inscription complet
- [ ] Scénario reset password complet
- [ ] Scénario session management OK
- [ ] Token refresh automatique

---

## 🐛 Debugging

### Problème: Backend 502
```bash
# Vérifier compilation
docker compose logs backend --tail 50

# Vérifier santé
curl -I http://localhost:8080/health
```

### Problème: CORS errors
```bash
# Vérifier config Traefik
docker compose logs traefik | grep CORS

# Vérifier headers
curl -I -H "Origin: http://localhost:8000" http://localhost:8000/api/health
```

### Problème: Hot reload ne marche pas
```bash
# Backend: Vérifier volumes
docker compose ps backend -q | xargs docker inspect | grep -A 10 Mounts

# Frontend: Vérifier Vite
docker compose logs frontend | grep "hmr"
```

---

## 📝 Rapport de Tests

Après avoir complété tous les tests, créer un rapport:

```markdown
# Rapport Tests E2E - Time Manager
Date: YYYY-MM-DD
Mode: Dev

## ✅ Réussis
- Infrastructure: X/X
- Backend: X/X
- Frontend: X/X
- Monitoring: X/X
- Sécurité: X/X

## ❌ Échecs
- [Liste des tests échoués avec détails]

## ⚠️ Warnings
- [Liste des avertissements]

## 📊 Métriques
- Tests totaux: XXX
- Taux de réussite: XX%
- Temps total: XX min

## 🔧 Actions Requises
- [Liste des corrections nécessaires]
```

---

**Bon courage pour les tests! 🚀**
