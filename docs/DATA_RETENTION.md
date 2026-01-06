# Politique de Rétention des Données - Time Manager

> Conformité RGPD - Principe de limitation de conservation (Article 5.1.e)

## Vue d'Ensemble

Ce document définit les durées de conservation des données personnelles traitées par l'application Time Manager, conformément au principe de minimisation du RGPD et aux obligations légales applicables.

---

## 1. Tableau Récapitulatif

| Type de données | Rétention active | Anonymisation | Suppression | Base légale |
|-----------------|------------------|---------------|-------------|-------------|
| **Pointages** | 6 ans | Après 6 ans | Après anonymisation | Code du travail |
| **Absences** | 6 ans | Après 6 ans | Après anonymisation | Code du travail |
| **Comptes utilisateurs** | Durée contrat | 30 jours grâce | Anonymisation | Contrat travail |
| **Audit logs** | 6 mois complets | 6-24 mois | > 24 mois | Intérêt légitime |
| **Sessions/Tokens** | 30 jours max | N/A | Après expiration | Intérêt légitime |
| **Logs connexion** | 90 jours | Après 90 jours | Après anonymisation | Sécurité |

---

## 2. Détail par Type de Données

### 2.1 Données de Pointage (Clock Entries)

| Phase | Durée | État des données | Action automatique |
|-------|-------|------------------|-------------------|
| **Active** | 0-6 ans | Complètes | Aucune |
| **Archive** | 6 ans | Anonymisées | user_id → hash, commentaires supprimés |
| **Suppression** | > 6 ans | N/A | Possible si non requis légalement |

**Base légale** : Article L3171-3 du Code du travail - Conservation des documents relatifs au décompte du temps de travail pendant 3 à 6 ans selon interprétation.

**Données anonymisées conservées** :
- Horodatages (entrée/sortie)
- Durées agrégées
- ID organisation (pour statistiques)

**Données supprimées** :
- Lien vers l'utilisateur
- Commentaires et notes
- Adresse IP de pointage

### 2.2 Données d'Absence (Absences)

| Phase | Durée | État des données |
|-------|-------|------------------|
| **Active** | 0-6 ans | Complètes |
| **Archive** | 6 ans | Anonymisées |
| **Suppression** | > 6 ans | Selon politique organisation |

**Données anonymisées conservées** :
- Type d'absence (catégorie générique)
- Dates de début/fin
- Durée

**Données supprimées** :
- Lien vers l'utilisateur
- Motifs et commentaires
- Workflow de validation (approbateur)

### 2.3 Comptes Utilisateurs

| Phase | Durée | État des données |
|-------|-------|------------------|
| **Actif** | Durée du contrat | Complètes |
| **Désactivé** | 30 jours grâce | Complètes, accès bloqué |
| **Anonymisé** | > 30 jours | Pseudonymisées |

**Processus d'anonymisation** :

```rust
// Avant anonymisation
User {
    id: "550e8400-...",
    email: "jean.dupont@company.com",
    first_name: "Jean",
    last_name: "Dupont",
    // ...
}

// Après anonymisation
User {
    id: "550e8400-...",  // Conservé pour intégrité référentielle
    email: null,
    first_name: "Utilisateur",
    last_name: "Anonyme",
    deleted_at: "2024-01-15T00:00:00Z",
    anonymized_at: "2024-02-15T00:00:00Z",
    // ...
}
```

### 2.4 Audit Logs

| Phase | Durée | État des données | Job automatique |
|-------|-------|------------------|-----------------|
| **Complète** | 0-6 mois | IP, User-Agent, toutes données | Aucun |
| **Anonymisée** | 6-24 mois | IP masquée, UA supprimé | Quotidien 3h |
| **Purgée** | > 24 mois | Suppression | Quotidien 3h |

**Anonymisation IP** :

```rust
// IPv4: Dernier octet masqué
"192.168.1.100" → "192.168.1.xxx"

// IPv6: 64 derniers bits masqués
"2001:0db8:85a3:0000:0000:8a2e:0370:7334"
→ "2001:0db8:85a3:0000:xxxx:xxxx:xxxx:xxxx"
```

