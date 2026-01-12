# Quickstart

> Démarrage en 5 minutes avec Time Manager

---

## Étape 1 : Cloner

```bash
git clone <repository-url>
cd time-manager
```

---

## Étape 2 : Configurer

```bash
cp .env.dev.example .env
```

> 💡 La configuration par défaut fonctionne immédiatement.

---

## Étape 3 : Lancer

```bash
docker compose --profile dev up -d
```

**Résultat attendu :**

```
[+] Running 6/6
 ✔ Network timemanager-network    Created
 ✔ Container timemanager-postgres Started
 ✔ Container timemanager-backend  Started
 ✔ Container timemanager-frontend Started
 ✔ Container timemanager-traefik  Started
 ✔ Container timemanager-mailpit  Started
```

---

## Étape 4 : Vérifier

```bash
# Attendre que le backend soit prêt
docker compose logs -f backend
```

**Attendez ce message :**

```
INFO time_manager: 🚀 Server running on 0.0.0.0:8080
```

---

## Étape 5 : Accéder

Ouvrez votre navigateur : **http://localhost:8000**

---

## Connexion

### Compte Admin

| Champ | Valeur |
|-------|--------|
| Email | `admin@acme.local` |
| Password | `Admin123!` |

### Autres comptes

| Role | Email | Password |
|------|-------|----------|
| Super Admin | `superadmin@timemanager.local` | `SuperAdmin123!` |
| Manager | `manager@acme.local` | `Manager123!` |
| Employee | `employee@acme.local` | `Employee123!` |

---

## Premier tour

### 1. Dashboard

Après connexion, vous arrivez sur le dashboard avec :
- Statistiques de présence
- Actions rapides (clock in/out)
- Notifications récentes

### 2. Pointage

Cliquez sur "Clock In" pour pointer votre arrivée.

### 3. Absences

Menu "Absences" → "Nouvelle demande" pour créer une demande de congé.

### 4. Administration

(Admin/Manager) Menu "Admin" pour gérer utilisateurs et équipes.

---

## Services disponibles

| Service | URL | Usage |
|---------|-----|-------|
| 🏠 Application | http://localhost:8000 | App principale |
| 📧 Mailpit | http://localhost:8025 | Voir les emails |
| 🗄️ pgAdmin | http://localhost:5050 | Admin PostgreSQL |
| 🔀 Traefik | http://localhost:8081 | Dashboard proxy |

### Accès pgAdmin

| Champ | Valeur |
|-------|--------|
| Email | `admin@timemanager.dev` |
| Password | `admin` |

**Connexion à la DB :**
- Host : `postgres`
- Port : `5432`
- Database : `timemanager`
- User : `timemanager`
- Password : `devpassword`

---

## Commandes utiles

```bash
# Voir les logs
docker compose logs -f

# Logs d'un service
docker compose logs -f backend

# Restart un service
docker compose restart backend

# Arrêter tout
docker compose down

# Arrêter + supprimer les données
docker compose down -v
```

---

## Activer le monitoring

```bash
# Arrêter les services actuels
docker compose down

# Relancer avec monitoring
docker compose --profile dev --profile monitoring up -d
```

**Nouveaux services :**

| Service | URL |
|---------|-----|
| Grafana | http://localhost:3001 |
| Prometheus | http://localhost:9090 |

**Grafana login :** `admin` / `admin`

---

## Problèmes fréquents

### Le backend ne démarre pas

```bash
# Vérifier les logs
docker compose logs backend

# Vérifier la DB
docker compose exec postgres pg_isready
```

### Page blanche sur localhost:8000

```bash
# Vérifier le frontend
docker compose logs frontend

# Vérifier Traefik
docker compose logs traefik
```

### Emails non reçus

1. Vérifier que Mailpit est démarré
2. Aller sur http://localhost:8025
3. Les emails sont capturés localement

---

## Prochaines étapes

1. **Explorer l'API** → [API Documentation](../api/)
2. **Comprendre l'auth** → [Auth Flow](../features/auth-flow.md)
3. **Configurer production** → [CD Pipeline](../devops/cd-pipeline.md)

---

## Liens connexes

- [Installation détaillée](./installation.md)
- [Configuration](./configuration.md)
- [Architecture](../architecture/)
