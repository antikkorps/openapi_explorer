# TODO - OpenAPI Field Explorer Development

## Phase 1: Project Setup & Dependencies ✅
- [x] Initialize Rust project with Cargo
- [x] Create basic project structure
- [x] Add dependencies to Cargo.toml
- [x] Verify dependencies installation
- [x] Test basic compilation

## Phase 2: Core Data Structures ✅
- [x] Define OpenAPI schema structures
- [x] Create field indexing data models
- [x] Implement endpoint representation
- [x] Add search result structures

## Phase 3: OpenAPI Parser ✅
- [x] Implement JSON parsing for OpenAPI specs
- [x] Extract schemas and their fields
- [x] Parse endpoints and their parameters
- [x] Build field-to-endpoint relationships
- [x] Add error handling for invalid specs

## Phase 4: Field Indexer ✅
- [x] Create field extraction from schemas
- [x] Build reverse index (field -> schemas)
- [x] Map fields to endpoints usage
- [x] Implement relationship tracking
- [x] Add field metadata (type, description, etc.)

## Phase 5: Basic TUI Framework ✅
- [x] Initialize ratatui application
- [x] Create main layout structure
- [x] Implement basic event handling
- [x] Add keyboard input handling
- [x] Create application state management

## Phase 6: UI Components ✅
- [x] Implement search bar component
- [x] Create fields list panel
- [x] Build field details panel
- [x] Add endpoints list panel
- [x] Create status bar

## Phase 7: Views Implementation 🚧
- [x] Fields View (primary navigation) - Structure de base créée
- [x] Schemas View (schema-centric) - Structure de base créée
- [x] Endpoints View (endpoint-centric) - Structure de base créée
- [x] Graph View (relationship visualization) - Structure de base créée
- [x] Stats View (dashboard) - Structure de base créée
- [ ] Améliorer la navigation et la sélection dans les vues

## Phase 8: Search & Navigation 🚧
- [ ] Implement fuzzy search functionality
- [x] Add real-time filtering - Filtrage basique fonctionnel
- [x] Create panel switching (Tab) - Implémenté
- [x] Add keyboard navigation - Navigation de base en place
- [x] Implement view switching (1-5 keys) - Fonctionnel
- [ ] Ajouter la sélection de champs avec Enter

## Phase 9: Advanced Features 🚧
- [x] ASCII graph visualization - Structure créée
- [x] Impact analysis mode - Logique de base présente
- [x] Endpoint detail pop-ups - Structure créée
- [ ] File watching for auto-reload
- [x] Help system - Popup d'aide fonctionnel

## Phase 10: Polish & Testing
- [ ] Add comprehensive error handling
- [ ] Implement proper logging
- [ ] Write unit tests
- [ ] Add integration tests
- [ ] Performance optimization
- [ ] Documentation

## Phase 11: Build & Release
- [ ] Create release build configuration
- [ ] Add installation instructions
- [ ] Create user documentation
- [ ] Test with various OpenAPI specs
- [ ] Prepare for distribution

## Current Priority Tasks
1. **Add field selection and navigation logic** - Permettre la sélection de champs avec Enter
2. **Implement fuzzy search functionality** - Améliorer la recherche avec fuzzy matching
3. **Complete Fields view interaction** - Navigation complète dans la vue principale
4. **Fix parameter parsing issue** - ✅ Résolu (champ `in_`)

## Bugs Fixes Récemts ✅
- Fix parameter parsing issue (champ `in_` manquant) - Ajout de `#[serde(rename = "in")]`

## Notes
- Focus on incremental development
- Test each component before moving to next
- Keep the UI responsive during parsing
- Handle large OpenAPI specs efficiently
- Consider memory usage for field indexing