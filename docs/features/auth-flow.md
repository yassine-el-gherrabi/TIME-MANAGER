# Authentification

> Flux complet d'authentification JWT avec refresh tokens et gestion des sessions

---

## Vue d'ensemble

Le système d'authentification utilise :
- **JWT RS256** pour les access tokens (15 min)
- **Refresh tokens** en HttpOnly cookie (7 jours)
- **Argon2id** pour le hashing des mots de passe
- **HIBP** pour la vérification des mots de passe compromis

---

## Flux de Login

```mermaid
sequenceDiagram
    participant U as User
    participant F as Frontend
    participant B as Backend
    participant DB as Database
    participant HIBP as HIBP API

    U->>F: Saisie email + password
    F->>B: POST /api/v1/auth/login

    B->>DB: Find user by email
    DB-->>B: User record

    alt User not found
        B-->>F: 401 Invalid credentials
    else User found
        B->>B: Verify password (Argon2id)

        alt Password invalid
            B->>DB: Increment failed_attempts
            B-->>F: 401 Invalid credentials
        else Password valid
            B->>DB: Reset failed_attempts
            B->>B: Generate JWT (RS256)
            B->>B: Generate refresh token
            B->>DB: Store session
            B-->>F: 200 + AccessToken + Set-Cookie(refresh)
        end
    end

    F->>F: Store access token (memory)
    F-->>U: Redirect to Dashboard
```

---

## Flux de Refresh Token

```mermaid
sequenceDiagram
    participant F as Frontend
    participant B as Backend
    participant DB as Database

    Note over F: Access token expiré (15min)

    F->>B: POST /api/v1/auth/refresh
    Note over F,B: Cookie refresh_token envoyé automatiquement

    B->>B: Extract refresh token from cookie
    B->>DB: Validate refresh token

    alt Token invalide ou expiré
        B-->>F: 401 + Clear cookie
        F->>F: Redirect to login
    else Token valide
        B->>B: Generate new access token
        B->>B: Rotate refresh token
        B->>DB: Update session
        B-->>F: 200 + New access token + New cookie
        F->>F: Continue request
    end
```

---

## Flux d'Invitation Utilisateur

```mermaid
sequenceDiagram
    participant A as Admin
    participant B as Backend
    participant DB as Database
    participant E as Email Service
    participant U as New User

    A->>B: POST /api/v1/users (create user)
    B->>DB: Create user (status: pending)
    B->>B: Generate invitation token
    B->>DB: Store invitation
    B->>E: Send invitation email
    E-->>U: Email avec lien
    B-->>A: 201 User created

    Note over U: Click sur le lien d'invitation

    U->>B: POST /api/v1/auth/verify-invite
    B->>DB: Validate token
    B-->>U: Token valid + user info

    U->>B: POST /api/v1/auth/accept-invite
    Note over U,B: {token, password, firstName, lastName}

    B->>B: Validate password strength
    B->>B: Check HIBP for compromised password
    B->>B: Hash password (Argon2id)
    B->>DB: Update user (status: active)
    B->>DB: Delete invitation
    B->>B: Generate JWT tokens
    B-->>U: 200 + Access token + Cookie
```

---

## Gestion des Sessions

### Architecture des sessions

```mermaid
graph TB
    subgraph User["👤 Utilisateur"]
        Desktop["💻 Desktop Chrome"]
        Mobile["📱 Mobile Safari"]
        Tablet["📋 Tablet Firefox"]
    end

    subgraph Sessions["🔐 Sessions actives"]
        S1["Session 1<br/><small>Desktop - Paris</small>"]
        S2["Session 2<br/><small>Mobile - Lyon</small>"]
        S3["Session 3<br/><small>Tablet - Nice</small>"]
    end

    Desktop --> S1
    Mobile --> S2
    Tablet --> S3

    subgraph Actions["Actions disponibles"]
        List["GET /sessions<br/><small>Lister toutes</small>"]
        Revoke["DELETE /sessions/:id<br/><small>Révoquer une</small>"]
        RevokeAll["POST /logout-all<br/><small>Révoquer toutes</small>"]
    end
```

### Endpoints de session

| Endpoint | Méthode | Description |
|----------|---------|-------------|
| `/auth/sessions` | GET | Liste toutes les sessions actives |
| `/auth/sessions/:id` | DELETE | Révoque une session spécifique |
| `/auth/logout` | POST | Déconnexion session courante |
| `/auth/logout-all` | POST | Révoque toutes les sessions |

---

## Sécurité des Mots de Passe

### Validation

