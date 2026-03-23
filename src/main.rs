use eframe::egui;

/// Represents a single node (idea or file) in the gravity map.
/// Each node has a position and velocity for physics simulation,
/// and can be displayed as either a generic idea or a Python module.
struct IdeaNode {
    /// Unique identifier for this node
    id: usize,
    /// Display name (e.g. "main.py", "Idea 3")
    title: String,
    /// Text content shown inside the node's window
    content: String,
    /// Current position on the 2D canvas (pixels)
    pos: egui::Pos2,
    /// Current velocity vector, updated each frame by physics
    vel: egui::Vec2,
    /// Whether this node represents a Python file (affects icon display)
    is_python: bool,
}

/// Top-level application state.
/// Holds the current UI mode and all nodes in the scene.
struct GravityApp {
    /// Active mode: "Blueprint" (free layout with repulsion) or "Gravity" (solar system model)
    mode: String,
    /// All nodes currently in the scene
    nodes: Vec<IdeaNode>,
}

impl Default for GravityApp {
    /// Initialises the app in Blueprint mode with an empty node list.
    fn default() -> Self {
        Self {
            mode: "Blueprint".to_owned(),
            nodes: Vec::new(),
        }
    }
}

impl eframe::App for GravityApp {
    /// Main update loop — runs physics then renders UI every frame.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- MODE 1: BLUEPRINT PHYSICS (Repulsion Only) ---
        // Nodes push each other apart so they don't overlap, but have no central attractor.
        if self.mode == "Blueprint" {
            // Simulation timestep — scales how far nodes move per frame
            let dt = 0.1;
            // How strongly nodes push each other apart (higher = stronger repulsion)
            let repulsion_strength = 500.0;
            // Maximum distance at which repulsion is felt (pixels)
            let min_dist = 200.0;
            // Velocity damping per frame (0.0–1.0) — lower values make nodes stop faster
            let damping = 0.85;

            for i in 0..self.nodes.len() {
                // Accumulated repulsion force from all other nodes
                let mut force = egui::Vec2::ZERO;
                for j in 0..self.nodes.len() {
                    if i == j { continue; }
                    // Vector pointing from node j towards node i (the push direction)
                    let diff = self.nodes[i].pos - self.nodes[j].pos;
                    // Distance between the two nodes, clamped to avoid division by zero
                    let dist = diff.length().max(1.0);
                    if dist < min_dist {
                        // Repulsion force: inversely proportional to distance
                        force += diff.normalized() * (repulsion_strength / dist);
                    }
                }
                // Apply damping to bleed off energy each frame, then update position
                let new_vel = (self.nodes[i].vel + force) * damping;
                self.nodes[i].vel = new_vel;
                self.nodes[i].pos += new_vel * dt;
            }
        }
        // --- MODE 2: GRAVITY PHYSICS (Draggable Sun) ---
        // The Sun (main.py) acts as a central attractor. Planets are pulled towards it
        // by a spring-like force and pushed apart from each other by inverse-square repulsion.
        else if self.mode == "Gravity" && !self.nodes.is_empty() {
            // Find the Sun node's index so we can read its (potentially dragged) position
            let sun_idx = self.nodes.iter().position(|n| n.title == "main.py");

            if let Some(s_idx) = sun_idx {
                // Current position of the Sun — planets orbit around this point
                let sun_pos = self.nodes[s_idx].pos;
                // Spring constant for Sun attraction — lower = planets settle further out
                let attraction_strength = 0.08;
                // Strength of planet-to-planet repulsion — higher = more spacing between planets
                let repulsion_strength = 250000.0;
                // Maximum distance at which planets repel each other (pixels)
                let min_dist = 1400.0;
                // Simulation timestep — scales how far planets move per frame
                let dt = 0.1;
                // Velocity friction per frame (0.0–1.0) — bleeds off kinetic energy to help settle
                let friction = 0.9;

                for i in 0..self.nodes.len() {
                    // Skip the Sun — it stays where the user places it
                    if i == s_idx { continue; }

                    // Accumulated force acting on this planet this frame
                    let mut force = egui::Vec2::ZERO;

                    // 1. Attraction to Sun (Spring-like)
                    // Force is proportional to distance: far planets feel a stronger pull,
                    // close planets feel less — this creates a natural equilibrium radius.
                    let diff_sun = sun_pos - self.nodes[i].pos;
                    force += diff_sun * attraction_strength;

                    // 2. Repulsion from other planets (Inverse-square)
                    // Prevents planets from clumping together. Falls off with distance squared
                    // so it only has a meaningful effect at close range.
                    for j in 0..self.nodes.len() {
                        if i == j { continue; }
                        // Vector pointing away from node j (the push direction)
                        let diff = self.nodes[i].pos - self.nodes[j].pos;
                        // Distance between the two nodes, clamped to avoid division by zero
                        let dist = diff.length().max(10.0);
                        if dist < min_dist {
                            force += diff.normalized() * (repulsion_strength / (dist * dist));
                        }
                    }

                    // Integrate force into velocity, then apply friction to drain energy
                    let mut new_vel = (self.nodes[i].vel + force * dt) * friction;

                    // Sleep Threshold — if a planet is barely moving, snap its velocity to zero
                    // so it fully settles instead of drifting forever with tiny residual motion.
                    if new_vel.length() < 0.5 {
                        new_vel = egui::Vec2::ZERO;
                    }

                    self.nodes[i].vel = new_vel;
                    self.nodes[i].pos += new_vel * dt;
                }
            }
        }

