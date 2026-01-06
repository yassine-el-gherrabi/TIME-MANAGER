# TM-E11 : Audit Logs

## Informations Epic

| Champ | Valeur |
|-------|--------|
| **ID** | TM-E11 |
| **Titre** | Audit Logs |
| **Priorité** | P2 - Moyenne |
| **Estimation globale** | 11 SP |
| **Sprint cible** | Sprint 6 |
| **Dépendances** | TM-E02 (Authentification) |

---

## Description

### Contexte

Les audit logs (journaux d'audit) permettent de tracer toutes les actions sensibles effectuées dans l'application : modifications de données, changements de permissions, validations, etc. Cette traçabilité est essentielle pour la sécurité, la conformité RGPD, et le debugging en production.

### Objectif Business

Fournir une piste d'audit complète permettant de répondre aux questions "qui a fait quoi, quand et comment ?", facilitant ainsi les audits de sécurité, les investigations en cas d'incident, et la conformité réglementaire.

### Valeur Apportée

- **Pour la sécurité** : Détection d'activités suspectes et traçabilité complète
- **Pour la conformité** : Preuves d'audit pour RGPD et audits internes, politique de rétention documentée
- **Pour le support** : Investigation rapide des problèmes signalés
- **Pour les admins** : Visibilité sur toutes les actions de l'organisation

---

## Scope

### Inclus

- Enregistrement automatique des actions sensibles
- Stockage structuré avec métadonnées (IP, user-agent)
- Consultation des logs par les admins
- Filtres et recherche
- Rétention configurable
- Anonymisation automatique des données personnelles (IP, user-agent) après 6 mois
- Job de nettoyage automatique des logs expirés

### Exclus

- Alertes temps réel sur patterns suspects
- Export vers SIEM externe
- Analyse comportementale automatisée
- Logs d'accès (consultations sans modification)
- Replay des actions

---

## Critères de Succès de l'Epic

- [ ] Toutes les actions sensibles sont loguées automatiquement
- [ ] Les logs contiennent : qui, quoi, quand, depuis où
- [ ] Un admin peut consulter les logs de son organisation
- [ ] Les logs sont filtrables par utilisateur, action, date
- [ ] Les logs ne peuvent pas être modifiés (append-only)
- [ ] Super admin peut voir tous les logs
- [ ] Les IPs sont anonymisées après 6 mois (RGPD)
- [ ] Les logs sont purgés après la période de rétention configurée

---

## User Stories

---

### TM-73 : Modèle d'audit log

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur backend,
**Je veux** un modèle de journal d'audit,
**Afin de** stocker les actions sensibles de manière structurée.

#### Contexte Détaillé

Un audit log contient :
- L'organisation concernée
- L'utilisateur qui a effectué l'action
- Le type d'action (create, update, delete, etc.)
- La ressource concernée (type + id)
- Les données avant/après modification (diff)
- Métadonnées de contexte (IP, user-agent)
- Timestamp

#### Critères d'Acceptation

- [ ] Migration table audit_logs créée
- [ ] Enum AuditAction (create, update, delete, login, logout, etc.)
- [ ] Colonnes : actor_id, action, resource_type, resource_id, old_data, new_data
- [ ] Colonnes métadonnées : ip_address, user_agent
- [ ] Index sur organization_id, actor_id, created_at
- [ ] Table partitionnée par mois (optionnel, pour volume)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-73.1 | Créer migration table audit_logs | 1h |
| TM-73.2 | Créer enum AuditAction | 0.5h |
| TM-73.3 | Créer modèle AuditLog | 1h |
| TM-73.4 | Créer AuditLogRepository (insert only) | 1h |
| TM-73.5 | Tests unitaires | 0.5h |

---

### TM-74 : Service d'audit

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur backend,
**Je veux** un service centralisé d'audit,
**Afin de** logger les actions depuis n'importe quel module.

#### Contexte Détaillé

Le AuditService est injecté dans les services métier et appelé aux moments clés. Il extrait automatiquement l'IP et le user-agent depuis le contexte de requête.

Actions à logger :
- Authentification : login, logout, password_reset
- Utilisateurs : create, update, delete, role_change
- Équipes : create, update, delete, member_add, member_remove
- Absences : create, approve, reject, cancel
- Pointages : correction_request, correction_approve, manual_entry
- Organisation : settings_update

#### Critères d'Acceptation

- [ ] AuditService créé avec méthode `log(action, resource, old_data, new_data)`
- [ ] Extraction automatique IP et user-agent depuis RequestContext
- [ ] Sérialisation JSON des données avant/après
- [ ] Intégration dans tous les services concernés
- [ ] Logging asynchrone (ne bloque pas la requête)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-74.1 | Créer AuditService | 1h |
| TM-74.2 | Implémenter extraction contexte requête | 1h |
| TM-74.3 | Intégrer dans UserService | 0.5h |
| TM-74.4 | Intégrer dans AuthService | 0.5h |
| TM-74.5 | Intégrer dans AbsenceService | 0.5h |
| TM-74.6 | Intégrer dans ClockService | 0.5h |
| TM-74.7 | Tests d'intégration | 1h |

---

### TM-75 : Consultation des audit logs

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur,
**Je veux** consulter les journaux d'audit,
**Afin de** suivre les actions effectuées dans mon organisation.

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/audit-logs` créé
- [ ] Filtres : actor_id, action, resource_type, date_range
- [ ] Tri par date (desc par défaut)
- [ ] Pagination obligatoire (max 100 par page)
- [ ] Admin : voit son organisation
- [ ] Super Admin : peut filtrer par organization_id
- [ ] Retour : actor (nom + email), action, resource, timestamp, détails

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-75.1 | Implémenter AuditLogRepository.find_all() avec filtres | 1.5h |
| TM-75.2 | Créer endpoint GET /audit-logs | 1h |
| TM-75.3 | Implémenter scope par rôle | 0.5h |
| TM-75.4 | Tests d'intégration | 1h |

---

### TM-76 : Page audit logs (Frontend)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur,
**Je veux** une interface pour consulter les journaux d'audit,
**Afin de** visualiser l'activité de mon organisation.

#### Critères d'Acceptation

- [ ] Page `/admin/audit-logs` créée
- [ ] Tableau avec colonnes : date, utilisateur, action, ressource, détails
- [ ] Filtres : utilisateur (dropdown), action (dropdown), plage de dates
- [ ] Recherche textuelle optionnelle
- [ ] Pagination avec navigation
- [ ] Clic sur ligne → modal avec détails complets (avant/après)
- [ ] Export CSV
- [ ] Réservé aux admins

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-76.1 | Créer hook useAuditLogs | 1h |
| TM-76.2 | Créer composant AuditLogTable | 1.5h |
| TM-76.3 | Créer composant AuditLogFilters | 1h |
| TM-76.4 | Créer modal AuditLogDetailModal | 1h |
| TM-76.5 | Créer page AuditLogsPage | 1.5h |
| TM-76.6 | Implémenter export CSV | 0.5h |
| TM-76.7 | Tests composants | 1h |

---

### TM-111 : Rétention et anonymisation des audit logs

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant que** responsable conformité,
**Je veux** une gestion automatique de la rétention des logs,
**Afin de** respecter les obligations RGPD de minimisation des données.

#### Contexte Détaillé

Les adresses IP et user-agents sont des données personnelles au sens du RGPD. Elles doivent être anonymisées après une période raisonnable tout en conservant la valeur d'audit des logs.

Politique de rétention :
- **0-6 mois** : Logs complets (IP, user-agent, toutes données)
- **6 mois - 2 ans** : Logs avec IP/user-agent anonymisés
- **> 2 ans** : Logs purgés (sauf obligation légale spécifique)

#### Critères d'Acceptation

- [ ] Job cron quotidien de maintenance des logs
- [ ] Anonymisation IP après 6 mois :
  - IPv4 : `192.168.1.100` → `192.168.1.xxx`
  - IPv6 : Anonymisation des 64 derniers bits
- [ ] User-agent : `Mozilla/5.0...` → `[ANONYMIZED]`
- [ ] Purge des logs > 2 ans (configurable par environnement)
- [ ] Configuration via variables d'environnement :
  - `AUDIT_ANONYMIZE_AFTER_DAYS=180`
  - `AUDIT_RETENTION_DAYS=730`
- [ ] Métrique : nombre de logs anonymisés/purgés
- [ ] Log de l'exécution du job dans audit_logs (action système)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-111.1 | Créer service AuditMaintenanceService | 1h |
| TM-111.2 | Implémenter anonymisation IP/user-agent | 1h |
| TM-111.3 | Implémenter purge des logs expirés | 1h |
| TM-111.4 | Configurer job cron avec tokio-cron-scheduler | 1h |
| TM-111.5 | Tests d'intégration | 1h |

---

## Récapitulatif des Estimations

| Story | Titre | SP |
|-------|-------|:--:|
| TM-73 | Modèle d'audit log | 2 |
| TM-74 | Service d'audit | 2 |
| TM-75 | Consultation des audit logs | 2 |
| TM-76 | Page audit logs (Frontend) | 3 |
| TM-111 | Rétention et anonymisation | 2 |
| **Total** | | **11 SP** |

---

## Notes Techniques

### Modèle AuditLog

```rust
pub struct AuditLog {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub actor_id: Option<Uuid>,  // None pour actions système
    pub action: AuditAction,
    pub resource_type: String,   // "user", "absence", "clock_entry"
    pub resource_id: Option<Uuid>,
    pub old_data: Option<serde_json::Value>,
    pub new_data: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

pub enum AuditAction {
    // Auth
    Login,
    Logout,
    LoginFailed,
    PasswordReset,
    PasswordChanged,

    // CRUD générique
    Create,
    Update,
    Delete,

    // Spécifiques
    RoleChanged,
    TeamAssigned,
    AbsenceApproved,
    AbsenceRejected,
    CorrectionApproved,
    CorrectionRejected,
    SettingsUpdated,
}
```

### Pattern d'Utilisation

```rust
// Dans un service
impl UserService {
    pub async fn update_role(&self, user_id: Uuid, new_role: Role) -> Result<User> {
        let old_user = self.repo.find_by_id(user_id)?;
        let updated_user = self.repo.update_role(user_id, new_role)?;

        // Log de l'action
        self.audit_service.log(
            AuditAction::RoleChanged,
            "user",
            Some(user_id),
            Some(json!({"role": old_user.role})),
            Some(json!({"role": updated_user.role})),
        ).await;

        Ok(updated_user)
    }
}
```

### Endpoints Récapitulatif

| Méthode | Endpoint | Permission |
|---------|----------|------------|
| GET | /api/v1/audit-logs | Admin/SuperAdmin |

### Actions à Logger

| Module | Actions |
|--------|---------|
| Auth | login, logout, login_failed, password_reset, password_changed |
| Users | create, update, delete, restore, role_changed, team_assigned |
| Teams | create, update, delete, manager_assigned |
| Absences | create, approve, reject, cancel |
| Clock | correction_request, correction_approve, correction_reject, manual_entry |
| Organization | settings_updated |

### Politique de Rétention des Données

| Phase | Durée | Actions | Données conservées |
|-------|-------|---------|-------------------|
| **Complète** | 0-6 mois | Aucune | Toutes (IP, user-agent, old/new data) |
| **Anonymisée** | 6 mois - 2 ans | Anonymisation IP/UA | Logs sans données personnelles |
| **Purgée** | > 2 ans | Suppression | Aucune |

#### Configuration par Environnement

| Environnement | Anonymisation après | Purge après |
|---------------|--------------------:|------------:|
| Production | 180 jours | 730 jours (2 ans) |
| Staging | 7 jours | 30 jours |
| Développement | 1 jour | 7 jours |

#### Variables de Configuration

```bash
# .env
AUDIT_ANONYMIZE_AFTER_DAYS=180  # Anonymiser IP/UA après 6 mois
AUDIT_RETENTION_DAYS=730        # Purger après 2 ans
AUDIT_CLEANUP_CRON="0 3 * * *"  # Exécution quotidienne à 3h
```

### Considérations RGPD

| Aspect | Implémentation |
|--------|----------------|
| **Minimisation des données** | IP et user-agent anonymisés après 6 mois |
| **Droit d'accès (Art. 15)** | L'utilisateur peut exporter ses logs via /users/me/data-export |
| **Limitation de conservation** | Purge automatique après 2 ans |
| **Données personnelles** | IP addresses traitées comme données personnelles |
| **Base légale** | Intérêt légitime (sécurité) pour les logs d'authentification |

#### Anonymisation des IP

```rust
// Avant : 192.168.1.100
// Après : 192.168.1.xxx

fn anonymize_ipv4(ip: &str) -> String {
    let parts: Vec<&str> = ip.split('.').collect();
    format!("{}.{}.{}.xxx", parts[0], parts[1], parts[2])
}

// IPv6 : Anonymisation des 64 derniers bits
// Avant : 2001:0db8:85a3:0000:0000:8a2e:0370:7334
// Après : 2001:0db8:85a3:0000:xxxx:xxxx:xxxx:xxxx
```

### Actions à Logger

| Module | Actions |
|--------|---------|
| Auth | login, logout, login_failed, password_reset, password_changed |
| Users | create, update, delete, restore, role_changed, team_assigned, data_export, deletion_request |
| Teams | create, update, delete, manager_assigned |
| Absences | create, approve, reject, cancel |
| Clock | correction_request, correction_approve, correction_reject, manual_entry |
| Organization | settings_updated |
| System | audit_maintenance, data_anonymization |
