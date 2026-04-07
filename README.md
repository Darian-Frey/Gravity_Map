# 🌌 Gravity Map

**Gravity Map** is a dynamic, physics-driven architecture visualiser built in **Rust**. It bridges the gap between abstract project planning and concrete codebase reality through "Software Gravity" — turning folder trees, import graphs, and idea boards into living, interactive maps.

## 🚀 The Three Modes

Gravity Map is built around three complementary modes, all sharing a common infinite canvas with pan, zoom, background grid, and interactive bezier tethers.

### 🌞 Mode 1: Gravity (The Code Skeleton)
The filesystem visualiser. Scan a project folder and watch it bloom into a radial map.
* **Solar System Layout:** The entry point (e.g. `main.py`) becomes the Sun. Folders become sub-suns. Files orbit their parent.
* **Folder Hierarchy Tethers:** Connections show containment — who lives inside whom.
* **Interactive Exploration:** Draggable nodes, hoverable tether labels, depth-based colouring, collapse/expand sub-trees, off-screen sun indicator.

### 🧠 Mode 2: Neural Network (The Code Nervous System) — *In Development*
The dependency graph. Same nodes as Mode 1, but tethers come from real `import` statements parsed by tree-sitter, not folder structure.
* **Force-Directed Layout:** Nodes find their own positions based on connection density.
* **Directional Tethers:** Arrowheads show import direction (who depends on whom).
* **Architectural Insight:** Dead code highlighting, circular dependency detection, centrality scaling, hover-to-trace import paths.

### 🛠 Mode 3: Blueprint (The Idea Forge)
A freeform whiteboard for planning, brainstorming, and PRD work — independent of any codebase.
* **Node-Based Planning:** Create idea nodes, link them with manual tethers, organise your thinking spatially.
* **Categories:** Colour-coded borders for Idea, Task, Bug, Feature, Research, and Done states.
* **Save/Load:** Persist blueprints as JSON — positions, tethers, categories, and metadata all preserved.
* **Right-Click Tools:** Delete nodes and tethers with confirmation dialogs.

## 🌌 The Vision

The end goal is a **wormhole** between your plan and your code:

> Click a Blueprint idea node → jump to the corresponding file in the Gravity map → see in the Neural Network whether it's actually wired into the codebase.

Plan ↔ structure ↔ execution, kept in sync. No more drift between what you intended to build and what's actually there.

## 🏗 Tech Stack
* **Language:** Rust 🦀
* **UI Engine:** [egui / eframe](https://github.com/emilk/egui) (immediate mode GUI)
* **Physics:** Custom spring-attraction & inverse-square repulsion
* **Parsing:** [tree-sitter-python](https://github.com/tree-sitter/tree-sitter-python) and tree-sitter-c-sharp (Mode 2)
* **Serialization:** serde + JSON (Blueprint save/load)

## 🛠 Installation & Setup

1. **Install Rust:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone & Run:**
   ```bash
   git clone https://github.com/Darian-Frey/Gravity_Map.git
   cd Gravity_Map
   cargo run
   ```

## 📚 Project Documentation
* [RoadMap.md](RoadMap.md) — phased development plan across all three modes
* [Mode_1_plan.md](Mode_1_plan.md) — Gravity mode feature plan
* [Mode_2_plan.md](Mode_2_plan.md) — Neural Network mode feature plan
* [Mode_3_plan.md](Mode_3_plan.md) — Blueprint mode feature plan
* [BUGS.md](BUGS.md) — known bugs and fixes
* [IMPROVEMENTS.md](IMPROVEMENTS.md) — improvement ideas and implementations

---
*Created by Azathoth on the Linux Mint Frontier.*
