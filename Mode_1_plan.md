# Mode 1: Gravity - Feature Plan

## Core Features Still Needed

1. **Multi-language support** — currently only scans `.py` files, should support Rust, JavaScript, C#, Go, etc.
2. **Search/Filter** — no way to find a specific file node on a large project board
3. **Zoom to Fit** — button to auto-zoom to show the entire project tree
4. **Re-scan / Refresh** — no way to update the map when files change without clearing and re-scanning
5. **File watcher** — auto-detect when files are added/removed/modified and update the map live
6. **Open in editor** — click a node to open that file in your system editor or IDE
7. **Collapse/Expand sub-trees** — collapse a sub-sun and all its children into a single compact node
8. **Import analysis** — parse `import` statements to draw tethers based on actual code dependencies, not just folder structure
9. **Node right-click delete** — Mode 3 has this but Mode 1 doesn't
10. **File size indicators** — visually scale nodes by file size or line count
11. **Syntax-aware preview** — show function/class signatures in the node header instead of just the filename
12. **Git status overlay** — colour nodes by git status (modified, untracked, staged)
13. **Minimap** — small overview in the corner
14. **Save/Load project layout** — persist node positions so re-opening the same project restores your arrangement
15. **Drag nodes without them snapping back** — currently nodes are placed by radial layout but there's no persistence if you manually rearrange
16. **Node statistics** — show line count, function count, class count per file
17. **Dead code highlighting** — files with no imports pointing to them could be visually flagged
18. **Pan with middle mouse** — same issue as Mode 3, left-drag conflicts with window dragging
19. **Custom ignore patterns** — let users configure which folders/files to skip beyond the hardcoded list
20. **Project root indicator** — show the scanned folder path somewhere in the UI

---

## Future Feature Ideas

1. **Complexity Heatmap** — colour nodes by cyclomatic complexity or nesting depth, instantly showing which files are the most tangled and need refactoring
2. **Coupling Score on Tethers** — show how tightly two files are coupled (number of cross-imports, shared types), thicker/brighter tether = tighter coupling, makes architectural debt visible at a glance
3. **Change Frequency Overlay** — pull from git log to colour nodes by how often they change, files that change together but aren't near each other on the map reveal hidden dependencies
4. **Blast Radius Mode** — click a file and highlight every file that imports it (directly or transitively), answering "if I break this, what else breaks?"
5. **Code Smell Badges** — small icons on nodes for detected smells: file too long, too many functions, circular imports, god class
6. **Team Ownership View** — colour nodes by git blame majority author, shows which developer "owns" which part of the codebase and where ownership is fragmented
7. **Diff Overlay** — compare two git commits and highlight which nodes changed, added, or removed, making PRs visual
8. **Architecture Zones** — draw named boundary regions ("API Layer", "Database", "Frontend") and warn when a tether crosses zones in the wrong direction (enforcing layered architecture)
9. **Test Coverage Mapping** — colour nodes by test coverage percentage if coverage data is available, red = untested, green = well-tested
10. **Dependency Cycles Detection** — automatically find and highlight circular import chains with a pulsing red tether, these are invisible in normal code review
11. **File Age Gradient** — colour nodes from fresh (bright) to ancient (dim) based on last modified date, old untouched files might be dead code or tech debt
12. **Function-Level Zoom** — double-click a file node to "zoom into" it and see individual functions/classes as sub-nodes with their own internal call graph
13. **TODO/FIXME Scanner** — parse files for TODO, FIXME, HACK comments and show a badge count on each node, aggregate count in the toolbar
14. **Live Terminal Output** — run a command (tests, linter) and map its output back to nodes, failed test = red node, warning = yellow
15. **API Surface View** — for each file, show only its public exports as mini-labels on the node edge, making the interface visible without opening the file
16. **Orphan Detection** — highlight files that nothing imports and that import nothing, likely dead code or forgotten experiments
17. **Merge Conflict Predictor** — based on git branch activity, flag files that multiple branches are modifying simultaneously
18. **Performance Profiling Overlay** — import profiling data and colour nodes by execution time or memory usage, the hot path becomes literally visible
19. **Documentation Coverage** — badge showing which files have docstrings/comments vs which are undocumented
20. **Architectural Drift Alerts** — define rules like "nothing in /utils should import from /routes" and get visual warnings when the codebase violates them
