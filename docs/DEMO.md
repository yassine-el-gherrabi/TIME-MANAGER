# Guide de Demonstration - Time Manager

> **Date de presentation**: 12 janvier 2026
> **Duree totale**: 10 minutes (4 flows x 2:30)

---

## Credentials

| Role | Email | Mot de passe |
|------|-------|--------------|
| **Super Admin** | `demo@timemanager.com` | `Password123!` |
| **Admin** | `sophie.bernard@demo.com` | `Password123!` |
| **Manager** | `jean.dupont@demo.com` | `Password123!` |
| **Employee** | `marie.martin@demo.com` | `Password123!` |

---

## Flow 1: Employee - Marie Martin (2:30)

**Login**: `marie.martin@demo.com`

| Temps | Action | Feature a montrer |
|-------|--------|-------------------|
| 0:00 | Dashboard | KPI cards (Hours, Punctuality, Days Worked) |
| 0:30 | Charts | Naviguer les semaines precedentes (Bar Chart historique) |
| 1:00 | Clock In | Demarrer le timer live |
| 1:30 | Clock History | Liste des pointages + vue Calendar |
| 2:00 | Absences | Voir les soldes conges (Leave Balances) |
| 2:15 | Nouvelle demande | Creer une demande de conge |

**Points cles**:
- 6 mois d'historique de pointages
- Solde conges: 25 jours (8 utilises)
- Vue employee simplifiee

---

## Flow 2: Manager - Jean Dupont (2:30)

**Login**: `jean.dupont@demo.com`

| Temps | Action | Feature a montrer |
|-------|--------|-------------------|
| 0:00 | Dashboard | Presence Widget (employes presents/absents) |
| 0:30 | Pending Clock Approvals | Liste des pointages en attente |
| 1:00 | Approuver/Rejeter | Workflow d'approbation (2 approuves, 1 rejete) |
| 1:30 | Pending Absences | Demandes de conges en attente |
| 2:00 | Approuver absence | Valider une demande |
| 2:15 | Team Calendar | Vue calendrier avec absences colorees |

**Points cles**:
- Equipe Tech: 12 membres
- ~100 pointages en attente d'approbation
- Gestion des absences de l'equipe

---

## Flow 3: Admin - Sophie Bernard (2:30)

**Login**: `sophie.bernard@demo.com`

| Temps | Action | Feature a montrer |
|-------|--------|-------------------|
| 0:00 | Sidebar | Navigation Admin elargie (role-based) |
| 0:30 | Users | Liste de 44 utilisateurs + filtres |
| 1:00 | Teams | 6 departements configures |
| 1:30 | Schedules | 5 horaires de travail differents |
| 2:00 | Closed Days | Jours feries 2025-2026 |
| 2:15 | Absence Types | 6 types d'absence avec couleurs |

**Points cles**:
- Configuration complete de l'organisation
- Gestion des utilisateurs et equipes
- Parametrage des horaires et absences

---

## Flow 4: Super Admin (2:30)

**Login**: `demo@timemanager.com`

| Temps | Action | Feature a montrer |
|-------|--------|-------------------|
| 0:00 | Dashboard | KPIs globaux organisation |
| 0:30 | Audit Logs | Liste de 250+ entrees |
| 1:00 | Filtrer logs | Par entity type (users/clocks/absences) |
| 1:30 | Detail log | Voir old/new values en JSON |
| 2:00 | Organizations | Vue multi-tenant |
| 2:15 | Recap | Wrap-up des features |

**Points cles**:
- Tracabilite complete (audit logs)
- Vision globale de l'organisation
- Acces a toutes les fonctionnalites

---

## Donnees de demo

| Element | Quantite |
|---------|----------|
| Utilisateurs | 44 |
| Equipes | 6 |
| Clock entries | ~5500 (6 mois) |
| Audit logs | 250 |
| Absences | 10 |
| Leave balances | 148 |

### Organisation

**Nom**: TechCorp France
**Timezone**: Europe/Paris

### Equipes

| Equipe | Manager | Membres |
|--------|---------|---------|
| Tech | Jean Dupont | 12 |
| Sales | Pierre Leroy | 8 |
| Marketing | Claire Dubois | 6 |
| HR | Sophie Bernard | 4 |
| Finance | Marc Mercier | 5 |
| Support | Alice Moreau | 7 |

### Horaires de travail

| Nom | Heures | Jours | Horaires |
|-----|--------|-------|----------|
| Standard 35h | 35h | Lun-Ven | 09:00-17:00 |
| Flexible 40h | 40h | Lun-Ven | 08:00-17:00 |
| Mi-temps Matin | 20h | Lun-Ven | 09:00-13:00 |
| Mi-temps 3j | 24h | Lun/Mer/Ven | 09:00-17:00 |
| Support Soir | 35h | Lun-Ven | 14:00-22:00 |

### Types d'absence

| Type | Couleur | Approbation requise |
|------|---------|---------------------|
| Conges Payes (CP) | Vert | Oui |
| Maladie (MAL) | Rouge | Oui |
| Sans Solde (SS) | Orange | Oui |
| Teletravail (TT) | Bleu | Non |
| Formation (FOR) | Violet | Oui |
| RTT | Rose | Oui |

---

## Checklist avant demo

- [ ] Verifier que l'application est accessible
- [ ] Tester chaque login
- [ ] Verifier que les donnees sont presentes (clock entries, audit logs)
- [ ] Preparer les 4 onglets de navigateur (1 par persona)
- [ ] Chronometrer chaque flow (2:30 max)

---

## URLs

| Service | URL |
|---------|-----|
| Application | http://localhost:8000 |
| API | http://localhost:8000/api |
| pgAdmin (dev) | http://localhost:5050 |
| Mailpit (dev) | http://localhost:8025 |
