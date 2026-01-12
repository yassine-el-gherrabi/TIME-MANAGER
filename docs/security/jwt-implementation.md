# JWT Implementation

> Implémentation des JSON Web Tokens avec RS256

---

## Architecture JWT

```mermaid
graph TB
    subgraph Tokens["🎫 Token Types"]
        Access["Access Token<br/><small>15 min, Memory</small>"]
        Refresh["Refresh Token<br/><small>7 jours, HttpOnly Cookie</small>"]
    end

    subgraph Keys["🔑 RSA Keys"]
        Private["Private Key<br/><small>Signing</small>"]
        Public["Public Key<br/><small>Verification</small>"]
    end

    subgraph Flow["🔄 Token Flow"]
        Login["Login"]
        API["API Request"]
        Renew["Token Refresh"]
    end

    Login --> Access
    Login --> Refresh
    Private --> Access
    Public --> API
    Refresh --> Renew
    Renew --> Access
```

---

## Configuration

### Durée de vie

| Token | Durée | Configurable |
|-------|-------|--------------|
| Access Token | 15 minutes | `JWT_ACCESS_TOKEN_EXPIRY_SECONDS` |
| Refresh Token | 7 jours | `JWT_REFRESH_TOKEN_EXPIRY_SECONDS` |

### Variables d'environnement

```env
# Chemins des clés RSA
JWT_KEYS_PATH=/app/keys

# Ou clés inline (développement)
JWT_PRIVATE_KEY=-----BEGIN PRIVATE KEY-----...
JWT_PUBLIC_KEY=-----BEGIN PUBLIC KEY-----...

# Durées (secondes)
JWT_ACCESS_TOKEN_EXPIRY_SECONDS=900
JWT_REFRESH_TOKEN_EXPIRY_SECONDS=604800
```

---

## Structure des tokens

### Access Token

```json
{
  "header": {
    "alg": "RS256",
    "typ": "JWT"
  },
  "payload": {
    "sub": "550e8400-e29b-41d4-a716-446655440000",
    "email": "user@example.com",
    "role": "employee",
    "org_id": "org-uuid-here",
    "iat": 1704067200,
    "exp": 1704068100
  }
}
```

### Claims

| Claim | Type | Description |
|-------|------|-------------|
| `sub` | UUID | User ID unique |
| `email` | String | Email utilisateur |
| `role` | Enum | `super_admin`, `admin`, `manager`, `employee` |
| `org_id` | UUID | Organisation ID |
| `iat` | Timestamp | Issued at |
| `exp` | Timestamp | Expiration |

---

## Flux de rafraîchissement

```mermaid
sequenceDiagram
    participant C as Client
    participant B as Backend
    participant DB as Database

    Note over C: Access token expiré

    C->>B: POST /auth/refresh
    Note over C,B: Cookie: refresh_token=xxx

    B->>B: Parse refresh token from cookie

    alt Token absent ou malformé
        B-->>C: 401 Unauthorized
    else Token présent
        B->>DB: SELECT * FROM sessions WHERE token = ?

        alt Session non trouvée ou expirée
            B->>B: Clear cookie
            B-->>C: 401 Unauthorized
        else Session valide
            B->>B: Generate new access token
            B->>B: Generate new refresh token
            B->>DB: UPDATE session SET token = ?
            B-->>C: 200 + access_token + Set-Cookie
        end
    end
```

---

## Stockage des tokens

### Access Token (Frontend)

```typescript
// ❌ Ne JAMAIS stocker dans localStorage
localStorage.setItem('token', accessToken); // DANGER!

// ✅ Stocker en mémoire uniquement
let accessToken: string | null = null;

export const setAccessToken = (token: string) => {
  accessToken = token;
};

export const getAccessToken = () => accessToken;
```

### Refresh Token (Backend → Cookie)

```rust
// Set-Cookie header
Set-Cookie: refresh_token=xxx;
  HttpOnly;
  Secure;
  SameSite=Strict;
  Path=/api/v1/auth;
  Max-Age=604800
```

**Attributs du cookie :**

| Attribut | Valeur | Raison |
|----------|--------|--------|
| `HttpOnly` | true | Protection XSS |
| `Secure` | true | HTTPS only |
| `SameSite` | Strict | Protection CSRF |
| `Path` | /api/v1/auth | Scope limité |
| `Max-Age` | 604800 | 7 jours |

