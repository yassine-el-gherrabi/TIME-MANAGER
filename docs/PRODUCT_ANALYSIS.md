# 📋 TIME MANAGER - ANALYSE PRODUIT COMPLÈTE

> Document de référence pour la conception et l'implémentation du frontend
>
> **Date**: 14 Octobre 2025
> **Version**: 1.0
> **Auteur**: Analyse Claude Code

---

## Table des Matières

1. [User Flows Complets](#1-user-flows-complets)
2. [Stratégie KPIs](#2-stratégie-kpis)
3. [Clock In/Out - Proposition UX](#3-clock-inout---proposition-ux)
4. [Dashboards - Architecture](#4-dashboards---architecture)
5. [Vision End-to-End](#5-vision-end-to-end)
6. [Architecture Technique](#6-architecture-technique)

---

## 1. USER FLOWS COMPLETS

### 1.1 SCÉNARIO: Démarrage d'une Entreprise (Jour 1)

#### Étape 1: Premier Manager / Super Admin

```
Entreprise Acme Corp démarre avec Time Manager

┌─────────────────────────────────────────────────────────┐
│ 1. Installation système (Docker compose up)             │
│ 2. Premier accès: http://localhost:8000                 │
│ 3. Page d'accueil affiche:                             │
│    - "Aucun utilisateur détecté"                       │
│    - Formulaire "Créer le premier administrateur"      │
│                                                         │
│ 4. Le premier utilisateur créé automatiquement:        │
│    - Role: Manager (par défaut)                        │
│    - Accès complet au système                          │
└─────────────────────────────────────────────────────────┘
```

**JUSTIFICATION:**
- Conforme au PDF: "utilisateurs enregistrés par managers"
- Le tout premier compte est créé via setup initial
- Ensuite, seuls les managers peuvent créer des comptes

#### Étape 2: Manager crée son équipe

```
Sarah (Manager) configure son équipe

┌─────────────────────────────────────────────────────────┐
│ DASHBOARD MANAGER - Vue initiale                        │
│                                                         │
│ ┌─────────────────┐  ┌─────────────────┐              │
│ │  👥 Utilisateurs│  │  🏢 Équipes      │              │
│ │     0           │  │     0            │              │
│ └─────────────────┘  └─────────────────┘              │
│                                                         │
│ [+ Ajouter un employé]  [+ Créer une équipe]          │
│                                                         │
│ WORKFLOW:                                               │
│ 1. Cliquer "+ Ajouter un employé"                      │
│ 2. Formulaire:                                          │
│    - Email *                                            │
│    - Prénom *                                           │
│    - Nom *                                              │
│    - Téléphone                                          │
│    - Mot de passe temporaire (auto-généré)            │
│    - Rôle: [Employee ▼] ou [Manager ▼]                │
│    - Envoyer email de bienvenue ✓                      │
│                                                         │
│ 3. L'employé reçoit:                                    │
│    - Email avec credentials temporaires                 │
│    - Lien de première connexion                        │
│    - Instructions de changement de MDP                  │
│                                                         │
│ 4. Créer équipes:                                       │
│    - Nom: "Dev Team Frontend"                          │
│    - Description                                        │
│    - Assigner membres (multi-select)                   │
│    - Assigner un manager                               │
└─────────────────────────────────────────────────────────┘
```

### 1.2 SCÉNARIO: Employé - Premier Jour

```
Marc (Employee) - Premier jour de travail

┌─────────────────────────────────────────────────────────┐
│ JOUR 1 - 9h00                                           │
│                                                         │
│ 1. Accès email de bienvenue                            │
│    "Bienvenue chez Acme Corp!"                         │
│    Username: marc@acme.com                              │
│    Mot de passe temporaire: Acme2024!                  │
│    [Se connecter →]                                     │
│                                                         │
│ 2. Première connexion                                   │
│    ┌─────────────────────────────────────┐            │
│    │ 🔐 Première Connexion                │            │
│    │                                       │            │
│    │ Nouveau mot de passe: [••••••••••]   │            │
│    │ Confirmer: [••••••••••]              │            │
│    │                                       │            │
│    │ [Mettre à jour →]                    │            │
│    └─────────────────────────────────────┘            │
│                                                         │
│ 3. Redirection Dashboard Employee                      │
│    ┌─────────────────────────────────────┐            │
│    │ 👋 Bonjour Marc!                     │            │
│    │ Votre premier jour chez nous         │            │
│    │                                       │            │
│    │ ⏰ VOUS ÊTES: [HORS SERVICE]         │            │
│    │                                       │            │
│    │ ┌──────────────────────────────┐    │            │
│    │ │  🟢 CLOCK IN                  │    │            │
│    │ │  Commencer ma journée         │    │            │
│    │ └──────────────────────────────┘    │            │
│    │                                       │            │
│    │ 📊 Vos statistiques apparaîtront    │            │
│    │    une fois que vous pointez         │            │
│    └─────────────────────────────────────┘            │
└─────────────────────────────────────────────────────────┘
```

### 1.3 SCÉNARIO: Journée Type - Employee

```
Marc - Journée de travail complète

┌─────────────────────────────────────────────────────────┐
│ 9h00 - ARRIVÉE                                          │
│                                                         │
│ Action: Clic sur [🟢 CLOCK IN]                         │
│                                                         │
│ Résultat:                                               │
│ ┌─────────────────────────────────────────┐            │
│ │ ✅ Vous êtes pointé!                     │            │
│ │ 📅 Lundi 14 Oct 2025                     │            │
│ │ ⏰ 09:00:34                               │            │
│ │                                           │            │
│ │ STATUT: [🟢 EN SERVICE] ←────────────── │            │
│ │                                           │            │
│ │ ⏱️ Temps écoulé: 00:00:00 ←─────────── │            │
│ │ (mise à jour en temps réel)              │            │
│ │                                           │            │
│ │ ┌──────────────────────────────┐        │            │
│ │ │  🔴 CLOCK OUT                 │        │            │
│ │ │  Terminer ma journée          │        │            │
│ │ └──────────────────────────────┘        │            │
│ └─────────────────────────────────────────┘            │
│                                                         │
│ 📊 Statistiques du jour (mise à jour live):            │
│ ┌─────────────────────────────────────────┐            │
│ │ Aujourd'hui                              │            │
│ │ ⏱️ 3h 24m                                │            │
│ │                                           │            │
│ │ Cette semaine                            │            │
│ │ ⏱️ 12h 45m / 35h                         │            │
│ │ [████████░░░░░░░░] 36%                  │            │
│ └─────────────────────────────────────────┘            │
│                                                         │
├─────────────────────────────────────────────────────────┤
│ 12h30 - PAUSE DÉJEUNER                                 │
│                                                         │
│ Action: Clic [🔴 CLOCK OUT]                            │
│ Résultat: STATUT → [⚪ HORS SERVICE]                  │
│ Session sauvegardée: 09:00:34 → 12:30:12 (3h29m)      │
│                                                         │
├─────────────────────────────────────────────────────────┤
│ 13h30 - RETOUR PAUSE                                   │
│                                                         │
│ Action: Clic [🟢 CLOCK IN]                             │
│ Nouvelle session commence                               │
│                                                         │
├─────────────────────────────────────────────────────────┤
│ 18h00 - FIN DE JOURNÉE                                 │
│                                                         │
│ Action: Clic [🔴 CLOCK OUT]                            │
│                                                         │
│ ┌─────────────────────────────────────────┐            │
│ │ 🎉 Bonne journée Marc!                   │            │
│ │                                           │            │
│ │ Résumé de votre journée:                 │            │
│ │ ⏱️ Temps total: 8h 03m                   │            │
│ │ 📍 2 sessions:                           │            │
│ │    • 09:00 → 12:30 (3h30)               │            │
│ │    • 13:30 → 18:00 (4h33)               │            │
│ │                                           │            │
│ │ À demain! 👋                             │            │
│ └─────────────────────────────────────────┘            │
└─────────────────────────────────────────────────────────┘
```

### 1.4 SCÉNARIO: Manager - Suivi d'équipe

```
Sarah (Manager) - Monitoring journée

┌─────────────────────────────────────────────────────────┐
│ DASHBOARD MANAGER - Vue Temps Réel                      │
│                                                         │
│ ┌─────────────────────────────────────────────────┐   │
│ │ 🏢 ÉQUIPE: Dev Team Frontend (12 membres)        │   │
│ │                                                   │   │
│ │ 📊 Vue d'aujourd'hui (Lundi 14/10)              │   │
│ │                                                   │   │
│ │ ┌──────────┬────────┬──────────┬────────────┐  │   │
│ │ │ Employé  │ Statut │ Arrivée  │ Temps      │  │   │
│ │ ├──────────┼────────┼──────────┼────────────┤  │   │
│ │ │ 🟢 Marc  │ En srv │ 09:00    │ 8h 03m     │  │   │
│ │ │ 🟢 Julie │ En srv │ 08:45    │ 8h 18m     │  │   │
│ │ │ 🔴 Paul  │ Pause  │ 09:15    │ 6h 12m     │  │   │
│ │ │ ⚪ Anna  │ Absent │ --       │ --         │  │   │
│ │ │ 🟢 Tom   │ En srv │ 09:30    │ 7h 33m  ⚠️ │  │   │
│ │ └──────────┴────────┴──────────┴────────────┘  │   │
│ │                                                   │   │
│ │ ⚠️ Alertes:                                      │   │
│ │ • Anna: Absence non justifiée                   │   │
│ │ • Tom: Arrivé 30min en retard (3ème fois/mois) │   │
│ └─────────────────────────────────────────────────┘   │
│                                                         │
│ 📈 KPIs Hebdomadaires                                   │
│ [Détails dans section KPIs ci-dessous]                 │
└─────────────────────────────────────────────────────────┘
```

---

## 2. STRATÉGIE KPIs

### 2.1 Principe de Design des KPIs

```
❌ ÉVITER: Trop de KPIs → Confusion
✅ VISER: 5-7 KPIs essentiels → Clarté

Catégories:
1. PRÉSENCE (combien sont là?)
2. PONCTUALITÉ (sont-ils à l'heure?)
3. PRODUCTIVITÉ (temps de travail effectif)
4. ANOMALIES (problèmes à surveiller)
```

### 2.2 KPIs pour EMPLOYEE Dashboard

```
┌─────────────────────────────────────────────────────────┐
│ 📊 MES STATISTIQUES                                     │
│                                                         │
│ 1️⃣ TEMPS AUJOURD'HUI                                   │
│    ⏱️ 7h 24m / 8h                                       │
│    [████████████████░░] 92%                            │
│    JUSTIFICATION: Info la plus importante pour          │
│    l'employé = combien j'ai travaillé aujourd'hui       │
│                                                         │
│ 2️⃣ PROGRESSION HEBDOMADAIRE                            │
│    ⏱️ 32h 15m / 35h                                     │
│    [████████████████░░] 92%                            │
│    Objectif: 35h (reste 2h 45m)                        │
│    JUSTIFICATION: Vision claire de la semaine           │
│                                                         │
│ 3️⃣ PONCTUALITÉ CE MOIS                                 │
│    ✅ 18 jours à l'heure / 20 jours                     │
│    🟡 2 retards (< 15min)                              │
│    JUSTIFICATION: Feedback positif pour encourager      │
│                                                         │
│ 4️⃣ MOYENNE HEBDOMADAIRE (30 derniers jours)            │
│    📊 34h 12m /semaine                                  │
│    Tendance: ↗️ +2h vs mois dernier                    │
│    JUSTIFICATION: Vision long terme                     │
│                                                         │
│ 5️⃣ HISTORIQUE 7 DERNIERS JOURS                         │
│    Lu Ma Me Je Ve Sa Di                                 │
│    █  █  █  █  █  ░  ░  (Graphique barres)            │
│    8h 8h 7h 8h 9h -- --                                │
│    JUSTIFICATION: Vision graphique rapide               │
└─────────────────────────────────────────────────────────┘
```

### 2.3 KPIs pour MANAGER Dashboard

```
┌─────────────────────────────────────────────────────────┐
│ 📊 KPIS ÉQUIPE - Dev Team Frontend                      │
│                                                         │
│ VUE D'ENSEMBLE (Ce mois)                                │
│                                                         │
│ 1️⃣ TAUX DE PRÉSENCE                                    │
│    📈 94.2% (Moy entreprise: 91.5%)                    │
│    ✅ +2.7% vs moyenne                                  │
│    [████████████████████] 94%                          │
│    JUSTIFICATION: KPI #1 pour un manager                │
│    → Combien de mon équipe est présente?                │
│                                                         │
│ 2️⃣ TAUX DE PONCTUALITÉ                                 │
│    ⏰ 87.3% (arrivent à l'heure)                        │
│    🟡 12.7% de retards                                  │
│    Détail:                                              │
│    • < 15min: 10.2%                                     │
│    • 15-30min: 2.1%                                     │
│    • > 30min: 0.4%                                      │
│    JUSTIFICATION: Identifier les problèmes de           │
│    ponctualité systématiques                            │
│                                                         │
│ 3️⃣ HEURES MOYENNES PAR EMPLOYÉ                         │
│    ⏱️ 35.4h /semaine (Objectif: 35h)                   │
│    Top performers:                                      │
│    • Julie: 38h (+3h)                                   │
│    • Marc: 36.5h (+1.5h)                               │
│    Sous objectif:                                       │
│    • Paul: 32h (-3h) ⚠️                                │
│    JUSTIFICATION: Identifier surcharge ou               │
│    sous-performance                                     │
│                                                         │
│ 4️⃣ HEURES SUPPLÉMENTAIRES                              │
│    💰 124h ce mois (équivalent 4500€)                   │
│    Tendance: ↗️ +34h vs mois dernier                   │
│    Distribution:                                        │
│    • Julie: 48h                                         │
│    • Marc: 32h                                          │
│    • Tom: 44h                                           │
│    JUSTIFICATION: Contrôle budgétaire +                 │
│    prévention burnout                                   │
│                                                         │
│ 5️⃣ ANOMALIES & ALERTES                                 │
│    🚨 3 alertes actives                                 │
│    • Anna: 3 absences injustifiées                     │
│    • Tom: Pattern de retards (8 cette semaine)         │
│    • Paul: Sous objectif heures (-12h ce mois)         │
│    JUSTIFICATION: Intervention proactive manager        │
│                                                         │
│ 6️⃣ PRODUCTIVITÉ ÉQUIPE (Trend)                         │
│    📊 Graphique 90 derniers jours                       │
│    ┌─────────────────────────────┐                     │
│    │      ╱╲    ╱╲                │                     │
│    │     ╱  ╲  ╱  ╲  ╱           │                     │
│    │ ───╱────╲╱────╲╱            │                     │
│    └─────────────────────────────┘                     │
│    Jan  Fév  Mar  Avr  Mai  Juin                       │
│    JUSTIFICATION: Vision macro des tendances            │
│                                                         │
│ 7️⃣ COMPARAISON INTER-ÉQUIPES                           │
│    Classement équipes:                                  │
│    🥇 Dev Backend: 96.1%                                │
│    🥈 Dev Frontend (vous): 94.2%                        │
│    🥉 DevOps: 92.8%                                     │
│    JUSTIFICATION: Benchmarking et motivation            │
└─────────────────────────────────────────────────────────┘

📌 FILTRES DISPONIBLES:
- Période: Aujourd'hui | Cette semaine | Ce mois | Personnalisé
- Équipe: Toutes | Dev Frontend | Dev Backend | DevOps
- Membre: Tous | Sélection individuelle
```

### 2.4 KPIs EXCLUS (Et Pourquoi)

```
❌ Nombre de clics sur clock in/out
   → Pas pertinent pour business

❌ Temps moyen de pause
   → Trop intrusif, pas de valeur ajoutée

❌ Nombre de connexions à l'app
   → Vanity metric

❌ Taux d'utilisation de l'app
   → Évident (tout le monde doit l'utiliser)
```

---

## 3. CLOCK IN/OUT - PROPOSITION UX

### 3.1 Principe de Design

```
OBJECTIF: Pointer en < 3 secondes
- 1 clic pour clock in
- 1 clic pour clock out
- Feedback visuel immédiat
- Status toujours visible
```

### 3.2 Design Desktop

```
┌─────────────────────────────────────────────────────────┐
│ HEADER (toujours visible)                                │
│                                                         │
│ ⏰ Time Manager    👤 Marc Dubois    [🟢 EN SERVICE]   │
│                                      └─> Status badge   │
│                                          toujours       │
│                                          visible        │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│ DASHBOARD - Zone Clock In/Out                           │
│                                                         │
│ ┌─────────────────────────────────────────────────┐   │
│ │ 🟢 VOUS ÊTES EN SERVICE                          │   │
│ │                                                   │   │
│ │ 📅 Lundi 14 Octobre 2025                         │   │
│ │ ⏰ Pointage: 09:00:34                            │   │
│ │                                                   │   │
│ │ ⏱️ TEMPS ÉCOULÉ: 07:24:18 ←─────────────────── │   │
│ │    (mise à jour chaque seconde)                  │   │
│ │                                                   │   │
│ │ ┌───────────────────────────────────────────┐   │   │
│ │ │                                             │   │   │
│ │ │  🔴 CLOCK OUT - Terminer ma journée        │   │   │
│ │ │                                             │   │   │
│ │ └───────────────────────────────────────────┘   │   │
│ │      ↑                                           │   │
│ │      Bouton LARGE (impossible à rater)          │   │
│ │      Couleur adaptée au status                   │   │
│ │                                                   │   │
│ │ 📊 Aujourd'hui: 7h 24m / 8h [████████░░] 92%   │   │
│ └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

### 3.3 Interactions

```
FLOW CLOCK OUT:

1. Clic sur [🔴 CLOCK OUT]
   ↓
2. Animation transition (0.3s)
   ↓
3. Confirmation modale:
   ┌─────────────────────────────────┐
   │ ✅ Vous êtes pointé(e) sortie!  │
   │                                  │
   │ Session terminée:                │
   │ 09:00:34 → 16:24:52             │
   │ Durée: 7h 24m 18s               │
   │                                  │
   │ Bonne soirée! 👋                │
   │                                  │
   │ [OK]                            │
   └─────────────────────────────────┘
   Auto-dismiss après 3 secondes
   ↓
4. Status change:
   Badge header: [⚪ HORS SERVICE]
   Bouton change: [🟢 CLOCK IN]
```

### 3.4 Version Mobile

```
┌──────────────────────┐
│ ⏰ Time Manager      │
│ 👤 Marc  [🟢]        │ ← Status compact
├──────────────────────┤
│                      │
│  🟢 EN SERVICE       │
│                      │
│  ⏱️ 07:24:18         │ ← Gros, lisible
│                      │
│  ┌────────────────┐ │
│  │                 │ │
│  │  🔴 CLOCK OUT  │ │ ← Bouton 60% écran
│  │                 │ │
│  └────────────────┘ │
│                      │
│  📊 7h24 / 8h       │
│  [████████░]        │
│                      │
│  [Voir détails ↓]   │
└──────────────────────┘

OPTIMISATIONS MOBILE:
- Touch target 48x48px minimum
- Pas de hover, tout en tap
- Confirmation slide-up bottom sheet
- Haptic feedback au tap
- Notification push si oubli clock out
  (après 10h de service → "Oubli?")
```

### 3.5 États du Système

```
ÉTATS POSSIBLES:

1️⃣ HORS SERVICE (⚪)
   - Bouton: [🟢 CLOCK IN] (vert)
   - Badge header: ⚪ HORS SERVICE
   - Timer: Masqué
   - Action: 1 clic → Clock In

2️⃣ EN SERVICE (🟢)
   - Bouton: [🔴 CLOCK OUT] (rouge)
   - Badge header: 🟢 EN SERVICE
   - Timer: Affiché et compte
   - Action: 1 clic → Clock Out

JUSTIFICATION:
- 2 états suffisent (simplicité)
- Pause = Clock Out + Clock In
- Évite la complexité inutile
```

### 3.6 Features Avancées

```
📍 GÉOLOCALISATION (Optionnelle)
┌─────────────────────────────────┐
│ Clock In nécessite:              │
│ • Être sur site (GPS check)     │
│ • OU connexion WiFi entreprise  │
│                                  │
│ Si hors zone:                    │
│ ⚠️ "Vous n'êtes pas sur site"   │
│ → Contactez votre manager        │
│    pour pointage exceptionnel    │
└─────────────────────────────────┘

🔔 RAPPELS INTELLIGENTS
- 9h00: "Vous avez oublié de pointer?"
- 18h00: "Toujours en service, tout va bien?"
- Après 10h: "⚠️ Session anormalement longue"

📸 PHOTO OPTIONNELLE (Sécurité)
- Selfie au clock in/out
- Prévention fraude
- Configurable par entreprise

📊 PATTERNS DÉTECTÉS
- Alerte: "Vous pointez tard 3x cette semaine"
- Encouragement: "🎉 10 jours à l'heure d'affilée!"
- Suggestion: "Moyenne arrivée: 9:15, objectif: 9:00"
```

---

## 4. DASHBOARDS - ARCHITECTURE

### 4.1 Dashboard Employee - Layout Complet

```
┌─────────────────────────────────────────────────────────┐
│ HEADER                                                   │
│ ⏰ Time Manager | 👤 Marc Dubois | [🟢 EN SERVICE]      │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│ SECTION 1: CLOCK IN/OUT (Priorité #1)                   │
│ [Voir design complet section 3]                         │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│ SECTION 2: MES STATS RAPIDES                            │
│                                                         │
│ ┌───────────┬───────────┬───────────┬───────────┐     │
│ │Aujourd'hui│Cette      │Ce mois    │Ponctualité│     │
│ │           │semaine    │           │           │     │
│ │  7h 24m   │ 32h / 35h │  128h     │  18/20 ✅ │     │
│ │  /8h      │           │           │           │     │
│ │ [███░]92% │ [████]92% │           │    90%    │     │
│ └───────────┴───────────┴───────────┴───────────┘     │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│ SECTION 3: HISTORIQUE 7 DERNIERS JOURS                  │
│                                                         │
│ 📊 [Graphique en barres]                                │
│     █        █                                          │
│     █   █    █   █    █                                │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━                          │
│  Lu  Ma  Me  Je  Ve  Sa  Di                            │
│  8h  7h  8h  8h  9h  --  --                            │
│                                                         │
│  [< Semaine précédente] [Semaine suivante >]           │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│ SECTION 4: SESSIONS AUJOURD'HUI                         │
│                                                         │
│ ┌─────────────────────────────────────────────────┐   │
│ │ 📅 Lundi 14 Octobre 2025                         │   │
│ │                                                   │   │
│ │ Session 1:                                        │   │
│ │ 🟢 09:00:34 → 12:30:12 (3h 29m 38s)             │   │
│ │                                                   │   │
│ │ Session 2: (en cours)                            │   │
│ │ 🟢 13:30:45 → ... (3h 54m 40s)                   │   │
│ │                                                   │   │
│ │ Total journée: 7h 24m 18s                        │   │
│ └─────────────────────────────────────────────────┘   │
│                                                         │
│  [📅 Voir calendrier complet]                          │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│ SECTION 5: MON PROFIL                                   │
│                                                         │
│ 👤 Marc Dubois                                          │
│ 📧 marc.dubois@acme.com                                 │
│ 📱 +33 6 12 34 56 78                                    │
│ 🏷️ Rôle: Employee                                      │
│ 🏢 Équipe: Dev Team Frontend                            │
│                                                         │
│ [✏️ Modifier profil] [🔒 Changer mot de passe]         │
└─────────────────────────────────────────────────────────┘

NAVIGATION SIDEBAR (Gauche):
┌──────────────┐
│ 🏠 Dashboard │ ← Page actuelle
│ 📅 Calendrier│
│ 📊 Stats     │
│ 👤 Profil    │
│ ⚙️ Paramètres│
│ 🚪 Déconnexion│
└──────────────┘
```

### 4.2 Dashboard Manager - Layout Complet

```
┌─────────────────────────────────────────────────────────┐
│ HEADER                                                   │
│ ⏰ Time Manager | 👤 Sarah Martin (Manager)              │
│ 🏢 Dev Team Frontend (12 membres)                       │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│ SECTION 1: VUE TEMPS RÉEL ÉQUIPE                        │
│                                                         │
│ ┌─────────────────────────────────────────────────┐   │
│ │ 🟢 En service: 8    🔴 Pause: 2    ⚪ Absents: 2  │   │
│ │                                                   │   │
│ │ ┌────────┬────────┬──────┬───────┬─────────┐   │   │
│ │ │Employé │Statut  │Entrée│Temps  │Actions  │   │   │
│ │ ├────────┼────────┼──────┼───────┼─────────┤   │   │
│ │ │🟢 Marc │En srv  │09:00 │7h 24m │[Voir]   │   │   │
│ │ │🟢 Julie│En srv  │08:45 │7h 39m │[Voir]   │   │   │
│ │ │🔴 Paul │Pause   │09:15 │6h 12m │[Voir]   │   │   │
│ │ │⚪ Anna │Absent  │--    │--     │[Contact]│   │   │
│ │ │🟢 Tom  │En srv  │09:30 │6h 54m⚠│[Voir]   │   │   │
│ │ └────────┴────────┴──────┴───────┴─────────┘   │   │
│ │                                                   │   │
│ │ 📊 Moyenne équipe aujourd'hui: 7h 12m            │   │
│ └─────────────────────────────────────────────────┘   │
│                                                         │
│ ⚠️ ALERTES: 3                                           │
│ • Anna: Absence non justifiée                          │
│ • Tom: Retard 30min (3ème fois cette semaine)         │
│ • Paul: Temps insuffisant ce mois (-8h)               │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│ SECTION 2: KPIs ÉQUIPE (Cette semaine)                  │
│                                                         │
│ [Voir KPIs détaillés section 2]                         │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│ SECTION 3: GESTION RAPIDE                               │
│                                                         │
│ ┌─────────────────┐  ┌─────────────────┐              │
│ │ + Ajouter       │  │ 🏢 Gérer        │              │
│ │   Employé       │  │    Équipes      │              │
│ └─────────────────┘  └─────────────────┘              │
│                                                         │
│ ┌─────────────────┐  ┌─────────────────┐              │
│ │ 📊 Générer      │  │ 📧 Envoyer      │              │
│ │    Rapport      │  │    Message      │              │
│ └─────────────────┘  └─────────────────┘              │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│ SECTION 4: GRAPHIQUES TENDANCES                         │
│                                                         │
│ 📈 Évolution heures travaillées (90 jours)              │
│ [Graphique ligne]                                       │
│                                                         │
│ 📊 Distribution ponctualité équipe                      │
│ [Graphique camembert]                                   │
│ • À l'heure: 87.3%                                      │
│ • Retard <15min: 10.2%                                  │
│ • Retard >15min: 2.5%                                   │
└─────────────────────────────────────────────────────────┘

NAVIGATION SIDEBAR (Gauche):
┌──────────────────┐
│ 🏠 Dashboard     │ ← Page actuelle
│ 👥 Employés      │
│ 🏢 Équipes       │
│ 📊 Rapports      │
│ 📅 Calendrier    │
│ ⚙️ Administration│
│ 👤 Mon Profil    │
│ 🚪 Déconnexion   │
└──────────────────┘
```

---

## 5. VISION END-TO-END

### 5.1 Timeline Complète

#### Jour 1: Installation & Setup

```
┌─────────────────────────────────────────────────────────┐
│ ACME CORP démarre Time Manager                          │
│                                                         │
│ 1. IT Admin lance docker-compose up                    │
│ 2. Accès http://localhost:8000                         │
│ 3. Page setup initiale:                                │
│    "Créer le premier administrateur"                    │
│    → Sarah Martin (CEO) créée comme Manager            │
│                                                         │
│ 4. Sarah se connecte, voit dashboard vide:             │
│    "Aucun employé, aucune équipe"                      │
│    → Boutons d'action mis en avant                     │
└─────────────────────────────────────────────────────────┘
```

#### Semaine 1: Onboarding Équipe

```
┌─────────────────────────────────────────────────────────┐
│ Sarah (Manager) ajoute son équipe                       │
│                                                         │
│ Lundi:                                                  │
│ - Créer 12 employés                                    │
│ - Créer équipe "Dev Team Frontend"                     │
│ - Assigner membres                                      │
│ - Envoyer emails de bienvenue                          │
│                                                         │
│ Mardi - Vendredi:                                       │
│ - Employés se connectent première fois                  │
│ - Changement mot de passe obligatoire                   │
│ - Découverte interface                                  │
│ - Premiers clock in/out                                 │
│                                                         │
│ Résultat fin semaine:                                   │
│ - 12 employés actifs                                    │
│ - ~400 clock events enregistrés                        │
│ - Premières stats disponibles                          │
└─────────────────────────────────────────────────────────┘
```

#### Semaine 2-4: Utilisation Normale

```
┌─────────────────────────────────────────────────────────┐
│ Routine établie                                         │
│                                                         │
│ Chaque jour:                                            │
│ - 09:00: Employés arrivent, clock in                   │
│ - Dashboard manager: vue temps réel                     │
│ - 12:30: Certains clock out (pause)                    │
│ - 13:30: Clock in reprise                              │
│ - 18:00: Clock out fin journée                         │
│                                                         │
│ Chaque semaine:                                         │
│ - Vendredi: Sarah génère rapport hebdo                 │
│ - Review KPIs équipe                                    │
│ - Identification points d'amélioration                  │
│                                                         │
│ Chaque mois:                                            │
│ - Rapport complet pour RH/Paie                         │
│ - Analyse tendances                                     │
│ - Ajustements si nécessaire                            │
└─────────────────────────────────────────────────────────┘
```

#### Mois 3+: Expansion

```
┌─────────────────────────────────────────────────────────┐
│ Entreprise grandit                                      │
│                                                         │
│ Nouvelles équipes:                                      │
│ - Dev Team Backend (10 personnes)                      │
│ - DevOps (5 personnes)                                 │
│ - Marketing (8 personnes)                              │
│                                                         │
│ Nouveaux managers:                                      │
│ - 1 manager par équipe                                 │
│ - Droits délégués par équipe                           │
│                                                         │
│ Analytics:                                              │
│ - Comparaison inter-équipes                            │
│ - Benchmarking                                          │
│ - Optimisations processus                              │
│                                                         │
│ Résultat:                                               │
│ - 35 employés actifs                                    │
│ - 4 équipes                                             │
│ - 4 managers                                            │
│ - ~5000 clock events/mois                              │
└─────────────────────────────────────────────────────────┘
```

---

## 6. ARCHITECTURE TECHNIQUE

### 6.1 Stack Technology

```
FRONTEND:
✅ React 18+ avec Vite
✅ TypeScript (strict mode)
✅ Tailwind CSS
✅ Axios pour API calls
✅ React Router v6
✅ React Hook Form + Zod validation
✅ Recharts pour graphiques
✅ Date-fns pour dates
✅ Lucide React pour icons

BACKEND (Existant - Go):
✅ Gin framework
✅ PostgreSQL
✅ JWT auth
✅ GORM

INFRASTRUCTURE:
✅ KrakenD reverse proxy
✅ Docker + Docker Compose
✅ Nginx (production)
```

### 6.2 Routes Backend à Implémenter

```
AUTH:
POST   /api/login          → JWT token
POST   /api/logout         → Invalidate token
POST   /api/refresh-token  → Refresh JWT

USERS:
GET    /api/users          → List all (Manager only)
GET    /api/users/:id      → Get one
POST   /api/users          → Create (Manager only)
PUT    /api/users/:id      → Update
DELETE /api/users/:id      → Delete (Manager only)
GET    /api/me             → Current user profile

TEAMS:
GET    /api/teams          → List all
GET    /api/teams/:id      → Get one
POST   /api/teams          → Create (Manager only)
PUT    /api/teams/:id      → Update (Manager only)
DELETE /api/teams/:id      → Delete (Manager only)
POST   /api/teams/:id/members/:userId  → Add member
DELETE /api/teams/:id/members/:userId  → Remove member

CLOCKS:
POST   /api/clocks         → Clock in/out (authenticated user)
GET    /api/clocks         → Get my clocks
GET    /api/users/:id/clocks → Get user clocks (Manager)

WORKING TIMES:
GET    /api/working-times          → Get my periods
GET    /api/users/:id/working-times → Get user periods (Manager)
GET    /api/working-times/stats     → My statistics
GET    /api/teams/:id/working-times → Team statistics (Manager)

REPORTS:
GET    /api/reports/kpis           → Global KPIs (Manager)
GET    /api/reports/team/:id       → Team report (Manager)
GET    /api/reports/user/:id       → User report
POST   /api/reports/generate       → Generate PDF report (Manager)
```

### 6.3 Configuration Nginx

```nginx
server {
    listen 80;
    server_name localhost;

    # Frontend - React SPA
    location / {
        proxy_pass http://frontend:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }

    # Backend API - via KrakenD
    location /api {
        proxy_pass http://reverse-proxy:8000;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### 6.4 Modèles de Données

```typescript
// User
interface User {
  id: number;
  email: string;
  firstName: string;
  lastName: string;
  phoneNumber?: string;
  role: 'employee' | 'manager';
  teamId?: number;
  createdAt: string;
  updatedAt: string;
}

// Team
interface Team {
  id: number;
  name: string;
  description?: string;
  managerId: number;
  memberIds: number[];
  createdAt: string;
  updatedAt: string;
}

// Clock
interface Clock {
  id: number;
  userId: number;
  timestamp: string;
  status: 'clock_in' | 'clock_out';
  createdAt: string;
}

// Working Time (calculé)
interface WorkingTime {
  id: number;
  userId: number;
  startTime: string;
  endTime: string;
  duration: number; // en secondes
  date: string;
}

// Statistics
interface UserStatistics {
  userId: number;
  today: number;
  thisWeek: number;
  thisMonth: number;
  averageWeekly: number;
  punctualityRate: number;
  totalDays: number;
  onTimeDays: number;
  lateDays: number;
}

// KPI
interface TeamKPI {
  teamId: number;
  presenceRate: number;
  punctualityRate: number;
  averageHoursPerEmployee: number;
  overtimeHours: number;
  alerts: Alert[];
}
```

---

## 7. DESIGN SYSTEM

### 7.1 Palette de Couleurs

```css
/* Primary - Blue */
--primary-50: #eff6ff;
--primary-500: #3b82f6;
--primary-600: #2563eb;
--primary-700: #1d4ed8;

/* Success - Green */
--success-50: #f0fdf4;
--success-500: #22c55e;
--success-600: #16a34a;

/* Warning - Orange */
--warning-50: #fff7ed;
--warning-500: #f97316;
--warning-600: #ea580c;

/* Danger - Red */
--danger-50: #fef2f2;
--danger-500: #ef4444;
--danger-600: #dc2626;

/* Neutral - Gray */
--gray-50: #f9fafb;
--gray-100: #f3f4f6;
--gray-200: #e5e7eb;
--gray-500: #6b7280;
--gray-700: #374151;
--gray-900: #111827;
```

### 7.2 Typography

```css
/* Headings */
h1: text-3xl font-bold (30px)
h2: text-2xl font-semibold (24px)
h3: text-xl font-semibold (20px)
h4: text-lg font-medium (18px)

/* Body */
body: text-base (16px)
small: text-sm (14px)
tiny: text-xs (12px)
```

### 7.3 Spacing

```css
/* Spacing scale */
xs: 0.25rem (4px)
sm: 0.5rem (8px)
md: 1rem (16px)
lg: 1.5rem (24px)
xl: 2rem (32px)
2xl: 3rem (48px)
```

### 7.4 Composants Réutilisables

```typescript
// Button variants
<Button variant="primary" size="lg">Clock In</Button>
<Button variant="danger" size="md">Clock Out</Button>
<Button variant="secondary" size="sm">Cancel</Button>

// Card
<Card>
  <CardHeader>
    <CardTitle>Statistics</CardTitle>
  </CardHeader>
  <CardContent>...</CardContent>
</Card>

// Badge
<Badge variant="success">En service</Badge>
<Badge variant="warning">Pause</Badge>
<Badge variant="default">Hors service</Badge>

// Progress
<Progress value={75} max={100} />

// Input
<Input
  type="email"
  placeholder="Email"
  error="Invalid email"
/>
```

---

## 8. TESTS & QUALITÉ

### 8.1 Stratégie de Tests

```
FRONTEND:
✅ Unit tests: Components (Jest + RTL)
✅ Integration tests: User flows (RTL)
✅ E2E tests: Critical paths (Playwright)
✅ Coverage target: > 80%

BACKEND:
✅ Unit tests: Handlers, services
✅ Integration tests: API endpoints
✅ Coverage target: > 85%
```

### 8.2 Scénarios de Tests Critiques

```
1. Authentication Flow
   - Login success
   - Login failure
   - Token refresh
   - Logout

2. Clock In/Out Flow
   - Clock in success
   - Clock out success
   - Multiple sessions same day
   - Timer updates in real-time

3. Manager Operations
   - Create employee
   - Create team
   - View team statistics
   - Generate reports

4. Data Validation
   - Form validation
   - API error handling
   - Network failures
   - Edge cases
```

---

## 9. DÉPLOIEMENT & PRODUCTION

### 9.1 Configuration Production

```yaml
# docker-compose.prod.yml
version: '3.8'
services:
  frontend:
    build:
      context: ./front
      dockerfile: Dockerfile.prod
    environment:
      - NODE_ENV=production
      - REACT_APP_API_URL=/api

  backend:
    build:
      context: ./back
      dockerfile: Dockerfile
    environment:
      - GIN_MODE=release
      - DATABASE_URL=${DATABASE_URL}
      - JWT_SECRET=${JWT_SECRET}

  nginx:
    image: nginx:alpine
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
    ports:
      - "80:80"
      - "443:443"
```

### 9.2 Variables d'Environnement

```bash
# Frontend (.env.production)
REACT_APP_API_URL=/api
REACT_APP_ENV=production

# Backend (.env)
DATABASE_URL=postgres://user:pass@db:5432/timemanager
JWT_SECRET=your-secure-secret-key
JWT_TTL=24h
APP_PORT=8080
GIN_MODE=release
```

---

## 10. ROADMAP & ÉVOLUTIONS FUTURES

### Phase 1 (MVP - 4 semaines)
- ✅ Authentication
- ✅ Clock In/Out
- ✅ Dashboards Employee & Manager
- ✅ User & Team management
- ✅ KPIs basiques

### Phase 2 (6 semaines)
- 📅 Calendrier avancé
- 📊 Rapports PDF
- 📧 Notifications email
- 📱 Version mobile responsive
- 🌍 Géolocalisation

### Phase 3 (8 semaines)
- 📈 Analytics avancés
- 🔔 Notifications push
- 📸 Photo verification
- 🌐 Multi-langue
- 🎨 Thèmes personnalisables

### Phase 4 (Future)
- 🤖 IA - Détection patterns
- 📱 Applications natives (iOS/Android)
- 🔗 Intégrations externes (Slack, Teams)
- 🎯 Gamification
- 📊 Prédictions IA

---

**Document maintenu par**: Équipe Time Manager
**Dernière mise à jour**: 14 Octobre 2025
**Version**: 1.0