**User-Agent** :
```
"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36..."
→ "[ANONYMIZED]"
```

### 2.5 Sessions et Tokens

| Type | Durée de vie | Révocation |
|------|--------------|------------|
| Access Token (JWT) | 15 minutes | Non révocable (stateless) |
| Refresh Token | 7 jours | Révocable (base de données) |
| Session active | 30 jours max inactivité | Auto-expiration |

**Nettoyage automatique** :
- Job quotidien : suppression des refresh tokens expirés
- Limite : max 5 sessions simultanées (FIFO)

### 2.6 Logs de Connexion

| Phase | Durée | Données |
|-------|-------|---------|
| **Complète** | 0-90 jours | IP, User-Agent, succès/échec |
| **Anonymisée** | > 90 jours | IP masquée, compteurs uniquement |

---

## 3. Configuration par Environnement

### 3.1 Variables d'Environnement

```bash
# Production
AUDIT_ANONYMIZE_AFTER_DAYS=180    # 6 mois
AUDIT_RETENTION_DAYS=730          # 2 ans
AUDIT_CLEANUP_CRON="0 3 * * *"    # Tous les jours à 3h

USER_DELETION_GRACE_DAYS=30       # Délai de grâce
SESSION_MAX_AGE_DAYS=30           # Durée max session
REFRESH_TOKEN_DAYS=7              # Durée refresh token

# Staging
AUDIT_ANONYMIZE_AFTER_DAYS=7
AUDIT_RETENTION_DAYS=30
USER_DELETION_GRACE_DAYS=7

# Développement
AUDIT_ANONYMIZE_AFTER_DAYS=1
AUDIT_RETENTION_DAYS=7
USER_DELETION_GRACE_DAYS=1
```

### 3.2 Jobs de Maintenance

| Job | Fréquence | Heure | Action |
|-----|-----------|-------|--------|
| `audit_anonymize` | Quotidien | 03:00 | Anonymiser logs > 6 mois |
| `audit_purge` | Quotidien | 03:30 | Supprimer logs > 2 ans |
| `session_cleanup` | Quotidien | 04:00 | Supprimer sessions expirées |
| `user_anonymize` | Quotidien | 04:30 | Anonymiser comptes > 30j après suppression |

---

## 4. Exercice des Droits RGPD

### 4.1 Droit à l'Effacement (Article 17)

**Processus** :

```
1. Demande utilisateur → POST /api/v1/users/me/deletion-request
2. Email confirmation envoyé
3. Délai de grâce : 30 jours
   └─ Annulation possible par admin ou utilisateur
4. Anonymisation automatique après délai
5. Email de confirmation finale
```

**Données NON supprimées** (obligation légale) :
- Pointages anonymisés (6 ans)
- Audit logs anonymisés (2 ans)
- ID utilisateur (intégrité référentielle)

### 4.2 Droit d'Accès et Portabilité (Articles 15 & 20)

**Endpoint** : `GET /api/v1/users/me/data-export`

**Formats disponibles** :
- JSON (défaut)
- CSV

**Contenu de l'export** :
```json
{
  "user": {
    "email": "jean.dupont@company.com",
    "first_name": "Jean",
    "last_name": "Dupont",
    "created_at": "2023-01-15"
  },
  "clock_entries": [...],
  "absences": [...],
  "leave_balances": [...],
  "audit_logs": [...],  // Actions de l'utilisateur uniquement
  "export_date": "2024-01-15T10:30:00Z"
}
```

---

## 5. Implémentation Technique

### 5.1 Service de Maintenance