```mermaid
graph TD
    Input["Nouveau mot de passe"] --> Length{"≥ 12 caractères ?"}
    Length -->|Non| Reject1["❌ Trop court"]
    Length -->|Oui| Upper{"Majuscule ?"}
    Upper -->|Non| Reject2["❌ Majuscule requise"]
    Upper -->|Oui| Lower{"Minuscule ?"}
    Lower -->|Non| Reject3["❌ Minuscule requise"]
    Lower -->|Oui| Number{"Chiffre ?"}
    Number -->|Non| Reject4["❌ Chiffre requis"]
    Number -->|Oui| Special{"Caractère spécial ?"}
    Special -->|Non| Reject5["❌ Spécial requis"]
    Special -->|Oui| HIBP["🔍 Check HIBP"]
    HIBP -->|Compromis| Reject6["❌ Mot de passe compromis"]
    HIBP -->|OK| History["📜 Check historique"]
    History -->|Réutilisé| Reject7["❌ Déjà utilisé"]
    History -->|OK| Accept["✅ Mot de passe accepté"]
```

### Règles

| Règle | Valeur |
|-------|--------|
| Longueur minimum | 12 caractères |
| Majuscule | Au moins 1 |
| Minuscule | Au moins 1 |
| Chiffre | Au moins 1 |
| Caractère spécial | Au moins 1 |
| Historique | 5 derniers interdits |
| HIBP | Vérification breach |

---

## Reset Password Flow

```mermaid
sequenceDiagram
    participant U as User
    participant F as Frontend
    participant B as Backend
    participant DB as Database
    participant E as Email

    U->>F: Click "Mot de passe oublié"
    F->>B: POST /api/v1/auth/password/request-reset
    Note over F,B: {email}

    B->>DB: Find user

    alt User exists
        B->>B: Generate reset token (6h expiry)
        B->>DB: Store reset token
        B->>E: Send reset email
    end

    B-->>F: 200 OK
    Note over B,F: Toujours 200 (sécurité)

    U->>F: Click lien email
    F->>B: POST /api/v1/auth/password/reset
    Note over F,B: {token, newPassword}

    B->>DB: Validate token
    B->>B: Validate password
    B->>B: Hash password
    B->>DB: Update user password
    B->>DB: Invalidate all sessions
    B->>DB: Delete reset token
    B-->>F: 200 Password reset
```

---

## Protection Anti-Brute Force

### Mécanisme

```mermaid
graph LR
    Request["🔑 Login attempt"] --> Check{"Attempts < 5 ?"}
    Check -->|Oui| Process["Process login"]
    Check -->|Non| Locked["🔒 Account locked"]

    Process --> Success{"Success ?"}
    Success -->|Oui| Reset["Reset counter"]
    Success -->|Non| Increment["Increment counter"]

    Locked --> Wait["⏱️ Wait 15 min"]
    Wait --> Unlock["🔓 Auto unlock"]
```

### Configuration

| Paramètre | Valeur |
|-----------|--------|
| Max attempts | 5 |
| Lock duration | 15 minutes |
| Counter reset | After successful login |

---

## JWT Structure

### Access Token (15 min)

```json
{
  "header": {
    "alg": "RS256",
    "typ": "JWT"
  },
  "payload": {
    "sub": "user-uuid",
    "email": "user@example.com",
    "role": "employee",
    "org_id": "org-uuid",
    "exp": 1234567890,
    "iat": 1234567890
  }
}
```

### Claims utilisés

| Claim | Description |
|-------|-------------|
| `sub` | User ID (UUID) |
| `email` | Email utilisateur |
| `role` | Rôle RBAC |
| `org_id` | Organisation ID |
| `exp` | Expiration timestamp |
| `iat` | Issued at timestamp |

---

## Endpoints Auth

| Endpoint | Méthode | Auth | Description |
|----------|---------|------|-------------|
| `/auth/login` | POST | ❌ | Connexion |
| `/auth/refresh` | POST | 🍪 | Rafraîchir token |
| `/auth/logout` | POST | ✅ | Déconnexion |
| `/auth/logout-all` | POST | ✅ | Déconnexion globale |
| `/auth/me` | GET | ✅ | Profil utilisateur |
| `/auth/change-password` | PUT | ✅ | Changer mot de passe |
| `/auth/verify-invite` | POST | ❌ | Vérifier invitation |
| `/auth/accept-invite` | POST | ❌ | Accepter invitation |
| `/auth/sessions` | GET | ✅ | Lister sessions |
| `/auth/sessions/:id` | DELETE | ✅ | Révoquer session |
| `/auth/password/request-reset` | POST | ❌ | Demander reset |
| `/auth/password/reset` | POST | ❌ | Reset password |

---

## Liens connexes

- [RBAC & Permissions](./rbac.md)
- [Sécurité JWT](../security/jwt-implementation.md)
- [Protection des mots de passe](../security/password-security.md)
