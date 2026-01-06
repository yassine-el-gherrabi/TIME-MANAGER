# TM-E02 : Authentification & Sécurité

## Informations Epic

| Champ | Valeur |
|-------|--------|
| **ID** | TM-E02 |
| **Titre** | Authentification & Sécurité |
| **Priorité** | P0 - Bloquant |
| **Estimation globale** | 35 SP |
| **Sprint cible** | Sprint 2 |
| **Dépendances** | TM-E01 (Infrastructure) |

---

## Description

### Contexte

L'authentification est le pilier de la sécurité de Time Manager. Le système doit gérer les accès de plusieurs organisations (multi-tenant) avec différents niveaux de permissions (4 rôles). La sécurité est critique car l'application manipule des données RH sensibles (présences, absences, informations personnelles).

### Objectif Business

Permettre aux utilisateurs de s'authentifier de manière sécurisée et de maintenir leur session active sans friction, tout en protégeant l'application contre les attaques courantes (CSRF, XSS, brute force).

### Valeur Apportée

- **Pour les utilisateurs** : Connexion simple, session persistante, récupération de mot de passe
- **Pour l'entreprise** : Données protégées, conformité RGPD, traçabilité des accès
- **Pour les admins** : Création d'organisations autonomes via inscription

---

## Scope

### Inclus

- Inscription (création organisation + admin)
- Connexion avec JWT (access token 15min + refresh token 7 jours)
- Refresh automatique des tokens
- Déconnexion (révocation tokens)
- Reset de mot de passe par email
- Protection CSRF (Double Submit Cookie)
- Hash des mots de passe (Argon2)
- Middleware d'authentification
- Pages frontend (login, register, forgot password)

### Exclus

- OAuth / SSO (hors scope MVP)
- 2FA / MFA (hors scope MVP)
- Audit des connexions (Epic E11)

---

## Critères de Succès de l'Epic

- [ ] Un utilisateur peut créer une organisation et son compte admin
- [ ] Un utilisateur peut se connecter et reçoit ses tokens
- [ ] La session reste active grâce au refresh automatique
- [ ] Un utilisateur peut réinitialiser son mot de passe par email
- [ ] Les routes protégées rejettent les requêtes non authentifiées (401)
- [ ] Les tokens expirés sont correctement gérés
- [ ] Aucune faille CSRF exploitable

---

## User Stories

---

### TM-7 : Inscription et création d'organisation

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 5 SP |
| **Assigné** | - |

#### Description

**En tant que** nouvel utilisateur,
**Je veux** créer mon compte et mon organisation,
**Afin de** commencer à utiliser Time Manager pour mon entreprise.

#### Contexte Détaillé

