# Gravity Map — Development Roadmap

## Overview

Gravity Map has three modes, each serving a different purpose:

- **Mode 1: Gravity** — File system structure visualiser. Scans a project folder and displays files as nodes in a radial layout with folder hierarchy tethers.
- **Mode 2: Neural Network** — Dependency graph visualiser. Same nodes as Mode 1, but tethered by actual import/dependency relationships with force-directed layout.
- **Mode 3: Blueprint** — Freeform idea whiteboard. Independent canvas for brainstorming with manual nodes, tethers, categories, and save/load.

All three modes share a common infrastructure: infinite pannable canvas, zoom, background grid, and interactive bezier tether lines.

---

## Phase 0: Foundation (DONE)

What's already built and working:

- [x] Modular codebase (`app.rs`, `nodes.rs`, `utils.rs`, `canvas.rs`, `modes/`)
- [x] Mode 1: Project scanning with recursive sub-sun directories
- [x] Mode 1: Radial layout, interactive tether hover labels, depth-based colours
- [x] Mode 1: Collapse/expand all, sun off-screen indicator, draggable nodes
- [x] Mode 3: Add/delete nodes, manual tether linking, right-click delete with confirmation
- [x] Mode 3: Node categories with colour-coded borders (Idea, Task, Bug, Feature, Research, Done)
- [x] Mode 3: Save/Load blueprints as JSON (metadata, positions, tethers, categories)
- [x] Shared: Infinite canvas with pan, zoom, background grid
- [x] Shared: Interactive bezier tethers with hover proximity labels
- [x] Mode 2: Empty template placeholder

---

## Phase 1: Shared Infrastructure

Work that benefits all three modes. Do this first to avoid duplicating effort.

| # | Feature | Modes | Priority |
|---|---------|-------|----------|
| 1.1 | **Middle mouse pan** — reserve left-drag for window interaction, middle-drag for canvas pan | All | High |
| 1.2 | **Zoom to Fit button** — auto-zoom to show all nodes on screen | All | High |
| 1.3 | **Search/Filter** — find a node by name, highlight and pan to it | All | High |
| 1.4 | **Minimap** — small overview in the corner showing current viewport position | All | Medium |
| 1.5 | **Keyboard shortcuts** — Ctrl+S save, Ctrl+Z undo, Delete key for nodes | Mode 3 first, extend to others | Medium |
| 1.6 | **Undo/Redo system** — command pattern for reversible actions | Mode 3 first, extend to others | Medium |

---

## Phase 2: Mode 1 — Gravity (Core Completion)

Make Mode 1 a fully functional project explorer.

| # | Feature | Priority |
|---|---------|----------|
| 2.1 | **Multi-language support** — scan .rs, .js, .ts, .cs, .go, .py (configurable) | High |
| 2.2 | **Custom ignore patterns** — user-configurable folder/file exclusions | High |
| 2.3 | **Project root indicator** — show scanned folder path in the toolbar | High |
| 2.4 | **Re-scan / Refresh** — update the map without clearing and re-scanning | High |
| 2.5 | **Save/Load project layout** — persist node positions to JSON | High |
| 2.6 | **Node right-click delete** — parity with Mode 3 | Medium |
| 2.7 | **Open in editor** — click a node to open the file in the system editor/IDE | Medium |
| 2.8 | **Collapse/Expand sub-trees** — collapse a sub-sun and all children into one node | Medium |
| 2.9 | **Node statistics** — show line count, function count, class count per file | Medium |
| 2.10 | **File size indicators** — visually scale nodes by line count | Low |
| 2.11 | **Syntax-aware preview** — show function/class signatures in node header | Low |

---

## Phase 3: Mode 2 — Neural Network (Build from scratch)

The biggest chunk of new work. Depends on Mode 1's scan data.

### 3A: Parser Foundation

| # | Feature | Priority |
|---|---------|----------|
| 3A.1 | **Create `src/parser.rs`** — tree-sitter import extraction module | Critical |
| 3A.2 | **Python import parsing** — `import x`, `from x import y` | Critical |
| 3A.3 | **Multi-language grammar support** — C#, Rust (designed extensibly) | High |
| 3A.4 | **Add `nn_tethers`, `nn_cam_offset`, `nn_zoom` to GravityApp** | Critical |

### 3B: Core Rendering

| # | Feature | Priority |
|---|---------|----------|
| 3B.1 | **Basic Mode 2 rendering** — same nodes as Mode 1, import-based tethers | Critical |
| 3B.2 | **Directional tethers (arrows)** — arrow heads showing import direction | Critical |
| 3B.3 | **Force-directed physics layout** — nodes arrange by connection density | Critical |
| 3B.4 | **Separate camera state** — independent pan/zoom from Mode 1 | Critical |
| 3B.5 | **Refresh button** — recalculate imports without re-scanning disk | High |

### 3C: Analysis Features

| # | Feature | Priority |
|---|---------|----------|
| 3C.1 | **Dead code detection** — highlight files never imported (ghost state) | High |
| 3C.2 | **Centrality & scale** — nodes with many dependents glow brighter/larger | High |
| 3C.3 | **Circular dependency detection** — highlight import cycles in red | High |
| 3C.4 | **Transitive dependency count** — badge per node showing total transitive deps | Medium |
| 3C.5 | **Hover to trace path** — highlight full import chain from main.py to hovered node | Medium |
| 3C.6 | **Filtered view** — hide nodes below a connection threshold | Medium |
| 3C.7 | **Cluster/module boundaries** — auto-colour strongly connected components | Medium |

