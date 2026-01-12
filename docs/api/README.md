# API Reference

> Documentation de l'API REST Time Manager

---

## Vue d'ensemble

- **Base URL** : `/api/v1`
- **Format** : JSON
- **Auth** : Bearer JWT
- **Versioning** : URL path (`/v1/`)

---

## Authentification

### Header

```http
Authorization: Bearer <access_token>
```

### Obtenir un token

```bash
curl -X POST http://localhost:8000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "password": "password"}'
```

### Réponse

```json
{
  "access_token": "eyJhbG...",
  "token_type": "Bearer",
  "expires_in": 900
}
```

> Le refresh token est envoyé en cookie HttpOnly.

---

## Endpoints par domaine

| Domaine | Base Path | Description |
|---------|-----------|-------------|
| [Auth](#auth) | `/auth/*` | Authentification |
| [Users](#users) | `/users/*` | Gestion utilisateurs |
| [Teams](#teams) | `/teams/*` | Gestion équipes |
| [Clocks](#clocks) | `/clocks/*` | Pointage |
| [Absences](#absences) | `/absences/*` | Congés |
| [Schedules](#schedules) | `/schedules/*` | Plannings |
| [KPIs](#kpis) | `/kpis/*` | Analytics |

---

## Swagger

📄 **[openapi.yaml](./openapi.yaml)** - Spécification OpenAPI 3.0 complète

### Visualiser

```bash
# Avec Docker
docker run -p 8080:8080 \
  -e SWAGGER_JSON=/api/openapi.yaml \
  -v $(pwd)/docs/api:/api \
  swaggerapi/swagger-ui

# Ouvrir http://localhost:8080
```

---

## Pagination

### Paramètres

| Param | Type | Default | Description |
|-------|------|---------|-------------|
| `page` | int | 1 | Numéro de page |
| `per_page` | int | 20 | Items par page (max 100) |

### Réponse

```json
{
  "data": [...],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 150,
    "total_pages": 8
  }
}
```

---

## Codes d'erreur

| Code | Signification |
|------|---------------|
| `200` | Succès |
| `201` | Créé |
| `204` | Pas de contenu |
| `400` | Requête invalide |
| `401` | Non authentifié |
| `403` | Accès refusé |
| `404` | Non trouvé |
| `409` | Conflit |
| `422` | Validation échouée |
| `429` | Rate limit |
| `500` | Erreur serveur |

### Format d'erreur

```json
{
  "error": "validation_error",
  "message": "Invalid email format",
  "details": {
    "field": "email",
    "code": "invalid_format"
  }
}
```

---

## Exemples rapides

### Login

```bash
curl -X POST http://localhost:8000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "admin@acme.local", "password": "Admin123!"}'
```

### Liste utilisateurs

```bash
curl http://localhost:8000/api/v1/users \
  -H "Authorization: Bearer <token>"
```

### Clock In

```bash
curl -X POST http://localhost:8000/api/v1/clocks/in \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{"notes": "Morning shift"}'
```

### Créer absence

```bash
curl -X POST http://localhost:8000/api/v1/absences \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "absence_type_id": "uuid",
    "start_date": "2024-02-01",
    "end_date": "2024-02-05",
    "reason": "Vacances"
  }'
```

---

## Rate Limiting

| Endpoint | Limite | Fenêtre |
|----------|--------|---------|
| `/auth/login` | 5 | 1 min |
| `/auth/password/*` | 3 | 1 hour |
| Global | 100 | 1 sec |

### Headers de réponse

```http
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1705320000
```

---

## Liens connexes

- [Auth Flow](../features/auth-flow.md)
- [RBAC](../features/rbac.md)
- [Error Codes](./error-codes.md)