```rust
pub struct AuditMaintenanceService {
    config: RetentionConfig,
    audit_repo: AuditLogRepository,
}

impl AuditMaintenanceService {
    /// Anonymise les logs plus anciens que la période configurée
    pub async fn anonymize_old_logs(&self) -> Result<u64> {
        let cutoff = Utc::now() - Duration::days(self.config.anonymize_after_days);

        let count = self.audit_repo
            .anonymize_before_date(cutoff)
            .await?;

        // Log de l'action système
        self.audit_repo.insert(AuditLog {
            actor_id: None,  // Action système
            action: AuditAction::SystemMaintenance,
            resource_type: "audit_logs".to_string(),
            new_data: Some(json!({
                "anonymized_count": count,
                "cutoff_date": cutoff
            })),
            ..Default::default()
        }).await?;

        Ok(count)
    }

    /// Supprime les logs plus anciens que la période de rétention
    pub async fn purge_expired_logs(&self) -> Result<u64> {
        let cutoff = Utc::now() - Duration::days(self.config.retention_days);

        let count = self.audit_repo
            .delete_before_date(cutoff)
            .await?;

        Ok(count)
    }
}
```

### 5.2 Configuration Job Cron

```rust
// main.rs ou scheduler.rs
use tokio_cron_scheduler::{Job, JobScheduler};

async fn setup_maintenance_jobs(scheduler: &JobScheduler) {
    // Anonymisation quotidienne à 3h
    scheduler.add(
        Job::new_async("0 0 3 * * *", |_, _| {
            Box::pin(async {
                audit_maintenance_service.anonymize_old_logs().await?;
            })
        }).unwrap()
    ).await.unwrap();

    // Purge quotidienne à 3h30
    scheduler.add(
        Job::new_async("0 30 3 * * *", |_, _| {
            Box::pin(async {
                audit_maintenance_service.purge_expired_logs().await?;
            })
        }).unwrap()
    ).await.unwrap();
}
```

---

## 6. Monitoring et Alertes

### 6.1 Métriques

| Métrique | Description | Seuil alerte |
|----------|-------------|--------------|
| `audit_logs_anonymized` | Logs anonymisés (dernier job) | N/A |
| `audit_logs_purged` | Logs purgés (dernier job) | N/A |
| `audit_logs_total` | Total logs en base | > 10M |
| `job_maintenance_duration` | Durée du job | > 5 minutes |
| `job_maintenance_failure` | Échec du job | > 0 |

### 6.2 Alertes

| Condition | Action |
|-----------|--------|
| Job échoue 3x consécutifs | Alerte Slack/Email |
| Durée job > 10 minutes | Alerte performance |
| Volume logs anormal | Investigation |

---

## 7. Procédures Manuelles

### 7.1 Suppression Manuelle Exceptionnelle

En cas de demande légale ou judiciaire :

```bash
# 1. Documenter la demande (ticket, référence légale)
# 2. Exécuter avec supervision
cargo run --bin admin -- purge-user-data \
  --user-id "550e8400-..." \
  --reason "Court order #12345" \
  --operator "admin@company.com"

# 3. Générer rapport de suppression
# 4. Archiver le rapport
```

### 7.2 Restauration (Période de Grâce)

```bash
# Annuler une demande de suppression
cargo run --bin admin -- cancel-deletion \
  --user-id "550e8400-..." \
  --reason "User request" \
  --operator "admin@company.com"
```

---

## 8. Conformité et Audit

### 8.1 Vérifications Périodiques

| Fréquence | Action |
|-----------|--------|
| Mensuelle | Vérifier exécution jobs maintenance |
| Trimestrielle | Audit échantillon données anonymisées |
| Annuelle | Revue complète politique rétention |

### 8.2 Documentation

Chaque exécution de job génère un log :

```json
{
  "job": "audit_maintenance",
  "timestamp": "2024-01-15T03:00:00Z",
  "actions": {
    "anonymized": 1523,
    "purged": 892
  },
  "duration_ms": 45230,
  "status": "success"
}
```

---

## Historique des Révisions

| Version | Date | Modifications | Auteur |
|---------|------|---------------|--------|
| 1.0 | [À compléter] | Création initiale | [Équipe projet] |

---

## Références

- [RGPD Article 5.1.e](https://eur-lex.europa.eu/legal-content/FR/TXT/?uri=CELEX:32016R0679) - Principe de limitation de conservation
- [Code du travail L3171-3](https://www.legifrance.gouv.fr/) - Conservation documents temps de travail
- [CNIL - Durées de conservation](https://www.cnil.fr/fr/les-durees-de-conservation-des-donnees)
