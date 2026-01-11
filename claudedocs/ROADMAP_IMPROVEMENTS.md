# Roadmap des Améliorations

> Feuille de route issue de l'analyse de code du 11/01/2026

---

## Haute Priorité

- [x] **Fix lifetime warning** - Corriger l'avertissement de lifetime dans `repository_tests.rs:27`
- [x] **Complete team joined_at** - Implémenter la récupération réelle de `joined_at` dans `team_service.rs`
- [x] **Add schedule unassignment** - Ajouter l'endpoint et le frontend pour désassigner un planning

---

## Priorité Moyenne

- [x] **Structured frontend logging** - Remplacer `console.error` par un système de logging centralisé
- [~] **Redis rate limiting** - Décliné (instance unique, pas de bénéfice)
- [x] **Async database operations** - Migrer vers `diesel-async` pour les opérations non-bloquantes

---

## Basse Priorité

- [x] **E2E tests setup** - Configurer Playwright pour les tests end-to-end
- [x] **Bundle analysis** - Analyser et optimiser le bundle frontend
- [ ] **OpenAPI documentation** - Générer la documentation API automatique (reporté - API non finalisée)

---

## Historique des Commits

| Date | Tâche | Commit |
|------|-------|--------|
| 11/01/2026 | Fix lifetime warning | `558c235` |
| 11/01/2026 | Complete team joined_at | `d0ed2b9` |
| 11/01/2026 | Add schedule unassignment | `c35a438` |
| 11/01/2026 | Structured frontend logging | `0f2799d` |
| 11/01/2026 | Async database operations | `373e44f` |
| 11/01/2026 | E2E tests setup | `a690269` |
| 11/01/2026 | Bundle analysis | `ccb2d26` |

