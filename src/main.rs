use eframe::egui;

/// Represents a single node (idea or file) in the gravity map.
struct IdeaNode {
    id: usize,
    title: String,
    content: String,
    /// Position used in Gravity mode (Mode 2)
    pos: egui::Pos2,
    vel: egui::Vec2,
    /// Independent position used in Blueprint mode (Mode 1)
    blueprint_pos: egui::Pos2,
    blueprint_vel: egui::Vec2,
    is_python: bool,
    depth: usize, // 0 = root, 1 = subfolder, etc.
}

struct GravityApp {
    mode: String,
    nodes: Vec<IdeaNode>,
    /// Camera offset — drag the background to pan across the infinite canvas
    cam_offset: egui::Vec2,
    /// Zoom level (1.0 = 100%), zooms towards mouse pointer
    zoom: f32,
    /// One-shot trigger: +1 = collapse all this frame, -1 = expand all this frame, 0 = do nothing
    collapse_action: i8,
}

impl Default for GravityApp {
    fn default() -> Self {
        Self {
            mode: "Blueprint".to_owned(),
            nodes: Vec::new(),
            cam_offset: egui::Vec2::ZERO,
            zoom: 1.0,
            collapse_action: 0,
        }
    }
}

/// Returns a (glow, core) colour pair for tether lines based on folder depth.
fn depth_colors(depth: usize) -> (egui::Color32, egui::Color32) {
    match depth {
        0 => (
            egui::Color32::from_rgba_unmultiplied(80, 150, 255, 25),   // blue glow
            egui::Color32::from_rgba_unmultiplied(150, 220, 255, 180), // blue core
        ),
        1 => (
            egui::Color32::from_rgba_unmultiplied(80, 255, 120, 25),   // green glow
            egui::Color32::from_rgba_unmultiplied(150, 255, 180, 180), // green core
        ),
        2 => (
            egui::Color32::from_rgba_unmultiplied(255, 180, 50, 25),   // orange glow
            egui::Color32::from_rgba_unmultiplied(255, 220, 130, 180), // orange core
        ),
        3 => (
            egui::Color32::from_rgba_unmultiplied(255, 80, 80, 25),    // red glow
            egui::Color32::from_rgba_unmultiplied(255, 150, 150, 180), // red core
        ),
        4 => (
            egui::Color32::from_rgba_unmultiplied(200, 80, 255, 25),   // purple glow
            egui::Color32::from_rgba_unmultiplied(220, 160, 255, 180), // purple core
        ),
        _ => (
            egui::Color32::from_rgba_unmultiplied(255, 255, 80, 25),   // yellow glow
            egui::Color32::from_rgba_unmultiplied(255, 255, 180, 180), // yellow core
        ),
    }
}