---

## Rotation des tokens

### Stratégie de rotation

```mermaid
graph LR
    R1["Refresh Token v1"] -->|"Utilisé"| R2["Refresh Token v2"]
    R2 -->|"Utilisé"| R3["Refresh Token v3"]
    R1 -->|"Invalidé"| X1["❌"]
    R2 -->|"Invalidé"| X2["❌"]
```

**Règles :**
1. Chaque refresh génère un nouveau refresh token
2. L'ancien refresh token est immédiatement invalidé
3. Réutilisation d'un ancien token = compromission détectée

### Détection de compromission

```mermaid
graph TD
    Use["Utilisation refresh token"] --> Check{"Token déjà utilisé ?"}
    Check -->|Non| Rotate["Rotation normale"]
    Check -->|Oui| Compromise["🚨 Compromission détectée"]
    Compromise --> RevokeAll["Révoquer TOUTES les sessions"]
    Compromise --> Alert["Notifier l'utilisateur"]
```

---

## Gestion des clés RSA

### Génération des clés

```bash
# Générer clé privée (2048 bits minimum)
openssl genrsa -out private.pem 2048

# Extraire clé publique
openssl rsa -in private.pem -pubout -out public.pem
```

### Structure des fichiers

```
/app/keys/
├── private.pem  # Signing (backend only)
└── public.pem   # Verification (peut être distribué)
```

### Rotation des clés

1. Générer nouvelle paire de clés
2. Ajouter nouvelle clé publique au keystore
3. Basculer signing vers nouvelle clé privée
4. Attendre expiration des anciens tokens
5. Retirer ancienne clé publique

---

## Vérification des tokens

### Middleware d'authentification

```rust
// Pseudocode du middleware
pub async fn auth_middleware(req: Request, next: Next) -> Response {
    // 1. Extraire le token du header
    let token = extract_bearer_token(&req)?;

    // 2. Décoder et vérifier la signature
    let claims = decode_jwt(&token, &public_key)?;

    // 3. Vérifier l'expiration
    if claims.exp < now() {
        return Err(Unauthorized);
    }

    // 4. Charger l'utilisateur
    let user = load_user(claims.sub)?;

    // 5. Vérifier que l'utilisateur est actif
    if user.status != Active {
        return Err(Unauthorized);
    }

    // 6. Injecter dans le contexte
    req.extensions().insert(user);

    next.run(req).await
}
```

---

## Sécurité

### Protections implémentées

| Protection | Implémentation |
|------------|----------------|
| Signature forte | RS256 (RSA + SHA-256) |
| Clé asymétrique | Private key jamais exposée |
| Expiration courte | 15 min access token |
| HttpOnly cookie | Protection XSS |
| SameSite Strict | Protection CSRF |
| Rotation | Nouveaux tokens à chaque refresh |

### Bonnes pratiques

1. **Ne jamais exposer** la clé privée
2. **Toujours valider** la signature avant d'utiliser les claims
3. **Vérifier l'expiration** même après validation signature
4. **Logger** les tentatives d'utilisation de tokens invalides
5. **Révoquer** immédiatement en cas de compromission suspectée

---

## Troubleshooting

### Token invalide

```bash
# Décoder un JWT (sans vérification)
echo "eyJhbG..." | cut -d. -f2 | base64 -d | jq

# Vérifier avec la clé publique
# Utiliser jwt.io ou bibliothèque locale
```

### Token expiré trop vite

```bash
# Vérifier les env vars
echo $JWT_ACCESS_TOKEN_EXPIRY_SECONDS
echo $JWT_REFRESH_TOKEN_EXPIRY_SECONDS

# Vérifier l'heure du serveur
date -u
```

### Cookie non envoyé

1. Vérifier `Secure` flag (nécessite HTTPS)
2. Vérifier `SameSite` policy
3. Vérifier `Path` du cookie vs URL de la requête
4. Vérifier CORS `credentials: 'include'`

---

## Liens connexes

- [Auth Flow](../features/auth-flow.md)
- [Password Security](./password-security.md)
- [Protection Mechanisms](./protection-mechanisms.md)
