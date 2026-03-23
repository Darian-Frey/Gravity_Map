# Mode 3: Blueprint - Feature Plan

## Core Features Still Needed

1. **Undo/Redo** — no way to reverse mistakes currently
2. **Multi-select** — can't select and move/delete multiple nodes at once
3. **Search/Filter** — no way to find a node by name when the board gets busy
4. **Zoom to Fit** — a button that auto-zooms to show all nodes on screen
5. **Node resize** — all nodes are the same size regardless of content
6. **Tether labels** — connections have no description, you can't annotate what the relationship means
7. **Tether direction** — lines are undirected, no way to show "A leads to B" vs "B leads to A" (arrows)
8. **Drag to link** — currently requires clicking "Link Nodes" then clicking two "Connect" buttons, should be able to drag from one node edge to another directly
9. **Node duplication** — no way to clone a node
10. **Auto-save** — only manual save currently, easy to lose work
11. **Unsaved changes indicator** — no visual cue that you have unsaved work
12. **Keyboard shortcuts** — no hotkeys for common actions (Ctrl+S save, Delete node, Ctrl+Z undo, etc.)
13. **Node collapse in save** — collapsed/expanded state isn't persisted in the JSON
14. **Grid snapping** — option to snap nodes to the grid for neat alignment
15. **Minimap** — small overview in the corner showing where you are on the canvas
16. **Node grouping / colour regions** — draw coloured rectangles behind clusters of nodes to visually group them
17. **Pan with middle mouse** — currently only left-drag pans, which conflicts with window dragging

---

## Future Feature Ideas

## 1. Time Decay Indicators
Nodes that haven't been edited in X days start to visually fade or get a dust/cobweb effect. Forces you to confront stale ideas rather than letting them rot invisibly on a board.

## 2. Dependency Chains & Critical Path
Instead of just connecting nodes, mark tethers as "blocks" or "depends on". Then auto-highlight the critical path — the longest chain of blockers. Most planners let you connect things but never tell you *what's actually holding everything up*.

## 3. Heat Map / Activity Overlay
Colour nodes by how recently or frequently they've been edited. Instantly see which parts of your plan are getting attention and which are being ignored.

## 4. Node Weight / Effort Sizing
Let each node carry a size estimate (S/M/L or story points). Show it visually — bigger effort = bigger node. Most whiteboards treat all ideas as equal, which hides the 2-hour task sitting next to the 2-week task.

## 5. Snapshot History
Save automatic snapshots as you work. Let you scrub back through time to see how the plan evolved. Every whiteboard lacks an "undo history" for the *thinking process itself*.

## 6. Auto-Cluster / Gravity Grouping
A button that pulls nodes with many connections closer together and pushes isolated nodes outward. Reveals natural clusters you didn't consciously organise.

## 7. Focus Mode
Click a node and dim everything except it and its direct connections (1-hop or 2-hop). Whiteboards get noisy fast — this gives you tunnel vision when you need it.

## 8. Embedded Checklists Inside Nodes
Turn a node's content into a checklist with progress tracking. The node title bar shows a progress indicator (3/7 done). Most planners force you to choose between a card *or* a checklist, never both.

## 9. Export to Markdown / Task List
One-click export the whole blueprint as a structured markdown document with headings per category, nested by connections. Bridge the gap between visual planning and actionable documentation.

## 10. Conflict Detection
If two "Bug" nodes both connect to the same "Feature" node, flag it visually — you might have conflicting fixes targeting the same thing. No whiteboard does this.
