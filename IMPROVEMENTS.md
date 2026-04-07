# Gravity Map — Improvements Log

A running log of improvement ideas — quality-of-life tweaks, refactors, polish, and small enhancements that don't rise to the level of a roadmap phase. Bigger features belong in [RoadMap.md](RoadMap.md) and the per-mode plan files.

## How to log an improvement

```markdown
### [IMP-###] Short title
- **Mode:** Mode 1 / Mode 2 / Mode 3 / Shared
- **Type:** UX / Performance / Refactor / Polish / Accessibility
- **Priority:** High / Medium / Low
- **Proposed:** YYYY-MM-DD
- **Description:** what should change and why
- **Notes:** implementation hints, related files, prior art
```

When implemented, add:
```markdown
- **Implemented:** YYYY-MM-DD — commit `abc1234`
- **Outcome:** one-line summary of what shipped and any follow-ups
```

---

## 💡 Proposed

### [IMP-002] Replace hand-rolled timestamp generator in blueprint_io
- **Mode:** Mode 3
- **Type:** Refactor
- **Priority:** Low
- **Proposed:** 2026-04-07
- **Description:** [src/blueprint_io.rs](src/blueprint_io.rs) implements its own timestamp string generator (~lines 101–115) instead of pulling a date crate. It works but is fragile and weirdly specific.
- **Notes:** Two clean options: (a) use `std::time::SystemTime` + manual ISO 8601 formatting (no new dep, ~10 lines), or (b) add `chrono` to Cargo.toml and use `Utc::now().to_rfc3339()`. Prefer (a) unless we end up needing date math elsewhere.

---

## 🚧 In Progress

*Nothing in progress.*

---

## ✅ Implemented

### [IMP-001] Unify pan/zoom state into a shared Canvas struct
- **Mode:** Shared
- **Type:** Refactor
- **Priority:** High
- **Proposed:** 2026-04-07
- **Implemented:** 2026-04-07 — uncommitted
- **Outcome:** Introduced `CanvasState { offset, zoom }` in [src/canvas.rs](src/canvas.rs) with `world_to_screen`, `handle_pan_zoom`, and `draw_grid` as methods. [src/app.rs](src/app.rs) now holds three independent canvas fields (`gravity_canvas`, `neural_canvas`, `blueprint_canvas`) instead of duplicated pairs. [src/blueprint_io.rs](src/blueprint_io.rs) translates to/from `CanvasState` while keeping the JSON wire format unchanged so existing saved blueprints still load. Immediately paid for itself: BUG-001 was fixed in one place (`handle_pan_zoom`) and the fix took effect for all three modes simultaneously, and [IMP-Phase-1.2] (`zoom_to_fit`) landed as a single method shared by both modes that have nodes.

### [IMP-003] Unify node window patterns between Mode 1 and Mode 3
- **Mode:** Shared (Mode 1 + Mode 3)
- **Type:** UX / Refactor
- **Priority:** Medium
- **Proposed:** 2026-04-07
- **Implemented:** 2026-04-07 — uncommitted
- **Outcome:** New [src/node_window.rs](src/node_window.rs) with `render_node_window(...)` shell that handles positioning, drag-back-to-world-space sync, and right-click-inside detection. Both [src/modes/gravity.rs](src/modes/gravity.rs) and [src/modes/blueprint.rs](src/modes/blueprint.rs) now go through this helper; bodies (CollapsingHeader+source editor for Mode 1, title+category+content for Mode 3) stay in the per-mode files via closures. Drag-back math (`delta / zoom`) now lives in exactly one place. Mode 2 will pick this up automatically when its render loop is implemented.
