# TM-E09 : Rapports & KPIs

## Informations Epic

| Champ | Valeur |
|-------|--------|
| **ID** | TM-E09 |
| **Titre** | Rapports & KPIs |
| **Priorité** | P2 - Moyenne |
| **Estimation globale** | 21 SP |
| **Sprint cible** | Sprint 5-6 |
| **Dépendances** | TM-E07 (Pointage), TM-E08 (Absences) |

---

## Description

### Contexte

Les KPIs (Key Performance Indicators) permettent de mesurer et visualiser les métriques clés de gestion du temps. Ils offrent une vue synthétique aux différents niveaux : employé (ses propres stats), manager (son équipe), admin (toute l'organisation). Cette fonctionnalité est un critère obligatoire du projet Epitech (minimum 2 KPIs).

### Objectif Business

Fournir des indicateurs de performance exploitables pour améliorer la gestion du temps, identifier les anomalies (retards fréquents, heures supplémentaires excessives) et optimiser la planification des ressources.

### Valeur Apportée

- **Pour les employés** : Suivi personnel des heures et de la ponctualité
- **Pour les managers** : Vue consolidée de l'équipe, détection des problèmes
- **Pour les admins** : Métriques organisationnelles pour décisions stratégiques
- **Pour la conformité** : Rapports exportables pour audit et obligations légales

---

## Scope

### Inclus

- Dashboard avec KPIs principaux
- KPI 1 : Taux de ponctualité
- KPI 2 : Écart heures travaillées vs théoriques
- Graphiques de tendance (courbes, barres)
- Filtres temporels (jour, semaine, mois, trimestre)
- Export des données (CSV)
- Vue personnelle, équipe, organisation

### Exclus

- Prédictions et analyses IA
- Rapports automatiques planifiés
- Export PDF formaté
- Comparaison inter-organisations
- Intégration BI externe (Power BI, Tableau)

---

## Critères de Succès de l'Epic

- [ ] Au moins 2 KPIs sont calculés et affichés
- [ ] Les données sont filtrables par période
- [ ] Un employé voit ses propres KPIs
- [ ] Un manager voit les KPIs agrégés de son équipe
- [ ] Un admin voit les KPIs de toute l'organisation
- [ ] Les graphiques sont interactifs et responsives
- [ ] Les données sont exportables en CSV

---

## User Stories

---

### TM-56 : KPI - Taux de ponctualité

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur connecté,
**Je veux** voir mon taux de ponctualité,
**Afin de** suivre ma discipline horaire.

#### Contexte Détaillé

Le taux de ponctualité est calculé comme :
```
punctuality_rate = (arrivées_à_l'heure / jours_travaillés) × 100
```

Une arrivée est "à l'heure" si le clock_in est avant ou dans la tolérance de l'heure de début prévue (configurable, ex : 5 min). Les jours d'absence et jours fériés sont exclus du calcul.

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/reports/punctuality` créé
- [ ] Paramètres : user_id (optionnel), team_id (optionnel), start_date, end_date
- [ ] Retourne : taux global, détail par jour (on_time/late)
- [ ] Tolérance configurable dans les paramètres horaires
- [ ] Scope automatique selon rôle
- [ ] Inclut le nombre de jours analysés

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-56.1 | Créer ReportService avec calcul ponctualité | 2h |
| TM-56.2 | Implémenter récupération clock_in + schedule | 1h |
| TM-56.3 | Implémenter comparaison avec tolérance | 1h |
| TM-56.4 | Créer endpoint GET /reports/punctuality | 1h |
| TM-56.5 | Tests unitaires du calcul | 1h |
| TM-56.6 | Tests d'intégration | 1h |

---

### TM-57 : KPI - Écart heures travaillées vs théoriques

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur connecté,
**Je veux** voir l'écart entre mes heures travaillées et les heures théoriques,
**Afin de** suivre mon temps de travail effectif.

#### Contexte Détaillé

L'écart est calculé comme :
```
variance = heures_travaillées_réelles - heures_théoriques
```

- Positif : heures supplémentaires
- Négatif : heures manquantes
- Théorique = somme des durées prévues selon les horaires

Les absences approuvées réduisent les heures théoriques.

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/reports/time-variance` créé
- [ ] Paramètres : user_id (optionnel), team_id (optionnel), start_date, end_date
- [ ] Retourne : heures_théoriques, heures_réelles, variance, détail par jour
- [ ] Déduction des absences approuvées
- [ ] Scope automatique selon rôle
- [ ] Format heures en décimal et HH:MM

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-57.1 | Implémenter calcul heures théoriques | 1.5h |
| TM-57.2 | Implémenter calcul heures réelles | 1h |
| TM-57.3 | Implémenter déduction absences | 1h |
| TM-57.4 | Créer endpoint GET /reports/time-variance | 1h |
| TM-57.5 | Tests unitaires du calcul | 1h |
| TM-57.6 | Tests d'intégration | 1h |

---

### TM-58 : Résumé heures par période

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur connecté,
**Je veux** voir un résumé de mes heures par période,
**Afin d'** avoir une vue globale de mon activité.

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/reports/time-summary` créé
- [ ] Paramètres : user_id, team_id, granularity (day/week/month), start_date, end_date
- [ ] Retourne par période : heures_travaillées, jours_présence, jours_absence
- [ ] Scope automatique selon rôle
- [ ] Agrégation correcte par granularité

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-58.1 | Implémenter agrégation par granularité | 1.5h |
| TM-58.2 | Créer endpoint GET /reports/time-summary | 1h |
| TM-58.3 | Tests d'intégration | 1h |

---

### TM-59 : Dashboard personnel

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur connecté,
**Je veux** un endpoint qui agrège mes KPIs personnels,
**Afin d'** alimenter mon tableau de bord en un seul appel.

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/reports/dashboard` créé
- [ ] Paramètres : period (week/month/quarter)
- [ ] Retourne pour l'utilisateur courant :
  - Taux de ponctualité
  - Heures travaillées vs théoriques
  - Jours de présence
  - Jours d'absence
  - Tendance vs période précédente
- [ ] Performance : < 500ms

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-59.1 | Créer DashboardService agrégateur | 1.5h |
| TM-59.2 | Implémenter calcul tendances | 1h |
| TM-59.3 | Créer endpoint GET /reports/dashboard | 0.5h |
| TM-59.4 | Optimiser requêtes pour performance | 1h |
| TM-59.5 | Tests d'intégration | 1h |

---

### TM-60 : Dashboard équipe (manager)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant que** manager,
**Je veux** voir les KPIs agrégés de mon équipe,
**Afin de** suivre la performance collective.

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/reports/dashboard/team` créé
- [ ] Paramètres : team_id (optionnel, défaut = mes équipes), period
- [ ] Retourne :
  - Taux de ponctualité moyen équipe
  - Total heures équipe
  - Répartition présence/absence
  - Top 3 retardataires (anonymisé en option)
- [ ] Accessible aux managers et admins

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-60.1 | Implémenter agrégation équipe | 1.5h |
| TM-60.2 | Créer endpoint GET /reports/dashboard/team | 0.5h |
| TM-60.3 | Tests d'intégration | 1h |

---

### TM-61 : Export CSV des données

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant que** manager ou administrateur,
**Je veux** exporter les données de pointage en CSV,
**Afin de** les analyser dans un tableur ou les archiver.

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/reports/export` créé
- [ ] Paramètres : type (clock_entries/absences/summary), user_id, team_id, date_range
- [ ] Format CSV avec headers explicites
- [ ] Encodage UTF-8 avec BOM pour Excel
- [ ] Content-Disposition pour téléchargement
- [ ] Scope automatique selon rôle

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-61.1 | Créer ExportService avec génération CSV | 1.5h |
| TM-61.2 | Implémenter différents types d'export | 1h |
| TM-61.3 | Créer endpoint GET /reports/export | 0.5h |
| TM-61.4 | Tests d'intégration | 1h |

---

### TM-62 : Widget KPIs personnel (Frontend)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant qu'** employé,
**Je veux** voir mes KPIs sur mon dashboard,
**Afin de** suivre ma performance d'un coup d'œil.

#### Critères d'Acceptation

- [ ] Composants cards pour chaque KPI
- [ ] Taux de ponctualité : pourcentage + indicateur visuel (vert/jaune/rouge)
- [ ] Heures : barre de progression heures réelles vs théoriques
- [ ] Tendance : flèche haut/bas vs période précédente
- [ ] Sélecteur de période (semaine/mois/trimestre)
- [ ] Loading skeletons
- [ ] Responsive (2 colonnes desktop, 1 mobile)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-62.1 | Créer hook useReports | 1h |
| TM-62.2 | Créer composant KPICard générique | 1h |
| TM-62.3 | Créer composant PunctualityKPI | 1h |
| TM-62.4 | Créer composant TimeVarianceKPI | 1h |
| TM-62.5 | Créer composant TrendIndicator | 0.5h |
| TM-62.6 | Intégrer dans DashboardPage | 1h |
| TM-62.7 | Tests composants | 1h |

---

### TM-63 : Graphiques de tendance (Frontend)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur connecté,
**Je veux** voir des graphiques de tendance de mes KPIs,
**Afin de** visualiser mon évolution dans le temps.

#### Contexte Détaillé

Utilisation de Recharts pour les graphiques :
- Courbe de ponctualité sur 30 jours
- Barres heures travaillées par semaine
- Interactif avec tooltips

#### Critères d'Acceptation

- [ ] Graphique courbe pour ponctualité
- [ ] Graphique barres pour heures par semaine
- [ ] Tooltips au survol avec détails
- [ ] Légende claire
- [ ] Responsive (redimensionnement)
- [ ] Animation à l'apparition

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-63.1 | Installer et configurer Recharts | 0.5h |
| TM-63.2 | Créer composant PunctualityChart | 1.5h |
| TM-63.3 | Créer composant HoursBarChart | 1.5h |
| TM-63.4 | Intégrer dans dashboard | 1h |
| TM-63.5 | Tests composants | 1h |

---

### TM-64 : Page rapports manager (Frontend)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant que** manager,
**Je veux** une page de rapports détaillés pour mon équipe,
**Afin d'** analyser les performances individuelles et collectives.

#### Critères d'Acceptation

- [ ] Page `/reports` créée
- [ ] Tableau des membres avec leurs KPIs
- [ ] Tri et filtres par métrique
- [ ] Graphique de comparaison équipe
- [ ] Sélecteur de période
- [ ] Bouton export CSV
- [ ] Vue agrégée équipe + détail individuel

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-64.1 | Créer composant TeamKPITable | 1.5h |
| TM-64.2 | Créer composant TeamComparisonChart | 1.5h |
| TM-64.3 | Créer page ReportsPage | 2h |
| TM-64.4 | Implémenter export CSV frontend | 0.5h |
| TM-64.5 | Tests composants | 1h |

---

## Récapitulatif des Estimations

| Story | Titre | SP |
|-------|-------|:--:|
| TM-56 | KPI - Taux de ponctualité | 3 |
| TM-57 | KPI - Écart heures travaillées vs théoriques | 3 |
| TM-58 | Résumé heures par période | 2 |
| TM-59 | Dashboard personnel | 2 |
| TM-60 | Dashboard équipe (manager) | 2 |
| TM-61 | Export CSV des données | 2 |
| TM-62 | Widget KPIs personnel (Frontend) | 3 |
| TM-63 | Graphiques de tendance (Frontend) | 3 |
| TM-64 | Page rapports manager (Frontend) | 3 |
| **Total** | | **23 SP** |

---

## Notes Techniques

### Formules de Calcul

#### Taux de Ponctualité
```rust
pub fn calculate_punctuality_rate(
    clock_entries: &[ClockEntry],
    schedules: &[DaySchedule],
    tolerance_minutes: i32
) -> f64 {
    let mut on_time = 0;
    let mut total_days = 0;

    for entry in clock_entries {
        if let Some(schedule) = find_schedule_for_date(entry.clock_in.date(), schedules) {
            let expected_start = schedule.start_time;
            let actual_start = entry.clock_in.time();
            let diff_minutes = (actual_start - expected_start).num_minutes();

            if diff_minutes <= tolerance_minutes {
                on_time += 1;
            }
            total_days += 1;
        }
    }

    if total_days == 0 { 100.0 } else { (on_time as f64 / total_days as f64) * 100.0 }
}
```

#### Écart Heures
```rust
pub fn calculate_time_variance(
    clock_entries: &[ClockEntry],
    schedules: &[DaySchedule],
    absences: &[Absence]
) -> TimeVariance {
    let expected_hours = schedules.iter()
        .filter(|s| !is_absence_day(s.date, absences))
        .map(|s| s.duration_hours())
        .sum::<f64>();

    let actual_hours = clock_entries.iter()
        .filter(|e| e.clock_out.is_some())
        .map(|e| e.duration_hours())
        .sum::<f64>();

    TimeVariance {
        expected: expected_hours,
        actual: actual_hours,
        variance: actual_hours - expected_hours,
    }
}
```

### Endpoints Récapitulatif

| Méthode | Endpoint | Permission |
|---------|----------|------------|
| GET | /api/v1/reports/punctuality | Scoped |
| GET | /api/v1/reports/time-variance | Scoped |
| GET | /api/v1/reports/time-summary | Scoped |
| GET | /api/v1/reports/dashboard | All |
| GET | /api/v1/reports/dashboard/team | Manager/Admin |
| GET | /api/v1/reports/export | Manager/Admin |

### Critères Epitech Couverts

| Critère | Implémentation |
|---------|----------------|
| data_viz | Recharts avec graphiques interactifs |
| Minimum 2 KPIs | Ponctualité + Écart heures |

### Seuils KPIs

| KPI | Bon | Moyen | Mauvais |
|-----|-----|-------|---------|
| Ponctualité | ≥ 95% | 80-95% | < 80% |
| Écart heures | -2h à +2h | -5h à +5h | > ±5h |

### Optimisations Performance

- Cache des calculs par période (Redis ou in-memory)
- Requêtes SQL optimisées avec indexes sur dates
- Agrégation côté BDD quand possible
- Pagination pour exports volumineux
