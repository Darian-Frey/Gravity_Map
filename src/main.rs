use eframe::egui;

/// Represents a single node (idea or file) in the gravity map.
struct IdeaNode {
    id: usize,
    title: String,
    content: String,
    /// Relative path from project root with " > " separators
    file_path: String,
    /// Position used in Gravity mode (Mode 2)
    pos: egui::Pos2,
    vel: egui::Vec2,
    /// Independent position used in Blueprint mode (Mode 1)
    blueprint_pos: egui::Pos2,
    blueprint_vel: egui::Vec2,
    is_python: bool,
    /// Whether this node is a directory (sub-sun) rather than a file (planet)
    is_dir: bool,
    depth: usize,
    /// ID of the parent node this orbits around (None = root sun)
    parent_id: Option<usize>,
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

/// Returns the distance from `point` to the nearest point on the quadratic bezier.
fn dist_to_bezier(point: egui::Pos2, p0: egui::Pos2, p1: egui::Pos2, p2: egui::Pos2) -> f32 {
    let steps = 16;
    let mut min_dist = f32::MAX;
    for i in 0..steps {
        let t0 = i as f32 / steps as f32;
        let t1 = (i + 1) as f32 / steps as f32;
        let a = bezier_point(p0, p1, p2, t0);
        let b = bezier_point(p0, p1, p2, t1);
        min_dist = min_dist.min(point_to_segment_dist(point, a, b));
    }
    min_dist
}

fn bezier_point(p0: egui::Pos2, p1: egui::Pos2, p2: egui::Pos2, t: f32) -> egui::Pos2 {
    let inv = 1.0 - t;
    egui::pos2(
        inv * inv * p0.x + 2.0 * inv * t * p1.x + t * t * p2.x,
        inv * inv * p0.y + 2.0 * inv * t * p1.y + t * t * p2.y,
    )
}

fn point_to_segment_dist(p: egui::Pos2, a: egui::Pos2, b: egui::Pos2) -> f32 {
    let ab = b - a;
    let ap = p - a;
    let t = (ap.x * ab.x + ap.y * ab.y) / (ab.x * ab.x + ab.y * ab.y + 0.0001);
    let t = t.clamp(0.0, 1.0);
    let closest = egui::pos2(a.x + t * ab.x, a.y + t * ab.y);
    (p - closest).length()
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
        // --- MODE 2: GRAVITY PHYSICS (Hierarchical orbits) ---
        if self.mode == "Gravity" && !self.nodes.is_empty() {
            let attraction_strength = 0.1;
            let repulsion_strength = 200000.0;
            let dt = 0.1;
            let friction = 0.98;

            // Snapshot positions for stable force calculation
            let positions: Vec<(usize, egui::Pos2, Option<usize>, bool)> = self.nodes
                .iter()
                .map(|n| (n.id, n.pos, n.parent_id, n.is_dir))
                .collect();

            for i in 0..self.nodes.len() {
                // Root sun (no parent) stays put
                if self.nodes[i].parent_id.is_none() {
                    continue;
                }

                let mut force = egui::Vec2::ZERO;

                // 1. Attract to parent node
                if let Some(pid) = self.nodes[i].parent_id {
                    if let Some((_, parent_pos, _, _parent_is_dir)) = positions.iter().find(|(id, _, _, _)| *id == pid) {
                        let diff = self.nodes[i].pos - *parent_pos;
                        let dist = diff.length().max(1.0);
                        // Sub-suns orbit at tighter radius, files at wider
                        let target_orbit = if self.nodes[i].is_dir { 200.0 } else { 150.0 };
                        let orbit_error = dist - target_orbit;
                        force -= diff.normalized() * orbit_error * attraction_strength;
                    }
                }

                // 2. Repulsion from siblings (same parent)
                let my_parent = self.nodes[i].parent_id;
                for (j, (_, other_pos, other_parent, _)) in positions.iter().enumerate() {
                    if i == j { continue; }
                    // Stronger repulsion for siblings, weaker for unrelated
                    let is_sibling = *other_parent == my_parent;
                    let diff = self.nodes[i].pos - *other_pos;
                    let dist = diff.length().max(10.0);
                    let range = if is_sibling { 800.0 } else { 400.0 };
                    let strength = if is_sibling { repulsion_strength } else { repulsion_strength * 0.3 };
                    if dist < range {
                        force += diff.normalized() * (strength / (dist * dist));
                    }
                }

                let new_vel = (self.nodes[i].vel + force * dt) * friction;
                self.nodes[i].vel = new_vel;
                self.nodes[i].pos += new_vel * dt;
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
                            file_path: String::new(),
                            pos: spawn_pos,
                            vel: egui::Vec2::ZERO,
                            blueprint_pos: spawn_pos,
                            blueprint_vel: egui::Vec2::ZERO,
                            is_python: false,
                            is_dir: false,
                            depth: 0,
                            parent_id: None,
                        });
                    }
                    if ui.button("🐍 Add Python").clicked() {
                        let id = self.nodes.len();
                        self.nodes.push(IdeaNode {
                            id,
                            title: format!("module_{}.py", id),
                            content: String::new(),
                            file_path: String::new(),
                            pos: egui::pos2(200.0, 200.0),
                            vel: egui::Vec2::ZERO,
                            blueprint_pos: egui::pos2(200.0, 200.0),
                            blueprint_vel: egui::Vec2::ZERO,
                            is_python: true,
                            is_dir: false,
                            depth: 0,
                            parent_id: None,
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

                // --- INTERACTIVE TETHER LINES (parent -> child) ---
                {
                    let painter = ui.painter();
                    let screen_center = egui::pos2(panel_rect.center().x, panel_rect.center().y);
                    let mouse_pos = ctx.input(|i| i.pointer.hover_pos());
                    // Snapshot blueprint positions by id for lookup
                    let bp_positions: Vec<(usize, egui::Pos2)> = self.nodes.iter().map(|n| (n.id, n.blueprint_pos)).collect();
                    for node in &self.nodes {
                        if let Some(pid) = node.parent_id {
                            if let Some((_, parent_pos)) = bp_positions.iter().find(|(id, _)| *id == pid) {
                                let parent_screen = to_screen(*parent_pos);
                                let child_screen = to_screen(node.blueprint_pos);
                                let mid = parent_screen + (child_screen - parent_screen) * 0.5;
                                let cp = mid + (screen_center - mid) * 0.2;

                                let dist = mouse_pos
                                    .map(|mp| dist_to_bezier(mp, parent_screen, cp, child_screen))
                                    .unwrap_or(f32::MAX);
                                let hover_range = 60.0;
                                let is_near = dist < hover_range;

                                let (glow_color, core_color) = depth_colors(node.depth);
                                let (glow_w, core_w) = if dist < 8.0 {
                                    (8.0, 2.5)
                                } else {
                                    (4.0, 1.2)
                                };

                                painter.add(egui::epaint::QuadraticBezierShape {
                                    points: [parent_screen, cp, child_screen],
                                    closed: false,
                                    fill: egui::Color32::TRANSPARENT,
                                    stroke: egui::Stroke::new(glow_w, glow_color).into(),
                                });
                                painter.add(egui::epaint::QuadraticBezierShape {
                                    points: [parent_screen, cp, child_screen],
                                    closed: false,
                                    fill: egui::Color32::TRANSPARENT,
                                    stroke: egui::Stroke::new(core_w, core_color).into(),
                                });

                                if is_near {
                                    let fade = ((1.0 - dist / hover_range) * 255.0) as u8;
                                    let label_pos = bezier_point(parent_screen, cp, child_screen, 0.5);
                                    let label = if node.file_path.is_empty() {
                                        node.title.clone()
                                    } else {
                                        node.file_path.clone()
                                    };
                                    let text_galley = painter.layout_no_wrap(
                                        label.clone(),
                                        egui::FontId::proportional(11.0),
                                        egui::Color32::TRANSPARENT,
                                    );
                                    let pill_w = text_galley.size().x + 16.0;
                                    let pill_h = text_galley.size().y + 10.0;
                                    painter.rect_filled(
                                        egui::Rect::from_center_size(label_pos, egui::vec2(pill_w, pill_h)),
                                        pill_h / 2.0,
                                        egui::Color32::from_rgba_unmultiplied(15, 15, 25, (fade as f32 * 0.85) as u8),
                                    );
                                    painter.rect_stroke(
                                        egui::Rect::from_center_size(label_pos, egui::vec2(pill_w, pill_h)),
                                        pill_h / 2.0,
                                        egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(120, 180, 255, fade / 2)),
                                        egui::StrokeKind::Outside,
                                    );
                                    painter.text(
                                        label_pos,
                                        egui::Align2::CENTER_CENTER,
                                        &label,
                                        egui::FontId::proportional(11.0),
                                        egui::Color32::from_rgba_unmultiplied(200, 230, 255, fade),
                                    );
                                }
                            }
                        }
                    }
                }

                for node in &mut self.nodes {
                    let title = if node.parent_id.is_none() {
                        format!("🌞 {}", node.title)
                    } else if node.is_dir {
                        format!("⭐ {}", node.title)
                    } else if node.is_python {
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
                                root: &std::path::Path,
                                nodes: &mut Vec<IdeaNode>,
                                id_gen: &mut usize,
                                depth: usize,
                                parent_id: Option<usize>,
                            ) {
                                if let Ok(entries) = std::fs::read_dir(dir) {
                                    for entry in entries.flatten() {
                                        let path = entry.path();
                                        let file_name =
                                            path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                                        if path.is_dir() {
                                            if !["venv", ".git", "__pycache__", "node_modules", ".venv"]
                                                .contains(&file_name)
                                            {
                                                // Check if this dir contains any .py files (directly or nested)
                                                let has_py = std::fs::read_dir(&path)
                                                    .map(|e| e.flatten().any(|e| {
                                                        let p = e.path();
                                                        (p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("py"))
                                                            || p.is_dir()
                                                    }))
                                                    .unwrap_or(false);
                                                if has_py {
                                                    let dir_id = *id_gen;
                                                    let rel = path.strip_prefix(root)
                                                        .unwrap_or(&path)
                                                        .to_string_lossy()
                                                        .replace(std::path::MAIN_SEPARATOR, " > ");
                                                    nodes.push(IdeaNode {
                                                        id: dir_id,
                                                        title: format!("{}/", file_name),
                                                        content: format!("Directory: {}", rel),
                                                        file_path: rel,
                                                        pos: egui::pos2(640.0, 360.0),
                                                        vel: egui::Vec2::ZERO,
                                                        blueprint_pos: egui::pos2(640.0, 360.0),
                                                        blueprint_vel: egui::Vec2::ZERO,
                                                        is_python: false,
                                                        is_dir: true,
                                                        depth,
                                                        parent_id,
                                                    });
                                                    *id_gen += 1;
                                                    scan_recursive(&path, root, nodes, id_gen, depth + 1, Some(dir_id));
                                                }
                                            }
                                        } else if path.extension().and_then(|s| s.to_str())
                                            == Some("py")
                                        {
                                            let source = std::fs::read_to_string(&path)
                                                .unwrap_or_else(|_| "(could not read file)".to_owned());
                                            let rel = path.strip_prefix(root)
                                                .unwrap_or(&path)
                                                .to_string_lossy()
                                                .replace(std::path::MAIN_SEPARATOR, " > ");
                                            nodes.push(IdeaNode {
                                                id: *id_gen,
                                                title: file_name.to_owned(),
                                                content: source,
                                                file_path: rel,
                                                pos: egui::pos2(640.0, 360.0),
                                                vel: egui::Vec2::ZERO,
                                                blueprint_pos: egui::pos2(640.0, 360.0),
                                                blueprint_vel: egui::Vec2::ZERO,
                                                is_python: true,
                                                is_dir: false,
                                                depth,
                                                parent_id,
                                            });
                                            *id_gen += 1;
                                        }
                                    }
                                }
                            }
                            scan_recursive(&path, &path, &mut temp_nodes, &mut id_counter, 0, None);
                            if !temp_nodes.is_empty() {
                                // Find or pick the root sun (main.py or first file)
                                let sun_idx = temp_nodes
                                    .iter()
                                    .position(|n| n.title == "main.py")
                                    .unwrap_or(0);
                                let mut sun_node = temp_nodes.remove(sun_idx);
                                let sun_id = sun_node.id;
                                sun_node.title = "main.py".to_owned();
                                sun_node.pos = egui::pos2(640.0, 360.0);
                                sun_node.blueprint_pos = egui::pos2(640.0, 360.0);
                                sun_node.parent_id = None;

                                // Remap: nodes with parent_id == None orbit the sun
                                for node in &mut temp_nodes {
                                    if node.parent_id.is_none() {
                                        node.parent_id = Some(sun_id);
                                    }
                                }

                                // Build final list: sun first, then all others
                                self.nodes.push(sun_node);
                                self.nodes.extend(temp_nodes);

                                // Reassign IDs sequentially
                                // Build old_id -> new_id map
                                let id_map: std::collections::HashMap<usize, usize> = self.nodes
                                    .iter()
                                    .enumerate()
                                    .map(|(new_id, node)| (node.id, new_id))
                                    .collect();
                                for node in &mut self.nodes {
                                    let new_id = *id_map.get(&node.id).unwrap_or(&node.id);
                                    node.id = new_id;
                                    node.parent_id = node.parent_id.and_then(|pid| id_map.get(&pid).copied());
                                }

                                // --- RADIAL LAYOUT ---
                                // Children spread in a circle around their parent.
                                // Sub-suns get a larger radius so their subtrees have room.
                                fn count_descendants(parent_id: usize, nodes: &[IdeaNode]) -> usize {
                                    let children: Vec<usize> = nodes
                                        .iter()
                                        .filter(|n| n.parent_id == Some(parent_id))
                                        .map(|n| n.id)
                                        .collect();
                                    let mut total = children.len();
                                    for cid in &children {
                                        total += count_descendants(*cid, nodes);
                                    }
                                    total
                                }

                                fn radial_layout(
                                    parent_id: usize,
                                    nodes: &mut Vec<IdeaNode>,
                                    center: egui::Pos2,
                                    radius: f32,
                                    start_angle: f32,
                                    sweep: f32,
                                ) {
                                    let child_ids: Vec<usize> = nodes
                                        .iter()
                                        .filter(|n| n.parent_id == Some(parent_id))
                                        .map(|n| n.id)
                                        .collect();
                                    if child_ids.is_empty() { return; }

                                    // Weight each child by how many descendants it has (+ 1 for itself)
                                    let weights: Vec<f32> = child_ids.iter()
                                        .map(|cid| 1.0 + count_descendants(*cid, nodes) as f32)
                                        .collect();
                                    let total_weight: f32 = weights.iter().sum();

                                    let mut angle_cursor = start_angle;
                                    for (i, child_id) in child_ids.iter().enumerate() {
                                        let child_sweep = sweep * (weights[i] / total_weight);
                                        let angle = angle_cursor + child_sweep / 2.0;
                                        let is_dir = nodes.iter().find(|n| n.id == *child_id).map(|n| n.is_dir).unwrap_or(false);

                                        let pos = egui::pos2(
                                            center.x + angle.cos() * radius,
                                            center.y + angle.sin() * radius,
                                        );

                                        if let Some(child) = nodes.iter_mut().find(|n| n.id == *child_id) {
                                            child.pos = pos;
                                            child.blueprint_pos = pos;
                                        }

                                        // Recurse into sub-suns with a smaller radius and their slice of the arc
                                        if is_dir {
                                            let sub_radius = radius * 0.7;
                                            radial_layout(*child_id, nodes, pos, sub_radius.max(150.0), angle - child_sweep / 2.0, child_sweep);
                                        }

                                        angle_cursor += child_sweep;
                                    }
                                }

                                let sun_id = self.nodes[0].id;
                                let child_count = self.nodes.iter().filter(|n| n.parent_id == Some(sun_id)).count();
                                let base_radius = 400.0 + (child_count as f32 * 40.0).min(400.0);
                                radial_layout(sun_id, &mut self.nodes, egui::pos2(640.0, 360.0), base_radius, 0.0, std::f32::consts::TAU);
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

                // --- INTERACTIVE TETHER LINES (parent -> child) ---
                {
                    let painter = ui.painter();
                    let screen_center = egui::pos2(panel_rect.center().x, panel_rect.center().y);
                    let mouse_pos = ctx.input(|i| i.pointer.hover_pos());
                    let grav_positions: Vec<(usize, egui::Pos2)> = self.nodes.iter().map(|n| (n.id, n.pos)).collect();
                    for node in &self.nodes {
                        if let Some(pid) = node.parent_id {
                            if let Some((_, parent_pos)) = grav_positions.iter().find(|(id, _)| *id == pid) {
                                let parent_screen = to_screen(*parent_pos);
                                let child_screen = to_screen(node.pos);
                                let mid = parent_screen + (child_screen - parent_screen) * 0.5;
                                let cp = mid + (screen_center - mid) * 0.2;

                                let dist = mouse_pos
                                    .map(|mp| dist_to_bezier(mp, parent_screen, cp, child_screen))
                                    .unwrap_or(f32::MAX);
                                let hover_range = 60.0;
                                let is_near = dist < hover_range;

                                let (glow_color, core_color) = depth_colors(node.depth);
                                let (glow_w, core_w) = if dist < 8.0 {
                                    (8.0, 2.5)
                                } else {
                                    (4.0, 1.2)
                                };

                                painter.add(egui::epaint::QuadraticBezierShape {
                                    points: [parent_screen, cp, child_screen],
                                    closed: false,
                                    fill: egui::Color32::TRANSPARENT,
                                    stroke: egui::Stroke::new(glow_w, glow_color).into(),
                                });
                                painter.add(egui::epaint::QuadraticBezierShape {
                                    points: [parent_screen, cp, child_screen],
                                    closed: false,
                                    fill: egui::Color32::TRANSPARENT,
                                    stroke: egui::Stroke::new(core_w, core_color).into(),
                                });

                                if is_near {
                                    let fade = ((1.0 - dist / hover_range) * 255.0) as u8;
                                    let label_pos = bezier_point(parent_screen, cp, child_screen, 0.5);
                                    let label = if node.file_path.is_empty() {
                                        node.title.clone()
                                    } else {
                                        node.file_path.clone()
                                    };
                                    let text_galley = painter.layout_no_wrap(
                                        label.clone(),
                                        egui::FontId::proportional(11.0),
                                        egui::Color32::TRANSPARENT,
                                    );
                                    let pill_w = text_galley.size().x + 16.0;
                                    let pill_h = text_galley.size().y + 10.0;
                                    painter.rect_filled(
                                        egui::Rect::from_center_size(label_pos, egui::vec2(pill_w, pill_h)),
                                        pill_h / 2.0,
                                        egui::Color32::from_rgba_unmultiplied(15, 15, 25, (fade as f32 * 0.85) as u8),
                                    );
                                    painter.rect_stroke(
                                        egui::Rect::from_center_size(label_pos, egui::vec2(pill_w, pill_h)),
                                        pill_h / 2.0,
                                        egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(120, 180, 255, fade / 2)),
                                        egui::StrokeKind::Outside,
                                    );
                                    painter.text(
                                        label_pos,
                                        egui::Align2::CENTER_CENTER,
                                        &label,
                                        egui::FontId::proportional(11.0),
                                        egui::Color32::from_rgba_unmultiplied(200, 230, 255, fade),
                                    );
                                }
                            }
                        }
                    }
                }

                // --- RENDER WINDOWS ---
                for node in &mut self.nodes {
                    let is_root_sun = node.parent_id.is_none();
                    let title = if is_root_sun {
                        "🌞 Sun"
                    } else if node.is_dir {
                        "⭐ Sub-Sun"
                    } else {
                        "🪐 Planet"
                    };
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