L'inscription crée simultanément :
1. Une nouvelle organisation avec un slug unique (utilisé pour l'URL)
2. Un utilisateur avec le rôle ADMIN (premier admin de l'organisation)

Le mot de passe doit respecter des critères de sécurité (min 8 caractères, 1 majuscule, 1 chiffre). L'email doit être unique globalement.

#### Critères d'Acceptation

- [ ] Endpoint `POST /api/v1/auth/register` créé
- [ ] Validation des données :
  - Email : format valide, unique globalement
  - Password : min 8 chars, 1 majuscule, 1 minuscule, 1 chiffre
  - Organisation name : min 2 chars
- [ ] Génération automatique du slug depuis le nom (ex: "Ma Société" → "ma-societe")
- [ ] Gestion des conflits de slug (ajout suffix numérique)
- [ ] Hash du password avec Argon2
- [ ] Création user avec rôle ADMIN
- [ ] Génération des tokens (access + refresh)
- [ ] Retour : user profile + access token + cookie refresh token
- [ ] Erreurs explicites (email déjà utilisé, validation failed)
- [ ] Expiration mot de passe : 90 jours par défaut (configurable par organisation)
- [ ] Historique mots de passe : empêcher réutilisation des 5 derniers
- [ ] Table password_history créée pour stocker les anciens hashs

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-7.1 | Créer DTO RegisterRequest avec validations | 1h |
| TM-7.2 | Implémenter service de génération de slug | 1h |
| TM-7.3 | Implémenter AuthService.register() | 3h |
| TM-7.4 | Créer endpoint POST /auth/register | 1h |
| TM-7.5 | Écrire tests unitaires du service | 2h |
| TM-7.6 | Écrire tests d'intégration de l'endpoint | 1h |
| TM-7.7 | Créer migration table password_history | 0.5h |
| TM-7.8 | Implémenter vérification historique mots de passe | 1h |

---

### TM-8 : Connexion utilisateur

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant que** utilisateur enregistré,
**Je veux** me connecter avec mon email et mot de passe,
**Afin d'** accéder à mon espace Time Manager.

#### Contexte Détaillé

La connexion vérifie les credentials et génère deux tokens :
- **Access Token** (15 min) : JWT signé, stocké en mémoire côté client
- **Refresh Token** (7 jours) : Stocké en cookie HttpOnly, hashé en base

Le refresh token permet de renouveler l'access token sans redemander le mot de passe.

#### Critères d'Acceptation

- [ ] Endpoint `POST /api/v1/auth/login` créé
- [ ] Validation email/password
- [ ] Vérification du hash password avec Argon2
- [ ] Gestion compte désactivé (deleted_at not null)
- [ ] Génération access token JWT (15 min expiry)
  - Claims : sub (user_id), org (organization_id), role, type, iat, exp
- [ ] Génération refresh token (7 jours expiry)
  - Token aléatoire 256 bits
  - Hashé en base (table refresh_tokens)
  - Cookie HttpOnly, Secure, SameSite=Strict
- [ ] Génération CSRF token (cookie + à inclure dans réponse)
- [ ] Retour : user profile + access token
- [ ] Erreur générique "Invalid credentials" (pas de leak d'info)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-8.1 | Créer migration table refresh_tokens | 1h |
| TM-8.2 | Créer DTO LoginRequest/LoginResponse | 0.5h |
| TM-8.3 | Implémenter génération JWT (utils/jwt.rs) | 1h |
| TM-8.4 | Implémenter AuthService.login() | 2h |
| TM-8.5 | Créer endpoint POST /auth/login | 1h |
| TM-8.6 | Configurer cookies HttpOnly | 1h |
| TM-8.7 | Tests unitaires et d'intégration | 1h |

---

### TM-9 : Refresh des tokens

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant que** utilisateur connecté,
**Je veux** que ma session reste active automatiquement,
**Afin de** ne pas être déconnecté toutes les 15 minutes.

#### Contexte Détaillé

Le refresh utilise la rotation des tokens pour la sécurité :
1. Client envoie le refresh token (cookie HttpOnly)
2. Backend vérifie le token (non révoqué, non expiré)
3. Backend révoque l'ancien refresh token
4. Backend génère nouveau refresh token + nouveau access token
5. Retour des nouveaux tokens

Cette rotation limite l'impact d'un vol de refresh token.

#### Critères d'Acceptation

- [ ] Endpoint `POST /api/v1/auth/refresh` créé
- [ ] Lecture du refresh token depuis cookie HttpOnly
- [ ] Vérification : token existe, non révoqué, non expiré
- [ ] Rotation : révocation ancien token, création nouveau
- [ ] Génération nouveau access token
- [ ] Mise à jour du cookie refresh token
- [ ] Erreur 401 si refresh token invalide/expiré
- [ ] Nettoyage automatique des tokens expirés (job ou à la volée)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-9.1 | Implémenter RefreshTokenRepository | 1h |
| TM-9.2 | Implémenter AuthService.refresh() | 2h |
| TM-9.3 | Créer endpoint POST /auth/refresh | 1h |
| TM-9.4 | Implémenter révocation de l'ancien token | 1h |
| TM-9.5 | Tests d'intégration du flow complet | 1h |

---

### TM-10 : Déconnexion

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant que** utilisateur connecté,
**Je veux** me déconnecter,
**Afin de** sécuriser mon accès quand je quitte l'application.

#### Contexte Détaillé

La déconnexion révoque le refresh token actuel. L'access token reste techniquement valide jusqu'à expiration (15 min max), mais sans refresh token, l'utilisateur devra se reconnecter.

Option "Déconnecter partout" : révoque TOUS les refresh tokens de l'utilisateur (utile si compromission suspectée).

#### Critères d'Acceptation

- [ ] Endpoint `POST /api/v1/auth/logout` créé
- [ ] Révocation du refresh token actuel
- [ ] Suppression du cookie refresh token
- [ ] Endpoint `POST /api/v1/auth/logout-all` créé
- [ ] Révocation de tous les refresh tokens de l'utilisateur
- [ ] Retour 200 OK même si déjà déconnecté (idempotent)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-10.1 | Implémenter AuthService.logout() | 1h |
| TM-10.2 | Implémenter AuthService.logoutAll() | 1h |
| TM-10.3 | Créer endpoints logout et logout-all | 1h |
| TM-10.4 | Tests d'intégration | 1h |

---

### TM-11 : Middleware d'authentification JWT

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur backend,
**Je veux** un middleware qui valide les JWT sur les routes protégées,
**Afin de** sécuriser les endpoints de l'API.

#### Contexte Détaillé

Le middleware Axum intercepte toutes les requêtes vers les routes protégées :
1. Extrait le header `Authorization: Bearer <token>`
2. Valide la signature JWT
3. Vérifie l'expiration
4. Extrait les claims (user_id, org_id, role)
5. Injecte un `CurrentUser` dans le contexte de la requête

Si invalide → 401 Unauthorized

#### Critères d'Acceptation

- [ ] Middleware `auth_middleware` créé
- [ ] Extraction du Bearer token depuis header Authorization
- [ ] Validation signature JWT avec la clé secrète
- [ ] Vérification expiration (exp claim)
- [ ] Vérification type = "access" (pas un refresh token)
- [ ] Extraction CurrentUser : id, organization_id, role (PAS d'email - RGPD)
- [ ] Injection dans request extensions
- [ ] Retour 401 avec message approprié si échec :
  - "Missing authorization header"
  - "Invalid token format"
  - "Token expired"
  - "Invalid token"
- [ ] Extractor `CurrentUser` pour les handlers

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-11.1 | Créer struct CurrentUser | 0.5h |
| TM-11.2 | Implémenter JWT validation (utils/jwt.rs) | 1h |
| TM-11.3 | Créer le middleware auth_middleware | 2h |
| TM-11.4 | Créer l'extractor CurrentUser | 1h |
| TM-11.5 | Appliquer aux routes protégées | 0.5h |
| TM-11.6 | Tests unitaires du middleware | 1h |

---

### TM-12 : Protection CSRF

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant que** responsable sécurité,
**Je veux** une protection CSRF sur les endpoints sensibles,
**Afin de** prévenir les attaques de type Cross-Site Request Forgery.

#### Contexte Détaillé

Pattern utilisé : **Double Submit Cookie**
1. Au login, génération d'un token CSRF aléatoire
2. Token stocké dans un cookie (SameSite=Strict)
3. Token aussi retourné dans la réponse JSON
4. Le frontend doit inclure le token dans le header `X-CSRF-Token`
5. Le backend compare header vs cookie

Les requêtes GET/HEAD/OPTIONS sont exemptées (safe methods).

#### Critères d'Acceptation

- [ ] Middleware CSRF créé
- [ ] Génération token CSRF au login (256 bits random)
- [ ] Cookie `csrf_token` (SameSite=Strict, non HttpOnly pour lecture JS)
- [ ] Validation header `X-CSRF-Token` vs cookie
- [ ] Appliqué sur POST, PUT, PATCH, DELETE
- [ ] Exempté sur GET, HEAD, OPTIONS
- [ ] Exempté sur /auth/login et /auth/register (pas encore de cookie)
- [ ] Retour 403 Forbidden si CSRF invalide

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-12.1 | Implémenter génération token CSRF | 0.5h |
| TM-12.2 | Créer middleware csrf_middleware | 2h |
| TM-12.3 | Configurer le cookie CSRF | 0.5h |
| TM-12.4 | Ajouter CSRF token à la réponse login | 0.5h |
| TM-12.5 | Tests de validation CSRF | 1h |

---

### TM-13 : Reset de mot de passe

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant que** utilisateur ayant oublié mon mot de passe,
**Je veux** le réinitialiser via un lien envoyé par email,
**Afin de** récupérer l'accès à mon compte.

#### Contexte Détaillé

Flow en deux étapes :
1. **Forgot password** : L'utilisateur entre son email → génération token → envoi email
2. **Reset password** : L'utilisateur clique le lien → entre nouveau password → token invalidé

Le token de reset expire après 1 heure et est à usage unique.

#### Critères d'Acceptation

- [ ] Endpoint `POST /api/v1/auth/forgot-password`
  - Accepte un email
  - Génère un token de reset (256 bits, hashé en base)
  - Envoie un email avec lien contenant le token
  - Retourne toujours 200 (pas de leak si email existe)
- [ ] Endpoint `POST /api/v1/auth/reset-password`
  - Accepte token + nouveau password
  - Valide le token (existe, non utilisé, non expiré)
  - Met à jour le password (hash Argon2)
  - Invalide le token
  - Révoque tous les refresh tokens (sécurité)
- [ ] Token expire après 1 heure
- [ ] Token à usage unique
- [ ] Template email simple et clair

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-13.1 | Créer migration table password_reset_tokens | 0.5h |
| TM-13.2 | Implémenter service d'envoi email | 1h |
| TM-13.3 | Implémenter AuthService.forgotPassword() | 1h |
| TM-13.4 | Implémenter AuthService.resetPassword() | 1h |
| TM-13.5 | Créer les endpoints | 1h |
| TM-13.6 | Créer template email HTML | 1h |
| TM-13.7 | Tests d'intégration | 1h |

---

### TM-14 : Page de connexion (Frontend)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant que** utilisateur,
**Je veux** une page de connexion claire et ergonomique,
**Afin de** m'authentifier facilement.

#### Contexte Détaillé

La page de login est la première impression de l'application. Elle doit être :
- Simple et rapide à charger
- Accessible (labels, focus, erreurs claires)
- Responsive (mobile-first)
- Sécurisée (pas d'autocomplete sur password en prod)

#### Critères d'Acceptation

- [ ] Page `/login` créée
- [ ] Formulaire avec champs email et password
- [ ] Validation côté client (email format, password requis)
- [ ] Bouton "Se connecter" avec loading state
- [ ] Affichage erreurs serveur (credentials invalides)
- [ ] Lien "Mot de passe oublié ?" vers /forgot-password
- [ ] Lien "Créer un compte" vers /register
- [ ] Redirection vers dashboard après login réussi
- [ ] Redirection vers login si accès page protégée non authentifié
- [ ] Responsive (mobile, tablet, desktop)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-14.1 | Créer composant LoginForm | 2h |
| TM-14.2 | Implémenter validation avec React Hook Form + Zod | 1h |
| TM-14.3 | Créer page LoginPage avec layout | 1h |
| TM-14.4 | Intégrer appel API login | 1h |
| TM-14.5 | Gérer les états (loading, error, success) | 1h |
| TM-14.6 | Styling responsive avec Tailwind | 1h |

---

### TM-15 : Page d'inscription (Frontend)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant que** nouveau utilisateur,
**Je veux** une page d'inscription pour créer mon organisation,
**Afin de** démarrer avec Time Manager.

#### Contexte Détaillé

L'inscription crée à la fois l'organisation et le compte admin. Le formulaire doit guider l'utilisateur et valider les données en temps réel.

#### Critères d'Acceptation

- [ ] Page `/register` créée
- [ ] Formulaire avec champs :
  - Nom de l'organisation
  - Email
  - Mot de passe
  - Confirmation mot de passe
  - Prénom, Nom
- [ ] Validation temps réel :
  - Email format
  - Password strength indicator
  - Match password confirmation
- [ ] Affichage des règles password
- [ ] Bouton "Créer mon compte" avec loading state
- [ ] Affichage erreurs serveur (email déjà utilisé)
- [ ] Redirection vers dashboard après inscription
- [ ] Lien "Déjà un compte ? Se connecter"

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-15.1 | Créer composant RegisterForm | 2h |
| TM-15.2 | Implémenter validation avec indicateur force password | 1h |
| TM-15.3 | Créer page RegisterPage | 1h |
| TM-15.4 | Intégrer appel API register | 1h |
| TM-15.5 | Gérer les états et erreurs | 1h |
| TM-15.6 | Styling responsive | 1h |

---

### TM-16 : Contexte d'authentification (Frontend)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur frontend,
**Je veux** un contexte React gérant l'état d'authentification,
**Afin de** partager l'état user dans toute l'application.

#### Contexte Détaillé

L'AuthContext centralise :
- L'état de l'utilisateur connecté
- Les fonctions login/logout/register
- Le refresh automatique des tokens
- La gestion du CSRF token

L'access token est stocké en mémoire (pas localStorage) pour la sécurité.

#### Critères d'Acceptation

- [ ] AuthContext créé avec Provider
- [ ] État : user, isAuthenticated, isLoading
- [ ] Fonctions : login(), logout(), register()
- [ ] Stockage access token en mémoire (variable)
- [ ] Stockage CSRF token en mémoire
- [ ] Refresh automatique avant expiration (à 80% de la durée)
- [ ] Hook `useAuth()` pour accéder au contexte
- [ ] Interceptor Axios pour ajouter :
  - Header Authorization: Bearer <token>
  - Header X-CSRF-Token: <csrf>
- [ ] Interceptor Axios pour refresh sur 401
- [ ] Initialisation : check si refresh token valide au chargement

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-16.1 | Créer AuthContext et AuthProvider | 1h |
| TM-16.2 | Implémenter les actions (login, logout, register) | 2h |
| TM-16.3 | Configurer interceptors Axios | 1h |
| TM-16.4 | Implémenter refresh automatique | 2h |
| TM-16.5 | Créer hook useAuth | 0.5h |
| TM-16.6 | Tests du contexte | 1h |

---

### TM-105 : Protection brute force login

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant que** responsable sécurité,
**Je veux** limiter les tentatives de connexion échouées,
**Afin de** protéger les comptes contre les attaques par force brute.

#### Contexte Détaillé

Les attaques brute force ciblent les endpoints d'authentification pour deviner les mots de passe. La protection combine :
- Rate limiting par IP (limite les attaques distribuées)
- Rate limiting par email (protège les comptes ciblés)
- Lockout progressif (ralentit les attaques persistantes)

#### Critères d'Acceptation

- [ ] Rate limiting : max 5 tentatives / 15 min par IP
- [ ] Rate limiting : max 5 tentatives / 15 min par email
- [ ] Lockout progressif :
  - 5s de délai après 3 échecs consécutifs
  - 60s de délai après 5 échecs consécutifs
  - 15min de lockout après 10 échecs
- [ ] Compteur réinitialisé après connexion réussie
- [ ] Toutes les tentatives échouées loguées dans audit_logs
- [ ] Retour 429 Too Many Requests si limite atteinte
- [ ] Message générique (pas de leak d'info sur l'existence du compte)
- [ ] Admin peut débloquer manuellement un compte via endpoint

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-105.1 | Créer migration table login_attempts | 0.5h |
| TM-105.2 | Implémenter LoginAttemptRepository | 1h |
| TM-105.3 | Implémenter rate limiting dans AuthService.login() | 2h |
| TM-105.4 | Créer endpoint admin pour débloquer compte | 0.5h |
| TM-105.5 | Tests d'intégration brute force | 1h |

---

### TM-106 : Gestion des sessions actives

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P2 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur,
**Je veux** voir et gérer mes sessions actives,
**Afin de** contrôler les appareils connectés à mon compte.

#### Contexte Détaillé

Les utilisateurs doivent pouvoir :
- Visualiser toutes leurs sessions actives (navigateurs, appareils)
- Révoquer une session spécifique
- Révoquer toutes les sessions (déjà implémenté via logout-all)

Limite de sessions simultanées pour éviter les abus.

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/auth/sessions` créé
  - Liste les sessions actives de l'utilisateur
  - Retourne : device, IP, user_agent, dernière activité, is_current
- [ ] Endpoint `DELETE /api/v1/auth/sessions/:id` créé
  - Révoque une session spécifique
  - Impossible de révoquer la session courante (use logout)
- [ ] Limite : max 5 sessions simultanées par utilisateur
- [ ] Nouvelle connexion évince la plus ancienne session si limite atteinte
- [ ] Notification optionnelle (email) si nouvelle connexion détectée
- [ ] Métadonnées enrichies sur refresh_tokens : ip, user_agent, device_name

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-106.1 | Enrichir table refresh_tokens (ip, user_agent, device) | 1h |
| TM-106.2 | Implémenter endpoint GET /auth/sessions | 1.5h |
| TM-106.3 | Implémenter endpoint DELETE /auth/sessions/:id | 1h |
| TM-106.4 | Implémenter limite sessions simultanées | 1h |
| TM-106.5 | Tests d'intégration | 1h |

---

### TM-107 : Notification expiration mot de passe (Frontend)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P2 |
| **Estimation** | 1 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur,
**Je veux** être notifié avant l'expiration de mon mot de passe,
**Afin de** le changer avant d'être bloqué.

#### Critères d'Acceptation

- [ ] Backend retourne `password_expires_at` dans le profil utilisateur
- [ ] Banner d'avertissement 7 jours avant expiration
- [ ] Banner plus urgent 3 jours avant expiration
- [ ] Redirection forcée vers changement password si expiré
- [ ] Email de rappel 7 jours avant (si service email configuré)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-107.1 | Ajouter champ password_expires_at au profil | 0.5h |
| TM-107.2 | Créer composant PasswordExpiryBanner | 1h |
| TM-107.3 | Intégrer dans le layout principal | 0.5h |
| TM-107.4 | Créer page de changement password forcé | 1h |

---

## Récapitulatif des Estimations

| Story | Titre | SP |
|-------|-------|:--:|
| TM-7 | Inscription et création d'organisation | 5 |
| TM-8 | Connexion utilisateur | 3 |
| TM-9 | Refresh des tokens | 3 |
| TM-10 | Déconnexion | 2 |
| TM-11 | Middleware d'authentification JWT | 3 |
| TM-12 | Protection CSRF | 2 |
| TM-13 | Reset de mot de passe | 3 |
| TM-14 | Page de connexion (Frontend) | 3 |
| TM-15 | Page d'inscription (Frontend) | 3 |
| TM-16 | Contexte d'authentification (Frontend) | 3 |
| TM-105 | Protection brute force login | 2 |
| TM-106 | Gestion des sessions actives | 2 |
| TM-107 | Notification expiration mot de passe | 1 |
| **Total** | | **35 SP** |

---

## Notes Techniques

### Structure des Tokens JWT

**Access Token (15 minutes)**

> ⚠️ **RGPD** : Le JWT ne doit PAS contenir d'email ni de données personnelles identifiantes (principe de minimisation des données). L'email est récupéré via `/auth/me` si nécessaire.

```json
{
  "sub": "550e8400-e29b-41d4-a716-446655440000",
  "org": "550e8400-e29b-41d4-a716-446655440001",
  "role": "admin",
  "type": "access",
  "iat": 1704067200,
  "exp": 1704068100
}
```

### Cookies Générés

| Cookie | HttpOnly | Secure | SameSite | Durée |
|--------|:--------:|:------:|:--------:|:-----:|
| refresh_token | Oui | Oui (prod) | Strict | 7 jours |
| csrf_token | Non | Oui (prod) | Strict | Session |

### Endpoints Récapitulatif

| Méthode | Endpoint | Auth | CSRF |
|---------|----------|:----:|:----:|
| POST | /api/v1/auth/register | Non | Non |
| POST | /api/v1/auth/login | Non | Non |
| POST | /api/v1/auth/refresh | Cookie | Non |
| POST | /api/v1/auth/logout | Oui | Oui |
| POST | /api/v1/auth/logout-all | Oui | Oui |
| POST | /api/v1/auth/forgot-password | Non | Non |
| POST | /api/v1/auth/reset-password | Non | Non |
| GET | /api/v1/auth/me | Oui | Non |
| GET | /api/v1/auth/sessions | Oui | Non |
| DELETE | /api/v1/auth/sessions/:id | Oui | Oui |
| POST | /api/v1/admin/unlock-account/:id | Admin | Oui |
