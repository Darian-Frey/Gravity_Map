# 🌌 Gravity Map

**Gravity Map** is a dynamic, physics-driven architecture visualizer built in **Rust**. It aims to bridge the gap between abstract project planning (PRDs) and concrete codebase reality through "Software Gravity."

## 🚀 Current Features

### 🛠 Mode 1: Blueprint (The Idea Forge)
* **Node-Based Planning:** Create "Idea Nodes" to map out your PRD, Tech Stack, and logic flow.
* **Repulsion Physics:** Nodes automatically push away from each other to prevent clutter and maintain clarity.
* **Python Modules:** Specialized nodes for planning Python module structures before a single line of code is written.

### 🌞 Mode 2: Gravity (The Code Orbit)
* **Solar System Logic:** Visualize your codebase with the entry point (`main.py`) as the Sun.
* **Dynamic Tethering:** Draggable "Sun" node—move the center of your project and watch all modules (Planets) reorganize their orbits in real-time.
* **Inverse-Square Physics:** High-fidelity attraction and repulsion math for a natural, "living" architectural map.

## 🏗 Tech Stack
* **Language:** Rust 🦀
* **UI Engine:** [egui / eframe](https://github.com/emilk/egui) (Immediate mode GUI)
* **Physics:** Custom Spring-Attraction & Inverse-Square Repulsion.
* **Parsing (Upcoming):** [tree-sitter-python](https://github.com/tree-sitter/tree-sitter-python) for automatic codebase scanning.

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

## 🌌 The Vision
The goal is to create a "Wormhole" between your PRD and your code. By clicking a node in the Blueprint, the map should instantly transport you to the corresponding gravity well in the actual codebase, ensuring that your plan and your execution never drift apart.

---
*Created by Azathoth on the Linux Mint Frontier.*
