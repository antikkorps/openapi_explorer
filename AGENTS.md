# OpenAPI Field Explorer - Spécifications du Projet

## Description
Interface TUI (Terminal User Interface) en Rust avec ratatui qui parse une spécification OpenAPI et permet de rechercher/analyser l'usage des champs de base de données à travers tous les endpoints.

## Stack Technique
- **ratatui**: Interface TUI
- **crossterm**: Gestion du terminal
- **serde + serde_json**: Parsing JSON
- **tokio**: Async et watch de fichiers
- **fuzzy-matcher**: Recherche floue
- **anyhow**: Gestion d'erreurs

## Interface TUI

### Layout Principal
```
┌─ OpenAPI Field Explorer ─────────────────────────────────────────┐
│ Search: USER_ID                                                  │
├─────────────┬─────────────────────┬───────────────────────────────┤
│ Fields      │ Field: USER_ID      │ Endpoints (3)                 │
│ ► USER_ID   │ Type: integer       │ GET /apiHc0/user              │
│   USER_NOM  │ Desc: ID utilisateur│ POST /apiHc0/user             │
│   PAT_ID    │                     │ PUT /apiHc0/user/{id}         │
│   ART_CODE  │ Used in 2 schemas:  │                               │
│   ...       │ • User              │ Impact: 3 endpoints           │
│             │ • Patient (FK)      │ Critical: 1 POST, 1 PUT      │
└─────────────┴─────────────────────┴───────────────────────────────┘
│ F1:Help  F2:Reload  F3:Graph  q:Quit                            │
```

### Panneaux
- **Barre de recherche**: En haut, recherche en temps réel
- **Panneau gauche**: Liste des champs/schémas (navigable)
- **Panneau central**: Détails du champ sélectionné
- **Panneau droit**: Liste des endpoints utilisant ce champ
- **Barre de statut**: Stats et raccourcis en bas

### Fonctionnalités Interactives
- Navigation au clavier (↑↓ pour naviguer, Tab pour changer de panneau)
- Recherche en temps réel (filtrage dynamique)
- Graphique ASCII des relations entre schémas
- Mode "impact analysis" avec visualisation
- Pop-ups pour les détails complets d'un endpoint

### Vues Multiples
1. **Vue Fields**: Navigation par champs de base
2. **Vue Schemas**: Navigation par schémas OpenAPI
3. **Vue Endpoints**: Navigation par endpoints
4. **Vue Graph**: Visualisation des relations
5. **Vue Stats**: Dashboard avec métriques

## Interactions Clavier
- `q/Ctrl+C`: Quitter
- `Tab`: Changer de panneau
- `/`: Mode recherche
- `Enter`: Voir détails/naviguer
- `Esc`: Retour
- `1-5`: Changer de vue (Fields/Schemas/Endpoints/Graph/Stats)
- `r`: Recharger le fichier OpenAPI
- `h`: Aide

## Structure du Projet
```
field-explorer/
├── src/
│   ├── main.rs           # Entry point + event loop
│   ├── app.rs            # Application state
│   ├── ui/
│   │   ├── mod.rs        # UI modules
│   │   ├── layout.rs     # Layout principal
│   │   ├── fields.rs     # Vue fields
│   │   ├── schemas.rs    # Vue schemas
│   │   ├── endpoints.rs  # Vue endpoints
│   │   └── graph.rs      # Vue graphique
│   ├── parser.rs         # OpenAPI parsing
│   ├── indexer.rs        # Field indexing
│   └── events.rs         # Gestion événements
├── examples/
└── Cargo.toml
```

## Commandes Utiles
```bash
# Lancer le projet
cargo run

# Lancer avec un fichier OpenAPI spécifique
cargo run -- examples/petstore.json

# Linter
cargo clippy

# Formatter
cargo fmt

# Tests
cargo test

# Build release
cargo build --release
```

## Objectif
Interface TUI interactive et intuitive pour explorer visuellement les relations entre champs de base de données et endpoints d'une API OpenAPI.