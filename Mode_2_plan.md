# Mode 2: Neural Network - Feature Plan

## Concept

Mode 2 is the "Active Logic" layer. If Mode 1 (Gravity) is the skeleton (the folder tree), Mode 2 is the nervous system (the actual code execution paths). We stop caring about where a file is stored on the filesystem and only care about dependencies — who imports whom.

Shared nodes from Mode 1's scan, but with import-based tethers replacing folder-based tethers, and force-directed physics replacing radial layout.

---

## Core Features Still Needed

1. **Static Import Parser** — use tree-sitter-python to parse `import module` and `from module import function` statements, populating an `nn_tethers: Vec<(usize, usize)>` list
2. **Directional Tethers (Arrows)** — tethers show direction with arrow heads: if main.py imports auth.py, the arrow points from main.py toward auth.py
3. **Force-Directed Layout** — nodes find their own positions based on connection count, replacing the radial layout used in Mode 1
4. **Centrality & Scale** — nodes with many incoming connections (popular libraries) scale up or glow brighter, instantly showing bottleneck files
5. **Dead Code Detection** — files never imported by another file (and not main.py) are highlighted in dim grey/"ghost" state
6. **Separate Camera State** — Mode 2 needs its own `nn_cam_offset`/`nn_zoom` so the force-directed layout doesn't fight with Mode 1's radial positions
7. **Circular Dependency Detection** — highlight import cycles (A imports B, B imports A) with a distinct colour, since they're a real code smell
8. **Hover to Trace Path** — hover a node and highlight the full import chain from main.py to that node, answering "how does execution reach this file?"
9. **Transitive Dependency Count** — badge showing "this file transitively depends on N other files", reveals hidden complexity
10. **Multi-Language Support** — parser module designed for multiple grammars from the start (tree-sitter-python and tree-sitter-c-sharp are already in Cargo.toml)
11. **Filtered View** — hide nodes below a certain connection threshold ("show me only files with 3+ imports") to reduce noise on large projects
12. **Refresh Button** — recalculate import graph without re-scanning from disk
13. **Cluster/Module Boundaries** — auto-colour clusters by strongly connected components to prevent large projects becoming a hairball

---

## Future Feature Ideas

1. **Pulse Animation** — small glowing dot or "data packet" that moves along the tether from importer to imported, showing flow direction visually
2. **Source Sync** — editing source code in a node triggers re-parsing and import recalculation (on manual refresh, not every keystroke)
3. **Export Dependency Graph** — export the import graph as a DOT file or adjacency list for use with external tools like Graphviz
4. **Blast Radius Mode** — click a file and highlight every file that imports it (directly or transitively), answering "if I break this, what else breaks?"
5. **Coupling Score on Tethers** — show how tightly two files are coupled (number of cross-imports, shared types), thicker/brighter tether = tighter coupling
6. **Change Frequency Overlay** — pull from git log to colour nodes by how often they change, files that change together but aren't near each other reveal hidden dependencies
7. **Team Ownership View** — colour nodes by git blame majority author
8. **Function-Level Zoom** — double-click a file node to "zoom into" it and see individual functions/classes as sub-nodes with their own internal call graph
9. **API Surface View** — for each file, show only its public exports as mini-labels on the node edge
10. **Live Terminal Output** — run tests/linter and map output back to nodes (failed test = red node, warning = yellow)

---

## Implementation Architecture

### New files
- `src/parser.rs` — tree-sitter import extraction logic, designed for multiple language grammars
- `src/modes/neural.rs` — Mode 2 render loop: import-based tethers, force-directed physics, directional arrows, centrality scaling

### Modified files
- `src/app.rs` — add `nn_tethers: Vec<(usize, usize)>`, `nn_cam_offset: egui::Vec2`, `nn_zoom: f32` to GravityApp
- `src/nodes.rs` — no changes (keep IdeaNode clean, store dependencies separately)

### Implementation Order
1. Create `src/parser.rs` with tree-sitter import extraction
2. Add `nn_tethers`, `nn_cam_offset`, `nn_zoom` to GravityApp
3. Build basic Mode 2 rendering: same nodes, import-based tethers, arrow heads for direction
4. Add force-directed physics layout
5. Add dead code highlighting
6. Add centrality scaling
7. Polish: circular dependency detection, pulse animations, cluster colouring

---

## Standout Feature Ideas

1. **Live Refactor Preview** — before you move a function from one file to another, simulate the change on the graph and show which tethers would break, add, or redirect — a visual "what if" for refactoring
2. **Import Cost Estimation** — calculate the transitive weight of each import (how many files get pulled in by importing this one module) and show it as a number on the tether — answers "is this import worth the coupling?"
3. **Dependency Layering Enforcer** — define layers (e.g. "routes can import services, services can import models, models import nothing") and the graph visually screams when a tether violates the rule — turns architecture diagrams into live enforceable constraints
4. **Time-Lapse Replay** — scrub through git history and watch the dependency graph grow commit by commit, see when complexity exploded and who introduced it
5. **Interface Contracts on Tethers** — hover a tether between two files and see exactly which functions/classes cross that boundary (not just "A imports B" but "A uses B.authenticate(), B.get_user()")
6. **Fragility Score** — a per-node score: (number of dependents) x (change frequency from git) — high score = this file breaks things often and lots of code depends on it, the most dangerous file in your codebase
7. **Phantom Dependency Detection** — find imports that exist in code but are never actually called (imported but unused), shown as dashed ghost tethers
8. **Cross-Language Bridge Highlighting** — when Python calls a Rust module via FFI, or JS calls a Python API, show those cross-language boundaries as a distinct tether style — most tools are blind to polyglot connections
9. **Dependency Diff Between Branches** — compare two git branches and show which tethers were added/removed/changed, making architectural impact of PRs visual
10. **Automatic Module Boundary Suggestion** — analyze the graph for natural clusters that have few connections between them and suggest "these 5 files should probably be their own package/module"
11. **Impact Ripple Animation** — click a node and watch a ripple propagate outward through all its dependents, with each hop getting dimmer — visualizes how far a change could cascade
12. **God File Detector** — automatically flag files that both import many things AND are imported by many things (high fan-in AND high fan-out) — these are architectural bottlenecks that no amount of testing can save
13. **Dependency Age Mismatch** — flag when a recently modified file depends on a file that hasn't been touched in years — potential compatibility risk
14. **Test Proximity Mapping** — draw faint tethers between test files and the files they test, showing which production files have no test companion nearby
15. **Runtime vs Static View Toggle** — switch between showing static imports (what the code says) and runtime calls (from profiling data or trace logs) — often very different graphs
