# Gravity Map — Bug Tracker

A running log of bugs found in Gravity Map. Move entries from **Open** to **Fixed** as they're resolved, and keep a short note on the fix so future-you knows what changed.

## How to log a bug

```markdown
### [BUG-###] Short title
- **Mode:** Mode 1 / Mode 2 / Mode 3 / Shared
- **Severity:** Critical / High / Medium / Low
- **Found:** YYYY-MM-DD
- **Steps to reproduce:**
  1. ...
  2. ...
- **Expected:** what should happen
- **Actual:** what actually happens
- **Notes:** stack traces, screenshots, hypotheses
```

When fixed, add:
```markdown
- **Fixed:** YYYY-MM-DD — commit `abc1234`
- **Fix summary:** one-line explanation of the root cause and resolution
```

---

## 🔴 Open

### [BUG-002] README overstates Mode 1 as "physics-driven"
- **Mode:** Docs
- **Severity:** Low
- **Found:** 2026-04-07
- **Steps to reproduce:**
  1. Read [README.md](README.md) intro and Tech Stack section
  2. Inspect [src/modes/gravity.rs:301-332](src/modes/gravity.rs#L301-L332) `radial_layout`
- **Expected:** README should match implementation
- **Actual:** README claims "physics-driven architecture visualiser" with "Custom spring-attraction & inverse-square repulsion", but Mode 1 currently uses a deterministic angle-slice radial layout with no force simulation at all. The first real physics in the project will be Mode 2's force-directed layout (RoadMap 3B.3).
- **Notes:** Either soften the README language now (e.g. "interactive architecture visualiser" + note that physics arrives in Mode 2), or leave it and treat it as forward-looking. Flagging so we make a deliberate choice rather than letting marketing copy drift from reality.

---

## ✅ Fixed

### [BUG-001] Left-drag pan conflicts with egui window dragging
- **Mode:** Shared (affected Mode 1 and Mode 3)
- **Severity:** Medium
- **Found:** 2026-04-07
- **Fixed:** 2026-04-07 — uncommitted (RoadMap item 1.1)
- **Steps to reproduce (original):**
  1. Open any mode with nodes on screen
  2. Try to left-drag on empty canvas to pan
  3. Try to drag a node window
- **Fix summary:** Restricted canvas pan to **middle-mouse drag** or **Ctrl + left-drag**, leaving plain left-drag free for window/node interaction. Implemented in `CanvasState::handle_pan_zoom` ([src/canvas.rs](src/canvas.rs)) so all three modes inherit the fix from a single place.
- **Notes:** Originally tried Space + left-drag (Figma pattern) but on Linux Mint Cinnamon the compositor freezes pointer-move events the moment space is held while a mouse button is down — `hover_pos` literally stops updating until space is released. Diagnostic logging confirmed this is OS/compositor-level, not an egui or app bug. Switched to Ctrl as the modifier; Ctrl+left-drag works cleanly. The OS-level space behaviour is documented in a code comment in [src/canvas.rs](src/canvas.rs) so future-us doesn't fall into the same trap.
