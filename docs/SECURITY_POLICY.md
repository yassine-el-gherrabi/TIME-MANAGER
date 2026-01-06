# Politique de Sécurité - Time Manager

> Version 1.0 | Classification : Interne

## Vue d'Ensemble

Ce document définit les mesures de sécurité implémentées dans l'application Time Manager pour protéger les données des utilisateurs et garantir l'intégrité du système.

---

## 1. Authentification et Gestion des Accès

### 1.1 Politique de Mots de Passe

| Critère | Exigence |
|---------|----------|
| Longueur minimale | 12 caractères |
| Majuscules | Au moins 1 |
| Minuscules | Au moins 1 |
| Chiffres | Au moins 1 |
| Caractères spéciaux | Au moins 1 (`!@#$%^&*()_+-=[]{}`) |
| Expiration | 90 jours (configurable par organisation) |
| Historique | 5 derniers mots de passe interdits |
| Stockage | Hashage Argon2id avec sel unique |

### 1.2 Protection Brute Force

| Mécanisme | Configuration |
|-----------|---------------|
| Limite par IP | 5 tentatives / 15 minutes |
| Limite par email | 5 tentatives / 15 minutes |
| Lockout progressif | 5s après 3 échecs, 60s après 5 échecs |
| Code HTTP | 429 Too Many Requests |
| Déblocage | Automatique après délai, ou manuel par admin |

### 1.3 Sessions et Tokens

| Élément | Durée de vie | Notes |
|---------|--------------|-------|
| Access Token (JWT) | 15 minutes | Stateless, non révocable |
| Refresh Token | 7 jours | Stocké en base, révocable |
| Sessions simultanées | Max 5 par utilisateur | Les plus anciennes révoquées |

### 1.4 Contenu du JWT (Minimisation RGPD)

```json
{
  "sub": "user-uuid",
  "org": "organization-uuid",
  "role": "employee|manager|admin|super_admin",
  "type": "access|refresh",
  "iat": 1704067200,
  "exp": 1704068100
}
```

> **Important** : L'email n'est PAS inclus dans le JWT (conformité RGPD - minimisation des données).

---

## 2. Chiffrement

### 2.1 Chiffrement en Transit

| Protocole | Version | Usage |
|-----------|---------|-------|
| TLS | 1.3 (minimum 1.2) | Toutes les communications HTTPS |
| HSTS | max-age=31536000 | Forcer HTTPS |

### 2.2 Chiffrement au Repos

| Donnée | Méthode |
|--------|---------|
| Base de données | AES-256 (PostgreSQL native) |
| Mots de passe | Argon2id |
| Tokens refresh | HMAC-SHA256 signé |
| Backups | AES-256 |

---

## 3. Headers de Sécurité HTTP

| Header | Valeur | Protection |
|--------|--------|------------|
| `X-Content-Type-Options` | `nosniff` | MIME sniffing |
| `X-Frame-Options` | `DENY` | Clickjacking |
| `X-XSS-Protection` | `1; mode=block` | XSS (legacy) |
| `Content-Security-Policy` | `default-src 'self'` | XSS, injection |
| `Strict-Transport-Security` | `max-age=31536000; includeSubDomains` | Downgrade HTTPS |
| `Referrer-Policy` | `strict-origin-when-cross-origin` | Fuite referer |
| `Permissions-Policy` | `geolocation=(), microphone=()` | APIs sensibles |

---

## 4. Configuration CORS

```yaml
Access-Control-Allow-Origin: https://app.timemanager.example.com  # PAS *
Access-Control-Allow-Credentials: true
Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS
Access-Control-Allow-Headers: Authorization, Content-Type, X-CSRF-Token
Access-Control-Max-Age: 3600
```

> **Important** : Ne jamais utiliser `*` pour Allow-Origin en production.

---

## 5. Rate Limiting

### 5.1 Limites Globales

| Endpoint | Limite | Par |
|----------|--------|-----|
| Global (authentifié) | 100 req/min | Utilisateur |
| `/auth/login` | 5 req/min | IP |
| `/auth/register` | 1 req/5min | IP |
| `/auth/password-reset` | 3 req/heure | Email |
| `/users/me/data-export` | 1 req/jour | Utilisateur |
| API publique | 30 req/min | IP |

### 5.2 Implémentation

- Via middleware Traefik (labels Docker)
- Stockage des compteurs en Redis
- Réponse 429 avec header `Retry-After`

---

## 6. Isolation Multi-Tenant

### 6.1 Principes

| Règle | Implémentation |
|-------|----------------|
| Isolation des données | Toutes les requêtes filtrées par `organization_id` |
| Cross-tenant = 404 | Accès à une ressource d'une autre org retourne 404 (pas 403) |
| Pas de listing global | Un utilisateur ne voit jamais les données d'autres organisations |

### 6.2 Contrôle d'Accès

```rust
// Middleware appliqué à toutes les routes protégées
fn verify_tenant_access(user: &User, resource: &Resource) -> Result<(), Error> {
    if user.organization_id != resource.organization_id {
        return Err(NotFound);  // 404, pas 403
    }
    Ok(())
}
```

---

## 7. Audit et Traçabilité

### 7.1 Actions Loguées

