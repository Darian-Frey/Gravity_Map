use eframe::egui;
use crate::app::GravityApp;
use crate::nodes::IdeaNode;
use crate::canvas;
use crate::utils::{dist_to_bezier, bezier_point, depth_colors};

pub(crate) fn render_gravity(app: &mut GravityApp, ctx: &egui::Context, ui: &mut egui::Ui, panel_rect: egui::Rect) {
    canvas::handle_pan_zoom(ctx, ui, panel_rect, &mut app.cam_offset, &mut app.zoom, "canvas_pan");

    let zoom = app.zoom;
    let cam = app.cam_offset;
    canvas::draw_grid(ui.painter(), panel_rect, cam, zoom);

    let to_screen = |world_pos: egui::Pos2| canvas::world_to_screen(world_pos, cam, zoom);

    // --- TOOLBAR ---
    ui.horizontal(|ui| {
        if ui.button("📁 Scan Project").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                app.nodes.clear();
                let mut temp_nodes = Vec::new();
                let mut id_counter = 0;

                scan_recursive(&path, &path, &mut temp_nodes, &mut id_counter, 0, None);

                if !temp_nodes.is_empty() {
                    let sun_idx = temp_nodes.iter().position(|n| n.title == "main.py").unwrap_or(0);
                    let mut sun_node = temp_nodes.remove(sun_idx);
                    let sun_id = sun_node.id;
                    sun_node.title = "main.py".to_owned();
                    sun_node.pos = egui::pos2(640.0, 360.0);
                    sun_node.parent_id = None;

                    for node in &mut temp_nodes {
                        if node.parent_id.is_none() {
                            node.parent_id = Some(sun_id);
                        }
                    }

                    app.nodes.push(sun_node);
                    app.nodes.extend(temp_nodes);

                    // Reassign IDs
                    let id_map: std::collections::HashMap<usize, usize> = app.nodes
                        .iter().enumerate().map(|(new_id, node)| (node.id, new_id)).collect();
                    for node in &mut app.nodes {
                        let new_id = *id_map.get(&node.id).unwrap_or(&node.id);
                        node.id = new_id;
                        node.parent_id = node.parent_id.and_then(|pid| id_map.get(&pid).copied());
                    }

                    let sun_id = app.nodes[0].id;
                    let child_count = app.nodes.iter().filter(|n| n.parent_id == Some(sun_id)).count();
                    let base_radius = 400.0 + (child_count as f32 * 40.0).min(400.0);
                    radial_layout(sun_id, &mut app.nodes, egui::pos2(640.0, 360.0), base_radius, 0.0, std::f32::consts::TAU);
                }
            }
        }

        if ui.button("🗑 Clear").clicked() { app.nodes.clear(); }

        ui.add_space(8.0);
        if ui.button("🌞 Return to Sun").clicked() {
            if let Some(sun) = app.nodes.iter().find(|n| n.title == "main.py") {
                let viewport_center = egui::vec2(panel_rect.center().x, panel_rect.center().y);
                app.cam_offset = viewport_center - sun.pos.to_vec2() * app.zoom;
            }
        }

        ui.add_space(8.0);
        if ui.button("▼ Collapse All").clicked() { app.collapse_action = 1; }
        if ui.button("▶ Expand All").clicked() { app.collapse_action = -1; }
    });

    ui.separator();

    // --- INTERACTIVE TETHER LINES ---
    {
        let painter = ui.painter();
        let screen_center = egui::pos2(panel_rect.center().x, panel_rect.center().y);
        let mouse_pos = ctx.input(|i| i.pointer.hover_pos());
        let positions: Vec<(usize, egui::Pos2)> = app.nodes.iter().map(|n| (n.id, n.pos)).collect();
        for node in &app.nodes {
            if let Some(pid) = node.parent_id {
                if let Some((_, parent_pos)) = positions.iter().find(|(id, _)| *id == pid) {
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
                    let (glow_w, core_w) = if dist < 8.0 { (8.0, 2.5) } else { (4.0, 1.2) };

                    painter.add(egui::epaint::QuadraticBezierShape {
                        points: [parent_screen, cp, child_screen],
                        closed: false, fill: egui::Color32::TRANSPARENT,
                        stroke: egui::Stroke::new(glow_w, glow_color).into(),
                    });
                    painter.add(egui::epaint::QuadraticBezierShape {
                        points: [parent_screen, cp, child_screen],
                        closed: false, fill: egui::Color32::TRANSPARENT,
                        stroke: egui::Stroke::new(core_w, core_color).into(),
                    });

                    if is_near {
                        let fade = ((1.0 - dist / hover_range) * 255.0) as u8;
                        let label_pos = bezier_point(parent_screen, cp, child_screen, 0.5);
                        let label = if node.file_path.is_empty() { node.title.clone() } else { node.file_path.clone() };
                        let text_galley = painter.layout_no_wrap(
                            label.clone(), egui::FontId::proportional(11.0), egui::Color32::TRANSPARENT,
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
                            label_pos, egui::Align2::CENTER_CENTER, &label,
                            egui::FontId::proportional(11.0),
                            egui::Color32::from_rgba_unmultiplied(200, 230, 255, fade),
                        );
                    }
                }
            }
        }
    }

    // --- RENDER WINDOWS ---
    for node in &mut app.nodes {
        let is_root_sun = node.parent_id.is_none();
        let title = if is_root_sun {
            format!("🌞 Sun : {}", node.title)
        } else if node.is_dir {
            format!("⭐ Sub-Sun : {}", node.title)
        } else {
            format!("🪐 Planet : {}", node.title)
        };
        let screen_pos = to_screen(node.pos);
        let win_id = egui::Id::new(node.id + 1000);

        if app.collapse_action != 0 {
            let mut state = egui::collapsing_header::CollapsingState::load_with_default_open(
                ctx, win_id.with("collapsing"), true,
            );
            state.set_open(app.collapse_action < 0);
            state.store(ctx);
        }

        let response = egui::Window::new(&title)
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
            }
        }
    }

    // --- SUN OFF-SCREEN INDICATOR ---
    if let Some(sun) = app.nodes.iter().find(|n| n.title == "main.py") {
        let sun_screen = to_screen(sun.pos);
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
                indicator_pos - dir * 18.0, egui::Align2::CENTER_CENTER,
                "Sun", egui::FontId::proportional(12.0),
                egui::Color32::from_rgb(255, 200, 50),
            );
        }
    }
}

