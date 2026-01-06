# Registre des Traitements - Time Manager

> Document requis par l'Article 30 du RGPD

## Informations Responsable de Traitement

| Champ | Valeur |
|-------|--------|
| **Application** | Time Manager |
| **Version** | 1.0 |
| **Date de mise à jour** | À définir par l'organisation cliente |
| **Responsable de traitement** | [Organisation cliente] |
| **DPO** | [À désigner par l'organisation cliente] |
| **Contact** | [Email/téléphone DPO] |

---

## Traitement 1 : Gestion des Pointages

### Informations Générales

| Champ | Valeur |
|-------|--------|
| **Identifiant** | TRT-001 |
| **Nom** | Gestion des temps et pointages |
| **Finalité** | Suivi du temps de travail des employés |
| **Base légale** | Art. 6.1.b RGPD (exécution contrat de travail) |
| **Responsable** | Service RH |

### Données Collectées

| Donnée | Catégorie | Obligatoire | Justification |
|--------|-----------|:-----------:|---------------|
| Heure d'entrée | Temps de travail | Oui | Calcul heures travaillées |
| Heure de sortie | Temps de travail | Oui | Calcul heures travaillées |
| Durée travaillée | Temps de travail | Oui | Reporting et paie |
| Corrections manuelles | Temps de travail | Non | Ajustements validés |
| Commentaires | Temps de travail | Non | Contexte des corrections |
| Adresse IP | Métadonnée technique | Oui | Audit et sécurité |

### Destinataires

| Destinataire | Accès | Finalité |
|--------------|-------|----------|
| Employé concerné | Ses propres données | Consultation personnelle |
| Manager direct | Équipe managée | Validation et suivi |
| Service RH | Organisation entière | Gestion administrative |
| Service Paie | Données agrégées | Calcul rémunération |
| Administrateur | Organisation entière | Administration système |

### Conservation et Sécurité

| Aspect | Valeur |
|--------|--------|
| **Durée de conservation** | 6 ans (obligation légale Code du travail) |
| **Anonymisation** | Après 6 ans |
| **Chiffrement** | En transit (TLS 1.3) et au repos (AES-256) |
| **Accès** | Authentification JWT, contrôle par rôle |

---

## Traitement 2 : Gestion des Absences

### Informations Générales

| Champ | Valeur |
|-------|--------|
| **Identifiant** | TRT-002 |
| **Nom** | Gestion des demandes d'absence |
| **Finalité** | Planification et suivi des congés et absences |
| **Base légale** | Art. 6.1.b RGPD (exécution contrat de travail) |
| **Responsable** | Service RH |

### Données Collectées

| Donnée | Catégorie | Obligatoire | Justification |
|--------|-----------|:-----------:|---------------|
| Type d'absence | Congés | Oui | Catégorisation (CP, RTT, maladie) |
| Date de début | Congés | Oui | Planification |
| Date de fin | Congés | Oui | Planification |
| Motif | Congés | Non | Contexte (hors maladie) |
| Statut demande | Congés | Oui | Workflow validation |
| Soldes congés | Congés | Oui | Droits acquis |

### Données Sensibles (Article 9)

| Donnée | Traitement |
|--------|------------|
| Arrêts maladie | Seule la catégorie "maladie" est stockée, pas le diagnostic |
| Motifs médicaux | **Non collectés** - respect vie privée |

### Destinataires

| Destinataire | Accès | Finalité |
|--------------|-------|----------|
| Employé concerné | Ses demandes | Consultation et demandes |
| Manager direct | Équipe managée | Validation absences |
| Service RH | Organisation entière | Gestion administrative |
| Collègues (planification) | Dates d'absence uniquement | Visibilité équipe |

### Conservation et Sécurité

| Aspect | Valeur |
|--------|--------|
| **Durée de conservation** | 6 ans (obligation légale) |
| **Anonymisation** | Après 6 ans |
| **Chiffrement** | En transit (TLS 1.3) et au repos (AES-256) |

---

## Traitement 3 : Authentification et Sessions

### Informations Générales

| Champ | Valeur |
|-------|--------|
| **Identifiant** | TRT-003 |
| **Nom** | Authentification et gestion des accès |
| **Finalité** | Sécurisation de l'accès à l'application |
| **Base légale** | Art. 6.1.f RGPD (intérêt légitime - sécurité) |
| **Responsable** | Service IT / Sécurité |

### Données Collectées

| Donnée | Catégorie | Obligatoire | Justification |
|--------|-----------|:-----------:|---------------|
| Email | Identifiant | Oui | Identification unique |
| Mot de passe (hashé) | Sécurité | Oui | Authentification |
| Refresh tokens | Session | Oui | Maintien connexion |
| Adresse IP | Métadonnée | Oui | Détection anomalies |
| User-Agent | Métadonnée | Oui | Identification appareil |
| Tentatives échouées | Sécurité | Oui | Protection brute force |

### Conservation et Sécurité

| Aspect | Valeur |
|--------|--------|
| **Refresh tokens** | 30 jours max, révocables |
| **Sessions actives** | Max 5 simultanées par utilisateur |
| **Historique connexions** | 90 jours |
| **IP et User-Agent** | Anonymisés après 6 mois |
| **Mots de passe** | Hashés (Argon2id), jamais stockés en clair |

---

## Traitement 4 : Journalisation et Audit

### Informations Générales

| Champ | Valeur |
|-------|--------|
| **Identifiant** | TRT-004 |
| **Nom** | Audit logs et traçabilité |
| **Finalité** | Sécurité, conformité, investigation incidents |
| **Base légale** | Art. 6.1.f RGPD (intérêt légitime - sécurité) |
| **Responsable** | Service IT / Sécurité |

### Données Collectées

| Donnée | Catégorie | Obligatoire | Justification |
|--------|-----------|:-----------:|---------------|
| Acteur (user_id) | Traçabilité | Oui | Qui a fait l'action |
| Action effectuée | Traçabilité | Oui | Quoi a été fait |
| Ressource concernée | Traçabilité | Oui | Sur quoi |
| Données avant/après | Traçabilité | Non | Historique modifications |
| Adresse IP | Métadonnée | Oui | Localisation source |
| User-Agent | Métadonnée | Oui | Appareil utilisé |
| Timestamp | Traçabilité | Oui | Quand |

### Conservation et Sécurité

| Phase | Durée | État des données |
|-------|-------|------------------|
| Complète | 0-6 mois | Toutes données (IP, User-Agent) |
| Anonymisée | 6-24 mois | IP masquée, User-Agent supprimé |
| Purgée | > 24 mois | Suppression définitive |

### Accès aux Logs

| Rôle | Accès |
|------|-------|
| Administrateur | Logs de son organisation |
| Super Administrateur | Tous les logs |
| Employé | **Aucun accès** aux logs |

---

## Traitement 5 : Gestion des Utilisateurs

### Informations Générales

| Champ | Valeur |
|-------|--------|
| **Identifiant** | TRT-005 |
| **Nom** | Gestion des comptes utilisateurs |
| **Finalité** | Administration des accès et profils |
| **Base légale** | Art. 6.1.b RGPD (exécution contrat de travail) |
| **Responsable** | Service RH |

### Données Collectées

| Donnée | Catégorie | Obligatoire | Justification |
|--------|-----------|:-----------:|---------------|
| Prénom | Identité | Oui | Identification |
| Nom | Identité | Oui | Identification |
| Email professionnel | Contact | Oui | Communication, login |
| Rôle | Accès | Oui | Gestion permissions |
| Équipe | Organisation | Non | Rattachement hiérarchique |
| Date création compte | Administrative | Oui | Traçabilité |

### Conservation et Sécurité

| Aspect | Valeur |
|--------|--------|
| **Compte actif** | Durée du contrat de travail |
| **Compte désactivé** | 30 jours de grâce puis anonymisation |
| **Données anonymisées** | Conservation minimale pour intégrité données |

---

## Droits des Personnes Concernées

### Exercice des Droits (Articles 15-22 RGPD)

| Droit | Implémentation | Délai de réponse |
|-------|----------------|------------------|
| **Accès (Art. 15)** | `GET /api/v1/users/me/data-export` | 30 jours max |
| **Rectification (Art. 16)** | Via interface utilisateur | Immédiat |
| **Effacement (Art. 17)** | `POST /api/v1/users/me/deletion-request` | 30 jours (grâce) |
| **Portabilité (Art. 20)** | `GET /api/v1/users/me/data-export?format=csv` | 30 jours max |
| **Opposition (Art. 21)** | Contact DPO | 30 jours max |
| **Limitation (Art. 18)** | Contact DPO | 30 jours max |

### Procédure d'Exercice des Droits

1. **Demande** : Via l'interface utilisateur ou email au DPO
2. **Vérification identité** : Confirmation par mot de passe ou double authentification
3. **Traitement** : Exécution de la demande dans le délai légal
4. **Confirmation** : Email de confirmation à l'utilisateur
5. **Traçabilité** : Log de l'exercice du droit dans audit_logs

---

## Mesures de Sécurité Techniques et Organisationnelles

### Mesures Techniques

| Mesure | Implémentation |
|--------|----------------|
| Chiffrement en transit | TLS 1.3 obligatoire |
| Chiffrement au repos | AES-256 pour la base de données |
| Hachage mots de passe | Argon2id avec sel unique |
| Authentification | JWT avec tokens de courte durée (15 min) |
| Autorisation | Contrôle d'accès basé sur les rôles (RBAC) |
| Rate limiting | Protection contre les attaques brute force |
| Audit trail | Journalisation complète des actions sensibles |
| Isolation tenant | Séparation stricte des données par organisation |

### Mesures Organisationnelles

| Mesure | Description |
|--------|-------------|
| Sensibilisation | Formation RGPD pour les administrateurs |
| Politique d'accès | Principe du moindre privilège |
| Gestion incidents | Procédure de notification CNIL sous 72h |
| Revue périodique | Audit annuel des accès et traitements |
| Documentation | Maintien à jour du présent registre |

---

## Transferts de Données

### Transferts Hors UE

| Élément | Valeur |
|---------|--------|
| Transferts hors UE | **Aucun** (hébergement UE uniquement) |
| Sous-traitants | À définir par l'organisation cliente |
| Clauses contractuelles | N/A si hébergement UE |

---

## Historique des Modifications

| Date | Version | Modification | Auteur |
|------|---------|--------------|--------|
| [À compléter] | 1.0 | Création initiale | [Équipe projet] |

---

## Contacts

| Rôle | Contact |
|------|---------|
| DPO | [À désigner] |
| Support technique | [Email support] |
| Signalement incident | [Email sécurité] |

---

> **Note** : Ce registre doit être mis à jour à chaque modification des traitements de données personnelles conformément à l'Article 30 du RGPD.