| Module | Actions |
|--------|---------|
| Auth | login, logout, login_failed, password_reset, password_changed |
| Users | create, update, delete, restore, role_changed, team_assigned |
| Teams | create, update, delete, manager_assigned |
| Absences | create, approve, reject, cancel |
| Clock | correction_request, correction_approve, correction_reject |
| Organization | settings_updated |
| RGPD | data_export, deletion_request, anonymization |

### 7.2 Données Capturées

```rust
pub struct AuditLog {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub actor_id: Option<Uuid>,
    pub action: AuditAction,
    pub resource_type: String,
    pub resource_id: Option<Uuid>,
    pub old_data: Option<Value>,
    pub new_data: Option<Value>,
    pub ip_address: Option<String>,  // Anonymisé après 6 mois
    pub user_agent: Option<String>,  // Supprimé après 6 mois
    pub created_at: DateTime<Utc>,
}
```

---

## 8. Protection contre les Vulnérabilités

### 8.1 OWASP Top 10

| Vulnérabilité | Protection |
|---------------|------------|
| **A01 - Broken Access Control** | RBAC strict, vérification tenant, tests permissions |
| **A02 - Cryptographic Failures** | TLS 1.3, Argon2id, AES-256 |
| **A03 - Injection** | Diesel ORM (requêtes paramétrées), validation entrées |
| **A04 - Insecure Design** | Architecture review, threat modeling |
| **A05 - Security Misconfiguration** | Headers sécurité, CORS strict, CI/CD checks |
| **A06 - Vulnerable Components** | cargo-audit, npm audit, CI hebdomadaire |
| **A07 - Auth Failures** | Brute force protection, password policy |
| **A08 - Software Integrity** | CI/CD vérifications, signatures |
| **A09 - Logging Failures** | Audit logs complets, monitoring |
| **A10 - SSRF** | Validation URLs, whitelist domaines |

### 8.2 Validation des Entrées

| Type | Validation |
|------|------------|
| Email | Format RFC 5322, longueur max 254 |
| Mot de passe | Politique complexité, longueur max 128 |
| Texte libre | Échappement HTML, longueur limitée |
| UUID | Format strict, validation existence |
| Dates | Format ISO 8601, plages cohérentes |
| Fichiers | Types autorisés, taille max, scan antivirus |

---

## 9. Gestion des Dépendances

### 9.1 Outils de Sécurité

| Outil | Fréquence | Action si vulnérabilité |
|-------|-----------|------------------------|
| `cargo audit` | CI + hebdomadaire | Bloquer si critique/high |
| `cargo deny` | CI | Bloquer si licence interdite |
| `npm audit` | CI + hebdomadaire | Bloquer si moderate+ |
| Dependabot | Continu | PR automatique |

### 9.2 Politique de Mise à Jour

| Sévérité | Délai max |
|----------|-----------|
| Critique | 24 heures |
| Haute | 7 jours |
| Moyenne | 30 jours |
| Basse | Prochain sprint |

---

## 10. Incident Response

### 10.1 Classification des Incidents

| Niveau | Description | Exemple |
|--------|-------------|---------|
| P1 - Critique | Compromission système, fuite données | Breach, ransomware |
| P2 - Haute | Vulnérabilité exploitée | Injection réussie |
| P3 - Moyenne | Tentative d'attaque détectée | Brute force massif |
| P4 - Basse | Anomalie détectée | Pic de 429 |

### 10.2 Procédure de Réponse

1. **Détection** : Monitoring, alertes, signalement
2. **Containment** : Isolation du système affecté
3. **Investigation** : Analyse logs, forensics
4. **Éradication** : Correction vulnérabilité
5. **Recovery** : Restauration services
6. **Notification** : CNIL sous 72h si données personnelles (RGPD)
7. **Post-mortem** : Analyse et améliorations

### 10.3 Contacts d'Urgence

| Rôle | Contact |
|------|---------|
| Responsable Sécurité | [À définir] |
| DPO | [À définir] |
| Support technique | [À définir] |
| CNIL (si breach) | notifications@cnil.fr |

---

## 11. Conformité

### 11.1 Réglementations

| Réglementation | Statut |
|----------------|--------|
| RGPD | Conforme (voir RGPD_PROCESSING_REGISTER.md) |
| Code du travail (données temps) | Conforme (rétention 6 ans) |

### 11.2 Audits

| Type | Fréquence |
|------|-----------|
| Revue des accès | Trimestrielle |
| Audit sécurité code | Continue (CI) |
| Pentest externe | Annuel (recommandé) |
| Revue registre RGPD | Semestrielle |

---

## 12. Checklist Avant Production

### Sécurité

- [ ] Email retiré du JWT
- [ ] Brute force protection active
- [ ] Rate limiting configuré
- [ ] Headers sécurité présents
- [ ] CORS configuré (pas de wildcard)
- [ ] TLS 1.3 actif
- [ ] Mots de passe hashés Argon2id
- [ ] Secrets en variables d'environnement (pas dans code)
- [ ] cargo-audit sans vulnérabilités critiques
- [ ] npm audit sans vulnérabilités critiques

### Conformité

- [ ] Registre des traitements à jour
- [ ] Politique de rétention documentée
- [ ] Endpoints RGPD fonctionnels (export, effacement)
- [ ] Procédure notification CNIL documentée

---

## Historique des Révisions

| Version | Date | Modifications | Auteur |
|---------|------|---------------|--------|
| 1.0 | [À compléter] | Création initiale | [Équipe projet] |

---

> **Classification** : Document interne - Ne pas diffuser en dehors de l'organisation