        // --- UI RENDERING ---
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🌌 Gravity Map");

            // Mode selector tabs at the top
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.mode, "Blueprint".to_owned(), "Mode 1: Blueprint");
                ui.selectable_value(&mut self.mode, "Gravity".to_owned(), "Mode 2: Gravity");
            });

            ui.separator();

            // --- BLUEPRINT MODE UI ---
            if self.mode == "Blueprint" {
                // Toolbar: buttons to add nodes or clear the canvas
                ui.horizontal(|ui| {
                    // Spawns a generic idea node at the current mouse position
                    if ui.button("➕ Add Idea").clicked() {
                        let id = self.nodes.len();
                        self.nodes.push(IdeaNode {
                            id,
                            title: format!("Idea {}", id),
                            content: String::new(),
                            pos: ctx.input(|i| i.pointer.hover_pos().unwrap_or(egui::pos2(100.0, 100.0))),
                            vel: egui::Vec2::ZERO,
                            is_python: false,
                        });
                    }
                    // Spawns a Python module node at a fixed position
                    if ui.button("🐍 Add Python").clicked() {
                        let id = self.nodes.len();
                        self.nodes.push(IdeaNode {
                            id,
                            title: format!("module_{}.py", id),
                            content: String::new(),
                            pos: egui::pos2(200.0, 200.0),
                            vel: egui::Vec2::ZERO,
                            is_python: true,
                        });
                    }
                    // Removes all nodes from the scene
                    if ui.button("🗑 Clear").clicked() { self.nodes.clear(); }
                });

                // Render each node as a draggable egui window
                for node in &mut self.nodes {
                    // Choose icon based on whether it's a Python file or a generic idea
                    let title = if node.is_python { format!("🐍 {}", node.title) } else { format!("💡 {}", node.title) };
                    egui::Window::new(title)
                        .current_pos(node.pos)
                        .show(ctx, |ui| {
                            // Editable title and content fields
                            ui.text_edit_singleline(&mut node.title);
                            ui.add(egui::TextEdit::multiline(&mut node.content).desired_rows(3));
                            // Handle drag: stop physics velocity and follow the mouse
                            if ui.interact(ui.max_rect(), ui.id(), egui::Sense::drag()).dragged() {
                                node.vel = egui::Vec2::ZERO;
                                node.pos += ui.input(|i| i.pointer.delta());
                            }
                        });
                }
            }
            // --- GRAVITY MODE UI ---
            else {
                ui.horizontal(|ui| {
                    ui.label("🌞 Software Gravity Mode");
                    
                    if ui.button("📁 Scan Project Folder").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.nodes.clear();

                            // Collect all .py files from the selected folder
                            let mut py_files: Vec<(String, u64)> = Vec::new();
                            if let Ok(entries) = std::fs::read_dir(&path) {
                                for entry in entries.flatten() {
                                    let file_path = entry.path();
                                    if file_path.extension().and_then(|s| s.to_str()) == Some("py") {
                                        let file_name = file_path.file_name()
                                            .and_then(|n| n.to_str())
                                            .unwrap_or("unknown.py")
                                            .to_owned();
                                        let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                                        py_files.push((file_name, size));
                                    }
                                }
                            }

                            // 1. Add the Sun (main.py first, or the first file found)
                            // Sort so main.py comes first if it exists
                            py_files.sort_by(|a, b| {
                                let a_is_main = a.0 == "main.py";
                                let b_is_main = b.0 == "main.py";
                                b_is_main.cmp(&a_is_main)
                            });

                            for (i, (name, size)) in py_files.iter().enumerate() {
                                let is_sun = i == 0;
                                self.nodes.push(IdeaNode {
                                    id: i,
                                    // First node becomes "main.py" (the Sun) for tether logic
                                    title: if is_sun { "main.py".to_owned() } else { name.clone() },
                                    content: format!("Size: {} bytes", size),
                                    // Sun at centre, planets slightly offset so physics can spread them
                                    pos: if is_sun { egui::pos2(640.0, 360.0) } else { egui::pos2(650.0, 370.0) },
                                    vel: egui::Vec2::ZERO,
                                    is_python: true,
                                });
                            }
                        }
                    }

                    if ui.button("🚀 Simulate (Mock)").clicked() {
                        // Keep your previous simulation code here for testing...
                    }

                    if ui.button("🗑 Clear").clicked() { self.nodes.clear(); }
                });

                ui.separator();

                // --- DRAW CURVED TETHER LINES (WORMHOLES) ---
                let sun_pos = self.nodes.iter().find(|n| n.title == "main.py").map(|n| n.pos);
                
                if let Some(s_pos) = sun_pos {
                    let painter = ui.painter();
                    for node in &self.nodes {
                        if node.title != "main.py" {
                            let planet_pos = node.pos;
                            
                            // Calculate a control point for the curve bowing towards screen center
                            let mid_point = s_pos + (planet_pos - s_pos) * 0.5;
                            let screen_center = egui::pos2(640.0, 360.0);
                            let control_point = mid_point + (screen_center - mid_point) * 0.2;

                            // 1. Draw the "Glow" layer (wide and faint)
                            painter.add(egui::epaint::QuadraticBezierShape {
                                points: [s_pos, control_point, planet_pos],
                                closed: false,
                                fill: egui::Color32::TRANSPARENT,
                                // .into() converts Stroke to PathStroke
                                stroke: egui::Stroke::new(4.0, egui::Color32::from_rgba_unmultiplied(80, 150, 255, 25)).into(),
                            });

                            // 2. Draw the "Core" layer (thin and bright)
                            painter.add(egui::epaint::QuadraticBezierShape {
                                points: [s_pos, control_point, planet_pos],
                                closed: false,
                                fill: egui::Color32::TRANSPARENT,
                                stroke: egui::Stroke::new(1.2, egui::Color32::from_rgba_unmultiplied(150, 220, 255, 180)).into(),
                            });
                        }
                    }
                }

                // --- RENDER WINDOWS ---
                for node in &mut self.nodes {
                    let is_sun = node.title == "main.py";
                    let title = if is_sun { "🌞 Sun" } else { "🪐 Planet" };

                    let window = egui::Window::new(format!("{} : {}", title, node.title))
                        .current_pos(node.pos)
                        .id(egui::Id::new(node.id + 1000));

                    let response = window.show(ctx, |ui| {
                        ui.label(&node.content);
                        if is_sun { ui.label("(Draggable System Center)"); }
                    });

                    if let Some(inner) = response {
                        if inner.response.dragged() {
                            node.pos += inner.response.drag_delta();
                            node.vel = egui::Vec2::ZERO; 
                        }
                    }
                }
            } // End of Gravity Mode else block

            // Request continuous repainting so physics runs every frame.
            // This MUST stay inside the CentralPanel closure.
            ctx.request_repaint();
        }); // End of CentralPanel closure
    }
}

/// Entry point — launches the egui/eframe window with the GravityApp.
fn main() -> eframe::Result {
    // Window configuration: sets the initial window size to 1280x720
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };
    // Start the native app loop with a default GravityApp instance
    eframe::run_native(
        "Gravity Map",
        options,
        Box::new(|_cc| Ok(Box::new(GravityApp::default()))),
    )
}