// --- Helper functions ---

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
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if path.is_dir() {
                if !["venv", ".git", "__pycache__", "node_modules", ".venv"].contains(&file_name) {
                    let has_py = std::fs::read_dir(&path)
                        .map(|e| e.flatten().any(|e| {
                            let p = e.path();
                            (p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("py")) || p.is_dir()
                        }))
                        .unwrap_or(false);
                    if has_py {
                        let dir_id = *id_gen;
                        let rel = path.strip_prefix(root).unwrap_or(&path)
                            .to_string_lossy().replace(std::path::MAIN_SEPARATOR, " > ");
                        nodes.push(IdeaNode {
                            id: dir_id,
                            title: format!("{}/", file_name),
                            content: format!("Directory: {}", rel),
                            file_path: rel,
                            pos: egui::pos2(640.0, 360.0),
                            is_dir: true, depth, parent_id,
                        });
                        *id_gen += 1;
                        scan_recursive(&path, root, nodes, id_gen, depth + 1, Some(dir_id));
                    }
                }
            } else if path.extension().and_then(|s| s.to_str()) == Some("py") {
                let source = std::fs::read_to_string(&path)
                    .unwrap_or_else(|_| "(could not read file)".to_owned());
                let rel = path.strip_prefix(root).unwrap_or(&path)
                    .to_string_lossy().replace(std::path::MAIN_SEPARATOR, " > ");
                nodes.push(IdeaNode {
                    id: *id_gen,
                    title: file_name.to_owned(),
                    content: source,
                    file_path: rel,
                    pos: egui::pos2(640.0, 360.0),
                    is_dir: false, depth, parent_id,
                });
                *id_gen += 1;
            }
        }
    }
}

fn count_descendants(parent_id: usize, nodes: &[IdeaNode]) -> usize {
    let children: Vec<usize> = nodes.iter()
        .filter(|n| n.parent_id == Some(parent_id)).map(|n| n.id).collect();
    let mut total = children.len();
    for cid in &children { total += count_descendants(*cid, nodes); }
    total
}

fn radial_layout(
    parent_id: usize, nodes: &mut Vec<IdeaNode>,
    center: egui::Pos2, radius: f32, start_angle: f32, sweep: f32,
) {
    let child_ids: Vec<usize> = nodes.iter()
        .filter(|n| n.parent_id == Some(parent_id)).map(|n| n.id).collect();
    if child_ids.is_empty() { return; }

    let weights: Vec<f32> = child_ids.iter()
        .map(|cid| 1.0 + count_descendants(*cid, nodes) as f32).collect();
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
        }
        if is_dir {
            radial_layout(*child_id, nodes, pos, (radius * 0.7).max(150.0),
                angle - child_sweep / 2.0, child_sweep);
        }
        angle_cursor += child_sweep;
    }
}
