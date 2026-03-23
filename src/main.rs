use eframe::egui;

/// Represents a single node (idea or file) in the gravity map.
struct IdeaNode {
    id: usize,
    title: String,
    content: String,
    pos: egui::Pos2,
    vel: egui::Vec2,
    is_python: bool,
    depth: usize, // 0 = root, 1 = subfolder, etc.
}

struct GravityApp {
    mode: String,
    nodes: Vec<IdeaNode>,
    orbit_enabled: bool,
}

impl Default for GravityApp {
    fn default() -> Self {
        Self {
            mode: "Blueprint".to_owned(),
            nodes: Vec::new(),
            orbit_enabled: false,
        }
    }
}

impl eframe::App for GravityApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- MODE 1: BLUEPRINT PHYSICS ---
        if self.mode == "Blueprint" {
            let dt = 0.1;
            let repulsion_strength = 500.0;
            let min_dist = 200.0;
            let damping = 0.85;

            for i in 0..self.nodes.len() {
                let mut force = egui::Vec2::ZERO;
                for j in 0..self.nodes.len() {
                    if i == j {
                        continue;
                    }
                    let diff = self.nodes[i].pos - self.nodes[j].pos;
                    let dist = diff.length().max(1.0);
                    if dist < min_dist {
                        force += diff.normalized() * (repulsion_strength / dist);
                    }
                }
                let new_vel = (self.nodes[i].vel + force) * damping;
                self.nodes[i].vel = new_vel;
                self.nodes[i].pos += new_vel * dt;
            }
        }
        // --- MODE 2: GRAVITY PHYSICS (Orbital Depth + Motion) ---
        else if self.mode == "Gravity" && !self.nodes.is_empty() {
            let sun_idx = self.nodes.iter().position(|n| n.title == "main.py");

            if let Some(s_idx) = sun_idx {
                let sun_pos = self.nodes[s_idx].pos;
                let attraction_strength = 0.1;
                let repulsion_strength = 200000.0;
                let orbit_speed = 1.5; // Cranked up for visible motion
                let dt = 0.1;
                let friction = 0.98; // High friction keeps it "silky" but moving

                for i in 0..self.nodes.len() {
                    if i == s_idx {
                        continue;
                    }

                    let mut force = egui::Vec2::ZERO;
                    let diff_sun = self.nodes[i].pos - sun_pos;
                    let dist_sun = diff_sun.length().max(1.0);

                    // 1. Target Orbit Logic (The Rings)
                    let target_orbit = 250.0 + (self.nodes[i].depth as f32 * 200.0);
                    let orbit_error = dist_sun - target_orbit;
                    force -= diff_sun.normalized() * orbit_error * attraction_strength;

                    // 2. ORBITAL MOTION (The Spinning)
                    if self.orbit_enabled {
                        let tangent = egui::vec2(-diff_sun.y, diff_sun.x).normalized();
                        force += tangent * orbit_speed;
                    }

                    // 3. Repulsion from other planets
                    for j in 0..self.nodes.len() {
                        if i == j {
                            continue;
                        }
                        let diff = self.nodes[i].pos - self.nodes[j].pos;
                        let dist = diff.length().max(10.0);
                        if dist < 800.0 {
                            force += diff.normalized() * (repulsion_strength / (dist * dist));
                        }
                    }

                    let new_vel = (self.nodes[i].vel + force * dt) * friction;
                    self.nodes[i].vel = new_vel;
                    self.nodes[i].pos += new_vel * dt;
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🌌 Gravity Map");

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.mode, "Blueprint".to_owned(), "Mode 1: Blueprint");
                ui.selectable_value(&mut self.mode, "Gravity".to_owned(), "Mode 2: Gravity");
            });

            ui.separator();

            if self.mode == "Blueprint" {
                // ... [Add Idea/Python buttons code]
                ui.horizontal(|ui| {
                    if ui.button("➕ Add Idea").clicked() {
                        let id = self.nodes.len();
                        self.nodes.push(IdeaNode {
                            id,
                            title: format!("Idea {}", id),
                            content: String::new(),
                            pos: ctx.input(|i| {
                                i.pointer.hover_pos().unwrap_or(egui::pos2(100.0, 100.0))
                            }),
                            vel: egui::Vec2::ZERO,
                            is_python: false,
                            depth: 0,
                        });
                    }
                    if ui.button("🐍 Add Python").clicked() {
                        let id = self.nodes.len();
                        self.nodes.push(IdeaNode {
                            id,
                            title: format!("module_{}.py", id),
                            content: String::new(),
                            pos: egui::pos2(200.0, 200.0),
                            vel: egui::Vec2::ZERO,
                            is_python: true,
                            depth: 0,
                        });
                    }
                    if ui.button("🗑 Clear").clicked() {
                        self.nodes.clear();
                    }

                    ui.add_space(8.0);
                    if ui
                        .button(if self.orbit_enabled {
                            "⏸ Pause Orbit"
                        } else {
                            "▶ Resume Orbit"
                        })
                        .clicked()
                    {
                        self.orbit_enabled = !self.orbit_enabled;
                    }
                });

                for node in &mut self.nodes {
                    let title = if node.is_python {
                        format!("🐍 {}", node.title)
                    } else {
                        format!("💡 {}", node.title)
                    };
                    egui::Window::new(title)
                        .current_pos(node.pos)
                        .show(ctx, |ui| {
                            ui.text_edit_singleline(&mut node.title);
                            ui.add(egui::TextEdit::multiline(&mut node.content).desired_rows(3));
                            if ui
                                .interact(ui.max_rect(), ui.id(), egui::Sense::drag())
                                .dragged()
                            {
                                node.vel = egui::Vec2::ZERO;
                                node.pos += ui.input(|i| i.pointer.delta());
                            }
                        });
                }
            } else {
                // --- GRAVITY MODE UI ---
                ui.horizontal(|ui| {
                    if ui.button("📁 Scan Project").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.nodes.clear();
                            let mut temp_nodes = Vec::new();
                            let mut id_counter = 0;

                            fn scan_recursive(
                                dir: &std::path::Path,
                                nodes: &mut Vec<IdeaNode>,
                                id_gen: &mut usize,
                                depth: usize,
                            ) {
                                if let Ok(entries) = std::fs::read_dir(dir) {
                                    for entry in entries.flatten() {
                                        let path = entry.path();
                                        let file_name =
                                            path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                                        if path.is_dir() {
                                            if !["venv", ".git", "__pycache__"].contains(&file_name)
                                            {
                                                scan_recursive(&path, nodes, id_gen, depth + 1);
                                            }
                                        } else if path.extension().and_then(|s| s.to_str())
                                            == Some("py")
                                        {
                                            nodes.push(IdeaNode {
                                                id: *id_gen,
                                                title: file_name.to_owned(),
                                                content: format!(
                                                    "Path: {}\nDepth: {}",
                                                    path.to_string_lossy(),
                                                    depth
                                                ),
                                                pos: egui::pos2(640.0, 360.0),
                                                vel: egui::Vec2::ZERO,
                                                is_python: true,
                                                depth,
                                            });
                                            *id_gen += 1;
                                        }
                                    }
                                }
                            }
                            scan_recursive(&path, &mut temp_nodes, &mut id_counter, 0);
                            if !temp_nodes.is_empty() {
                                let sun_idx = temp_nodes
                                    .iter()
                                    .position(|n| n.title == "main.py")
                                    .unwrap_or(0);
                                let mut sun_node = temp_nodes.remove(sun_idx);
                                sun_node.title = "main.py".to_owned();
                                self.nodes.push(sun_node);
                                self.nodes.extend(temp_nodes);
                                for (idx, node) in self.nodes.iter_mut().enumerate() {
                                    node.id = idx;
                                }
                            }
                        }
                    }
                    if ui.button("🗑 Clear").clicked() {
                        self.nodes.clear();
                    }

                    ui.add_space(8.0);
                    if ui
                        .button(if self.orbit_enabled {
                            "⏸ Pause Orbit"
                        } else {
                            "▶ Resume Orbit"
                        })
                        .clicked()
                    {
                        self.orbit_enabled = !self.orbit_enabled;
                    }
                });

                ui.separator();

                // --- TETHER LINES ---
                let sun_pos = self
                    .nodes
                    .iter()
                    .find(|n| n.title == "main.py")
                    .map(|n| n.pos);
                if let Some(s_pos) = sun_pos {
                    let painter = ui.painter();
                    for node in &self.nodes {
                        if node.title != "main.py" {
                            let mid = s_pos + (node.pos - s_pos) * 0.5;
                            let cp = mid + (egui::pos2(640.0, 360.0) - mid) * 0.2;
                            painter.add(egui::epaint::QuadraticBezierShape {
                                points: [s_pos, cp, node.pos],
                                closed: false,
                                fill: egui::Color32::TRANSPARENT,
                                stroke: egui::Stroke::new(
                                    4.0,
                                    egui::Color32::from_rgba_unmultiplied(80, 150, 255, 25),
                                )
                                .into(),
                            });
                            painter.add(egui::epaint::QuadraticBezierShape {
                                points: [s_pos, cp, node.pos],
                                closed: false,
                                fill: egui::Color32::TRANSPARENT,
                                stroke: egui::Stroke::new(
                                    1.2,
                                    egui::Color32::from_rgba_unmultiplied(150, 220, 255, 180),
                                )
                                .into(),
                            });
                        }
                    }
                }

                // --- CORRECTED RENDER WINDOWS LOOP ---
                for node in &mut self.nodes {
                    let is_sun = node.title == "main.py";
                    let title = if is_sun { "🌞 Sun" } else { "🪐 Planet" };

                    let response = egui::Window::new(format!("{} : {}", title, node.title))
                        .current_pos(node.pos)
                        .id(egui::Id::new(node.id + 1000))
                        .show(ctx, |ui| {
                            ui.label(&node.content);
                        });

                    if let Some(inner) = response {
                        // FIX: Dragging must be handled via the Window's response
                        if inner.response.dragged() {
                            node.pos += inner.response.drag_delta();
                            node.vel = egui::Vec2::ZERO;
                        }
                    }
                }
            }
            ctx.request_repaint();
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Gravity Map",
        options,
        Box::new(|_cc| Ok(Box::new(GravityApp::default()))),
    )
}