### 3D: Polish

| # | Feature | Priority |
|---|---------|----------|
| 3D.1 | **Pulse animation** — glowing dot moving along tethers showing data flow | Low |
| 3D.2 | **God file detector** — flag high fan-in AND high fan-out files | Low |
| 3D.3 | **Phantom dependency detection** — dashed tethers for imported-but-unused | Low |

---

## Phase 4: Mode 3 — Blueprint (Core Completion)

Make Mode 3 a proper planning tool.

| # | Feature | Priority |
|---|---------|----------|
| 4.1 | **Tether labels** — annotate what the relationship means | High |
| 4.2 | **Tether direction (arrows)** — show "A leads to B" vs "B leads to A" | High |
| 4.3 | **Drag to link** — drag from one node edge to another instead of clicking Connect buttons | High |
| 4.4 | **Multi-select** — select and move/delete multiple nodes at once | High |
| 4.5 | **Node duplication** — clone a node | Medium |
| 4.6 | **Auto-save** — periodic auto-save to prevent lost work | Medium |
| 4.7 | **Unsaved changes indicator** — visual cue in toolbar | Medium |
| 4.8 | **Node collapse state in save** — persist expanded/collapsed in JSON | Medium |
| 4.9 | **Grid snapping** — optional snap-to-grid for neat alignment | Medium |
| 4.10 | **Node grouping / colour regions** — draw coloured rectangles behind clusters | Medium |
| 4.11 | **Node resize** — allow resizing nodes based on content | Low |

---

## Phase 5: Advanced Features (All Modes)

Features that make Gravity Map stand out from existing tools. Pick and choose based on user feedback.

### Mode 1 Advanced

| # | Feature |
|---|---------|
| 5.1 | Complexity heatmap — colour by cyclomatic complexity |
| 5.2 | Git status overlay — colour by modified/untracked/staged |
| 5.3 | Change frequency overlay — colour by git log frequency |
| 5.4 | File age gradient — dim old untouched files |
| 5.5 | TODO/FIXME scanner — badge count per node |
| 5.6 | File watcher — live update on filesystem changes |
| 5.7 | Orphan detection — highlight files nothing imports |

### Mode 2 Advanced

| # | Feature |
|---|---------|
| 5.8 | Live refactor preview — simulate moving code and show tether changes |
| 5.9 | Import cost estimation — transitive weight number on tethers |
| 5.10 | Dependency layering enforcer — define and enforce architecture rules |
| 5.11 | Time-lapse replay — scrub through git history watching the graph grow |
| 5.12 | Interface contracts on tethers — show which functions cross the boundary |
| 5.13 | Fragility score — (dependents x change frequency) per node |
| 5.14 | Cross-language bridge highlighting — FFI/API boundaries as distinct tethers |
| 5.15 | Dependency diff between branches — visual PR impact |
| 5.16 | Automatic module boundary suggestion — recommend package splits |
| 5.17 | Impact ripple animation — visualise change cascade depth |
| 5.18 | Dependency age mismatch — flag new code depending on ancient code |
| 5.19 | Test proximity mapping — faint tethers between tests and production files |
| 5.20 | Runtime vs static view toggle — compare imports vs actual execution paths |

### Mode 3 Advanced

| # | Feature |
|---|---------|
| 5.21 | Time decay indicators — fade stale nodes |
| 5.22 | Dependency chains & critical path — highlight the longest blocker chain |
| 5.23 | Heat map / activity overlay — colour by edit recency |
| 5.24 | Node weight / effort sizing — S/M/L visual scaling |
| 5.25 | Snapshot history — scrub through plan evolution over time |
| 5.26 | Auto-cluster / gravity grouping — pull connected nodes together |
| 5.27 | Focus mode — dim everything except selected node's connections |
| 5.28 | Embedded checklists inside nodes — with progress indicator |
| 5.29 | Export to markdown / task list — structured document generation |
| 5.30 | Conflict detection — flag multiple bugs targeting the same feature |

---

## Phase 6: Cross-Mode Integration

Features that connect the modes together.

| # | Feature |
|---|---------|
| 6.1 | **Blueprint → Code linking** — link a Mode 3 idea node to a Mode 1 file node, creating a traceability chain from plan to implementation |
| 6.2 | **Neural Network → Blueprint overlay** — show which blueprint ideas have been implemented (dependency exists) vs which are still just plans |
| 6.3 | **Unified search** — search across all three modes from a single search bar |
| 6.4 | **Shared node annotations** — add notes to a Mode 1 node that are visible in Mode 2 |
| 6.5 | **Export combined report** — generate a single document covering project structure, dependency analysis, and planning status |

---

## Suggested Development Order

```
Phase 0  ██████████████████████████████  DONE
Phase 1  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  Shared infra (middle mouse, zoom-to-fit, search)
Phase 2  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  Mode 1 core completion
Phase 3A ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  Parser foundation
Phase 3B ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  Mode 2 core rendering
Phase 4  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  Mode 3 core completion
Phase 3C ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  Mode 2 analysis features
Phase 5  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  Advanced features (ongoing)
Phase 6  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  Cross-mode integration
```

Phase 1 first because it prevents duplicated work. Phase 2 before 3 because Mode 2 depends on Mode 1's scan data. Phase 4 can run in parallel with Phase 3 since Mode 3 is independent.