impl eframe::App for GravityApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- MODE 2: GRAVITY PHYSICS (Orbital Depth + Motion) ---
        if self.mode == "Gravity" && !self.nodes.is_empty() {
            let sun_idx = self.nodes.iter().position(|n| n.title == "main.py");

            if let Some(s_idx) = sun_idx {
                let sun_pos = self.nodes[s_idx].pos;
                let attraction_strength = 0.1;
                let repulsion_strength = 200000.0;
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

                    // 2. Repulsion from other planets
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

            // --- INFINITE CANVAS PANNING & ZOOM ---
            let panel_rect = ui.available_rect_before_wrap();
            let panel_response = ui.interact(panel_rect, ui.id().with("canvas_pan"), egui::Sense::drag());
            if panel_response.dragged() {
                self.cam_offset += panel_response.drag_delta();
            }

            // Mouse wheel zoom — zoom towards the pointer position
            let scroll_delta = ctx.input(|i| i.smooth_scroll_delta.y);
            if scroll_delta != 0.0 {
                let zoom_factor = 1.0 + scroll_delta * 0.002;
                let new_zoom = (self.zoom * zoom_factor).clamp(0.1, 5.0);
                // Zoom towards mouse pointer
                if let Some(mouse_pos) = ctx.input(|i| i.pointer.hover_pos()) {
                    let mouse_world_before = (mouse_pos - self.cam_offset) / self.zoom;
                    self.zoom = new_zoom;
                    let mouse_world_after = (mouse_pos - self.cam_offset) / self.zoom;
                    self.cam_offset += (mouse_world_after - mouse_world_before) * self.zoom;
                } else {
                    self.zoom = new_zoom;
                }
            }

            // Macro to convert world pos to screen pos
            let zoom = self.zoom;
            let cam = self.cam_offset;
            let to_screen = |world_pos: egui::Pos2| -> egui::Pos2 {
                egui::pos2(
                    world_pos.x * zoom + cam.x,
                    world_pos.y * zoom + cam.y,
                )
            };

            // --- BACKGROUND GRID ---
            {
                let painter = ui.painter();
                let grid_spacing = 100.0_f32;
                let grid_color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, 15);
                let label_color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, 40);

                let left = panel_rect.left();
                let right = panel_rect.right();
                let top = panel_rect.top();
                let bottom = panel_rect.bottom();

                let scaled_spacing = grid_spacing * zoom;

                // Vertical lines
                let first_x = ((left - cam.x) / scaled_spacing).floor() as i32;
                let last_x = ((right - cam.x) / scaled_spacing).ceil() as i32;
                for ix in first_x..=last_x {
                    let x = ix as f32 * scaled_spacing + cam.x;
                    painter.line_segment(
                        [egui::pos2(x, top), egui::pos2(x, bottom)],
                        egui::Stroke::new(1.0, grid_color),
                    );
                    if ix % 5 == 0 {
                        painter.text(
                            egui::pos2(x + 2.0, top + 2.0),
                            egui::Align2::LEFT_TOP,
                            format!("{}", ix as f32 * grid_spacing),
                            egui::FontId::proportional(10.0),
                            label_color,
                        );
                    }
                }

                // Horizontal lines
                let first_y = ((top - cam.y) / scaled_spacing).floor() as i32;
                let last_y = ((bottom - cam.y) / scaled_spacing).ceil() as i32;
                for iy in first_y..=last_y {
                    let y = iy as f32 * scaled_spacing + cam.y;
                    painter.line_segment(
                        [egui::pos2(left, y), egui::pos2(right, y)],
                        egui::Stroke::new(1.0, grid_color),
                    );
                    if iy % 5 == 0 {
                        painter.text(
                            egui::pos2(left + 2.0, y + 2.0),
                            egui::Align2::LEFT_TOP,
                            format!("{}", iy as f32 * grid_spacing),
                            egui::FontId::proportional(10.0),
                            label_color,
                        );
                    }
                }
            }

            if self.mode == "Blueprint" {
                // ... [Add Idea/Python buttons code]
                ui.horizontal(|ui| {
                    if ui.button("➕ Add Idea").clicked() {
                        let id = self.nodes.len();
                        let spawn_pos = ctx.input(|i| {
                            i.pointer.hover_pos().unwrap_or(egui::pos2(100.0, 100.0))
                        });
                        self.nodes.push(IdeaNode {
                            id,
                            title: format!("Idea {}", id),
                            content: String::new(),
                            pos: spawn_pos,
                            vel: egui::Vec2::ZERO,
                            blueprint_pos: spawn_pos,
                            blueprint_vel: egui::Vec2::ZERO,
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
                            blueprint_pos: egui::pos2(200.0, 200.0),
                            blueprint_vel: egui::Vec2::ZERO,
                            is_python: true,
                            depth: 0,
                        });
                    }
                    if ui.button("🗑 Clear").clicked() {
                        self.nodes.clear();
                    }

                    ui.add_space(8.0);
                    if ui.button("🌞 Return to Sun").clicked() {
                        if let Some(sun) = self.nodes.iter().find(|n| n.title == "main.py") {
                            let viewport_center = egui::vec2(panel_rect.center().x, panel_rect.center().y);
                            self.cam_offset = viewport_center - sun.blueprint_pos.to_vec2() * self.zoom;
                        }
                    }

                    ui.add_space(8.0);
                    if ui.button("▼ Collapse All").clicked() {
                        self.collapse_action = 1;
                    }
                    if ui.button("▶ Expand All").clicked() {
                        self.collapse_action = -1;
                    }
                });

                // --- TETHER LINES (sun-to-planet, matching Mode 2 style) ---
                {
                    let painter = ui.painter();
                    let screen_center = egui::pos2(panel_rect.center().x, panel_rect.center().y);
                    let sun_screen = self.nodes.iter()
                        .find(|n| n.title == "main.py")
                        .map(|n| to_screen(n.blueprint_pos));
                    if let Some(s_pos) = sun_screen {
                        for node in &self.nodes {
                            if node.title != "main.py" {
                                let planet_screen = to_screen(node.blueprint_pos);
                                let mid = s_pos + (planet_screen - s_pos) * 0.5;
                                let cp = mid + (screen_center - mid) * 0.2;
                                let (glow_color, core_color) = depth_colors(node.depth);
                                painter.add(egui::epaint::QuadraticBezierShape {
                                    points: [s_pos, cp, planet_screen],
                                    closed: false,
                                    fill: egui::Color32::TRANSPARENT,
                                    stroke: egui::Stroke::new(4.0, glow_color).into(),
                                });
                                painter.add(egui::epaint::QuadraticBezierShape {
                                    points: [s_pos, cp, planet_screen],
                                    closed: false,
                                    fill: egui::Color32::TRANSPARENT,
                                    stroke: egui::Stroke::new(1.2, core_color).into(),
                                });
                            }
                        }
                    }
                }

                for node in &mut self.nodes {
                    let title = if node.is_python {
                        format!("🐍 {}", node.title)
                    } else {
                        format!("💡 {}", node.title)
                    };
                    let screen_pos = to_screen(node.blueprint_pos);
                    let win_id = egui::Id::new(node.id + 2000);

                    if self.collapse_action != 0 {
                        let mut state = egui::collapsing_header::CollapsingState::load_with_default_open(ctx, win_id.with("collapsing"), true);
                        state.set_open(self.collapse_action < 0);
                        state.store(ctx);
                    }

                    let response = egui::Window::new(title)
                        .current_pos(screen_pos)
                        .movable(true)
                        .constrain(false)
                        .id(win_id)
                        .show(ctx, |ui| {
                            ui.label(&node.title);
                            egui::CollapsingHeader::new("Source Code")
                                .id_salt(node.id)
                                .show(ui, |ui| {
                                    egui::ScrollArea::vertical()
                                        .max_height(300.0)
                                        .show(ui, |ui| {
                                            ui.add(
                                                egui::TextEdit::multiline(&mut node.content)
                                                    .code_editor()
                                                    .desired_width(400.0),
                                            );
                                        });
                                });
                        });

                    if let Some(inner) = response {
                        let actual_pos = inner.response.rect.min;
                        let delta = actual_pos - screen_pos;
                        if delta.length() > 0.5 {
                            node.blueprint_vel = egui::Vec2::ZERO;
                            node.blueprint_pos += delta / zoom;
                        }
                    }
                }

                // --- SUN OFF-SCREEN INDICATOR (Blueprint) ---
                if let Some(sun) = self.nodes.iter().find(|n| n.title == "main.py") {
                    let sun_screen = to_screen(sun.blueprint_pos);
                    let margin = 40.0;
                    let is_offscreen = sun_screen.x < panel_rect.left()
                        || sun_screen.x > panel_rect.right()
                        || sun_screen.y < panel_rect.top()
                        || sun_screen.y > panel_rect.bottom();

                    if is_offscreen {
                        let center = panel_rect.center();
                        let dir = (sun_screen - center).normalized();
                        let half_w = panel_rect.width() * 0.5 - margin;
                        let half_h = panel_rect.height() * 0.5 - margin;
                        let scale = if dir.x.abs() > 0.001 && dir.y.abs() > 0.001 {
                            (half_w / dir.x.abs()).min(half_h / dir.y.abs())
                        } else if dir.x.abs() > 0.001 {
                            half_w / dir.x.abs()
                        } else {
                            half_h / dir.y.abs()
                        };
                        let indicator_pos = center + dir * scale;
                        let painter = ui.painter();
                        let arrow_size = 12.0;
                        let perp = egui::vec2(-dir.y, dir.x);
                        let tip = indicator_pos + dir * arrow_size;
                        let left = indicator_pos - dir * 4.0 + perp * arrow_size * 0.6;
                        let right = indicator_pos - dir * 4.0 - perp * arrow_size * 0.6;
                        painter.add(egui::epaint::PathShape::convex_polygon(
                            vec![tip, left, right],
                            egui::Color32::from_rgb(255, 200, 50),
                            egui::Stroke::NONE,
                        ));
                        painter.text(
                            indicator_pos - dir * 18.0,
                            egui::Align2::CENTER_CENTER,
                            "Sun",
                            egui::FontId::proportional(12.0),
                            egui::Color32::from_rgb(255, 200, 50),
                        );
                    }
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
                                            let source = std::fs::read_to_string(&path)
                                                .unwrap_or_else(|_| "(could not read file)".to_owned());
                                            nodes.push(IdeaNode {
                                                id: *id_gen,
                                                title: file_name.to_owned(),
                                                content: source,
                                                pos: egui::pos2(640.0, 360.0),
                                                vel: egui::Vec2::ZERO,
                                                blueprint_pos: egui::pos2(640.0, 360.0),
                                                blueprint_vel: egui::Vec2::ZERO,
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
                                sun_node.pos = egui::pos2(640.0, 360.0);
                                sun_node.blueprint_pos = egui::pos2(640.0, 360.0);
                                self.nodes.push(sun_node);

                                // Spread planets in a circle around the sun
                                let planet_count = temp_nodes.len();
                                for (i, mut planet) in temp_nodes.into_iter().enumerate() {
                                    let angle = (i as f32 / planet_count as f32) * std::f32::consts::TAU;
                                    let radius = 250.0 + (planet.depth as f32 * 200.0);
                                    let spread_pos = egui::pos2(
                                        640.0 + angle.cos() * radius,
                                        360.0 + angle.sin() * radius,
                                    );
                                    planet.pos = spread_pos;
                                    planet.blueprint_pos = spread_pos;
                                    self.nodes.push(planet);
                                }

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
                    if ui.button("🌞 Return to Sun").clicked() {
                        if let Some(sun) = self.nodes.iter().find(|n| n.title == "main.py") {
                            let viewport_center = egui::vec2(panel_rect.center().x, panel_rect.center().y);
                            self.cam_offset = viewport_center - sun.pos.to_vec2() * self.zoom;
                        }
                    }

                    ui.add_space(8.0);
                    if ui.button("▼ Collapse All").clicked() {
                        self.collapse_action = 1;
                    }
                    if ui.button("▶ Expand All").clicked() {
                        self.collapse_action = -1;
                    }
                });

                ui.separator();

                // --- TETHER LINES ---
                let sun_pos = self
                    .nodes
                    .iter()
                    .find(|n| n.title == "main.py")
                    .map(|n| to_screen(n.pos));
                if let Some(s_pos) = sun_pos {
                    let painter = ui.painter();
                    for node in &self.nodes {
                        if node.title != "main.py" {
                            let planet_screen = to_screen(node.pos);
                            let mid = s_pos + (planet_screen - s_pos) * 0.5;
                            let screen_center = egui::pos2(640.0, 360.0);
                            let cp = mid + (screen_center - mid) * 0.2;
                            let (glow_color, core_color) = depth_colors(node.depth);
                            painter.add(egui::epaint::QuadraticBezierShape {
                                points: [s_pos, cp, planet_screen],
                                closed: false,
                                fill: egui::Color32::TRANSPARENT,
                                stroke: egui::Stroke::new(4.0, glow_color).into(),
                            });
                            painter.add(egui::epaint::QuadraticBezierShape {
                                points: [s_pos, cp, planet_screen],
                                closed: false,
                                fill: egui::Color32::TRANSPARENT,
                                stroke: egui::Stroke::new(1.2, core_color).into(),
                            });
                        }
                    }
                }

                // --- RENDER WINDOWS ---
                for node in &mut self.nodes {
                    let is_sun = node.title == "main.py";
                    let title = if is_sun { "🌞 Sun" } else { "🪐 Planet" };
                    let screen_pos = to_screen(node.pos);
                    let win_id = egui::Id::new(node.id + 1000);

                    if self.collapse_action != 0 {
                        let mut state = egui::collapsing_header::CollapsingState::load_with_default_open(ctx, win_id.with("collapsing"), true);
                        state.set_open(self.collapse_action < 0);
                        state.store(ctx);
                    }

                    let response = egui::Window::new(format!("{} : {}", title, node.title))
                        .current_pos(screen_pos)
                        .movable(true)
                        .constrain(false)
                        .id(win_id)
                        .show(ctx, |ui| {
                            ui.label(&node.title);
                            egui::CollapsingHeader::new("Source Code")
                                .id_salt(node.id + 5000)
                                .show(ui, |ui| {
                                    egui::ScrollArea::vertical()
                                        .max_height(300.0)
                                        .show(ui, |ui| {
                                            ui.add(
                                                egui::TextEdit::multiline(&mut node.content)
                                                    .code_editor()
                                                    .desired_width(400.0),
                                            );
                                        });
                                });
                        });

                    if let Some(inner) = response {
                        let actual_pos = inner.response.rect.min;
                        let delta = actual_pos - screen_pos;
                        if delta.length() > 0.5 {
                            node.pos += delta / zoom;
                            node.vel = egui::Vec2::ZERO;
                        }
                    }
                }

                // --- SUN OFF-SCREEN INDICATOR ---
                if let Some(sun) = self.nodes.iter().find(|n| n.title == "main.py") {
                    let sun_screen = to_screen(sun.pos);
                    let margin = 40.0;
                    let is_offscreen = sun_screen.x < panel_rect.left()
                        || sun_screen.x > panel_rect.right()
                        || sun_screen.y < panel_rect.top()
                        || sun_screen.y > panel_rect.bottom();

                    if is_offscreen {
                        let center = panel_rect.center();
                        let dir = (sun_screen - center).normalized();

                        // Clamp the indicator to the panel edges with margin
                        let half_w = panel_rect.width() * 0.5 - margin;
                        let half_h = panel_rect.height() * 0.5 - margin;
                        let scale = if dir.x.abs() > 0.001 && dir.y.abs() > 0.001 {
                            (half_w / dir.x.abs()).min(half_h / dir.y.abs())
                        } else if dir.x.abs() > 0.001 {
                            half_w / dir.x.abs()
                        } else {
                            half_h / dir.y.abs()
                        };
                        let indicator_pos = center + dir * scale;

                        let painter = ui.painter();

                        // Draw arrow triangle pointing towards the sun
                        let arrow_size = 12.0;
                        let perp = egui::vec2(-dir.y, dir.x);
                        let tip = indicator_pos + dir * arrow_size;
                        let left = indicator_pos - dir * 4.0 + perp * arrow_size * 0.6;
                        let right = indicator_pos - dir * 4.0 - perp * arrow_size * 0.6;
                        painter.add(egui::epaint::PathShape::convex_polygon(
                            vec![tip, left, right],
                            egui::Color32::from_rgb(255, 200, 50),
                            egui::Stroke::NONE,
                        ));

                        // Label next to arrow
                        painter.text(
                            indicator_pos - dir * 18.0,
                            egui::Align2::CENTER_CENTER,
                            "Sun",
                            egui::FontId::proportional(12.0),
                            egui::Color32::from_rgb(255, 200, 50),
                        );
                    }
                }
            }
            // Reset one-shot collapse action after applying to all windows
            self.collapse_action = 0;

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
